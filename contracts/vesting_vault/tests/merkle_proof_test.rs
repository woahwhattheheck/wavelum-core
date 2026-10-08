#![cfg(test)]

use soroban_sdk::{Address, Bytes, BytesN, Env, Vec};
use soroban_sdk::testutils::Address as _;
use vesting_vault::errors::Error;
use vesting_vault::{Nullifier, VestingVault, VestingVaultClient, ZKClaimProof};

/// sha256(left || right) — same pairing the contract's verify_merkle_proof
/// applies per level.
fn hash_pair(env: &Env, left: &BytesN<32>, right: &BytesN<32>) -> BytesN<32> {
    let mut buf = Bytes::new(env);
    buf.append(&Bytes::from_array(env, &left.to_array()));
    buf.append(&Bytes::from_array(env, &right.to_array()));
    env.crypto().sha256(&buf).to_bytes()
}

fn zk_proof(
    env: &Env,
    commitment: &BytesN<32>,
    nullifier_hash: &BytesN<32>,
    root: &BytesN<32>,
    proof: Vec<BytesN<32>>,
    position: u32,
) -> ZKClaimProof {
    ZKClaimProof {
        commitment_hash: commitment.clone(),
        nullifier_hash: nullifier_hash.clone(),
        merkle_root: root.clone(),
        merkle_proof: proof,
        position,
        proof_data: Bytes::new(env),
    }
}

#[test]
fn test_private_claim_two_leaf_merkle_proof() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Two-leaf tree: leaf0 = commitment, leaf1 = sibling, root = H(l0 || l1).
    let leaf0 = BytesN::from_array(&env, &[42u8; 32]);
    let leaf1 = BytesN::from_array(&env, &[43u8; 32]);
    let root = hash_pair(&env, &leaf0, &leaf1);
    let nullifier = Nullifier { hash: BytesN::from_array(&env, &[44u8; 32]) };

    client.create_commitment(&user, &1u32, &500i128, &leaf0);
    client.add_merkle_root_admin(&admin, &root);

    let mut proof_vec = Vec::new(&env);
    proof_vec.push_back(leaf1);
    let proof = zk_proof(&env, &leaf0, &nullifier.hash, &root, proof_vec, 0);

    client.private_claim(&proof, &nullifier, &500i128);

    assert!(client.is_nullifier_used_public(&nullifier));
    assert!(client.get_commitment_info(&leaf0).unwrap().is_used);
}

#[test]
fn test_private_claim_rejects_tampered_proof() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let leaf0 = BytesN::from_array(&env, &[50u8; 32]);
    let leaf1 = BytesN::from_array(&env, &[51u8; 32]);
    let root = hash_pair(&env, &leaf0, &leaf1);
    let nullifier = Nullifier { hash: BytesN::from_array(&env, &[52u8; 32]) };

    client.create_commitment(&user, &1u32, &500i128, &leaf0);
    client.add_merkle_root_admin(&admin, &root);

    // Tampered sibling — recomputed root will not match.
    let mut bad = Vec::new(&env);
    bad.push_back(BytesN::from_array(&env, &[99u8; 32]));
    let proof = zk_proof(&env, &leaf0, &nullifier.hash, &root, bad, 0);

    let res = client.try_private_claim(&proof, &nullifier, &500i128);
    assert_eq!(res, Err(Ok(Error::InvalidZKProof)));
    assert!(!client.is_nullifier_used_public(&nullifier));
}

#[test]
fn test_private_claim_rejects_wrong_position() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let leaf0 = BytesN::from_array(&env, &[60u8; 32]);
    let leaf1 = BytesN::from_array(&env, &[61u8; 32]);
    let root = hash_pair(&env, &leaf0, &leaf1);
    let nullifier = Nullifier { hash: BytesN::from_array(&env, &[62u8; 32]) };

    client.create_commitment(&user, &1u32, &500i128, &leaf0);
    client.add_merkle_root_admin(&admin, &root);

    // Same sibling but position flipped — H(sibling || leaf) != root.
    let mut proof_vec = Vec::new(&env);
    proof_vec.push_back(leaf1);
    let proof = zk_proof(&env, &leaf0, &nullifier.hash, &root, proof_vec, 1);

    let res = client.try_private_claim(&proof, &nullifier, &500i128);
    assert_eq!(res, Err(Ok(Error::InvalidZKProof)));
}

#[test]
fn test_private_claim_rejects_position_above_proof_depth() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let leaf0 = BytesN::from_array(&env, &[63u8; 32]);
    let leaf1 = BytesN::from_array(&env, &[64u8; 32]);
    let root = hash_pair(&env, &leaf0, &leaf1);
    let nullifier = Nullifier { hash: BytesN::from_array(&env, &[65u8; 32]) };

    client.create_commitment(&user, &1u32, &500i128, &leaf0);
    client.add_merkle_root_admin(&admin, &root);

    // With one sibling only bit 0 is meaningful. Position 2 has the same
    // low bit as position 0, but names a leaf outside this proof depth.
    let mut proof_vec = Vec::new(&env);
    proof_vec.push_back(leaf1);
    let proof = zk_proof(&env, &leaf0, &nullifier.hash, &root, proof_vec, 2);

    let res = client.try_private_claim(&proof, &nullifier, &500i128);
    assert_eq!(res, Err(Ok(Error::InvalidZKProof)));
    assert!(!client.is_nullifier_used_public(&nullifier));
}

#[test]
fn test_private_claim_rejects_unregistered_root() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    // A locally-consistent proof whose root was never submitted is rejected.
    let leaf = BytesN::from_array(&env, &[70u8; 32]);
    let nullifier = Nullifier { hash: BytesN::from_array(&env, &[71u8; 32]) };
    client.create_commitment(&user, &1u32, &500i128, &leaf);

    let proof = zk_proof(&env, &leaf, &nullifier.hash, &leaf, Vec::new(&env), 0);
    let res = client.try_private_claim(&proof, &nullifier, &500i128);
    assert_eq!(res, Err(Ok(Error::InvalidInput)));
}

#[test]
fn test_submit_merkle_root() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    let root = BytesN::from_array(&env, &[80u8; 32]);
    client.submit_merkle_root(&admin, &root);

    assert!(client.get_merkle_roots().contains(root));
}
