#![cfg(test)]

use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env};
use soroban_sdk::testutils::Address as _;
use vesting_vault::errors::Error;
use vesting_vault::{Nullifier, VestingVault, VestingVaultClient, ZKClaimProof};

/// Reentrancy guard storage key (mirrors `storage::REENTRANCY_GUARD`; the
/// module is crate-private so the literal is used here).
const GUARD_KEY: &str = "REENTRANCY_GUARD";

/// Malicious callee: while the vault's reentrancy lock is held (i.e. the vault
/// is mid-claim), calls back into `claim` — the path a hostile token or oracle
/// contract would take if it regained control during a claim.
#[contract]
pub struct ReentrantAttacker;

#[contractimpl]
impl ReentrantAttacker {
    pub fn attack(env: Env, vault: Address, user: Address, vesting_id: u32, amount: i128) -> Result<(), Error> {
        let client = VestingVaultClient::new(&env, &vault);
        match client.try_claim(&user, &vesting_id, &amount) {
            Err(Ok(e)) => Err(e),
            Err(Err(_)) => Err(Error::TransferFailed),
            Ok(_) => Ok(()),
        }
    }
}

/// Hold the vault's reentrancy lock, simulating an in-flight guarded call.
fn hold_guard(env: &Env, vault: &Address) {
    env.as_contract(vault, || {
        env.storage().instance().set(&GUARD_KEY, &true);
    });
}

#[test]
fn test_claim_rejects_reentry_while_guard_held() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    hold_guard(&env, &contract_id);

    let res = client.try_claim(&user, &1u32, &100i128);
    assert_eq!(res, Err(Ok(Error::ReentrancyDetected)));
}

#[test]
fn test_private_claim_rejects_reentry_while_guard_held() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);

    let commitment_hash = BytesN::from_array(&env, &[1u8; 32]);
    let nullifier_hash = BytesN::from_array(&env, &[2u8; 32]);
    let merkle_root = BytesN::from_array(&env, &[3u8; 32]);
    let proof = ZKClaimProof {
        commitment_hash,
        nullifier_hash: nullifier_hash.clone(),
        merkle_root,
        proof_data: Bytes::new(&env),
    };
    let nullifier = Nullifier { hash: nullifier_hash };

    hold_guard(&env, &contract_id);

    // The guard is checked before nullifier/commitment state is touched.
    let res = client.try_private_claim(&proof, &nullifier, &100i128);
    assert_eq!(res, Err(Ok(Error::ReentrancyDetected)));
}

#[test]
fn test_cross_contract_reentry_blocked() {
    let env = Env::default();
    env.mock_all_auths();
    let vault_id = env.register_contract(None, VestingVault);
    let attacker_id = env.register_contract(None, ReentrantAttacker);
    let attacker = ReentrantAttackerClient::new(&env, &attacker_id);
    let user = Address::generate(&env);

    // Vault is mid-claim: the lock is held when the attacker calls back in.
    hold_guard(&env, &vault_id);

    let res = attacker.try_attack(&vault_id, &user, &1u32, &100i128);
    assert_eq!(res, Err(Ok(Error::ReentrancyDetected)));
}

#[test]
fn test_guard_released_after_successful_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    client.claim(&user, &1u32, &100i128);

    // The lock must have been dropped on return — a second claim reaches the
    // normal path and succeeds instead of failing with ReentrancyDetected.
    let res = client.try_claim(&user, &1u32, &100i128);
    assert_ne!(res, Err(Ok(Error::ReentrancyDetected)));
    assert!(res.is_ok());
}

#[test]
fn test_guard_released_after_failed_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    // Validation failure while the guard is held must still release the lock.
    let res = client.try_claim(&user, &1u32, &0i128);
    assert_eq!(res, Err(Ok(Error::AmountMustBePositive)));

    client.claim(&user, &1u32, &100i128);
}

#[test]
fn test_private_claim_releases_guard() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let commitment_hash = BytesN::from_array(&env, &[10u8; 32]);
    let nullifier_hash = BytesN::from_array(&env, &[11u8; 32]);
    let merkle_root = BytesN::from_array(&env, &[12u8; 32]);

    client.create_commitment(&user, &1u32, &100i128, &commitment_hash);
    client.add_merkle_root_admin(&admin, &merkle_root);

    let proof = ZKClaimProof {
        commitment_hash,
        nullifier_hash: nullifier_hash.clone(),
        merkle_root,
        proof_data: Bytes::new(&env),
    };
    client.private_claim(&proof, &Nullifier { hash: nullifier_hash }, &100i128);

    // Guard was held for the whole private_claim; a normal claim afterwards
    // proves it was released.
    client.claim(&user, &1u32, &100i128);
}
