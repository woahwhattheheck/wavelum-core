#![cfg(test)]

use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};
use soroban_sdk::{Address, Env};
use vesting_vault::errors::Error;
use vesting_vault::{VestingVault, VestingVaultClient};

/// Reentrancy guard storage key (mirrors `storage::REENTRANCY_GUARD`; the
/// module is crate-private so the literal is used here).
const GUARD_KEY: &str = "REENTRANCY_GUARD";

/// 30 days in seconds — mirrors `storage::EMERGENCY_WITHDRAW_WINDOW_SECS`.
const WINDOW_SECS: u64 = 2_592_000;

/// Hold the vault's reentrancy lock, simulating an in-flight guarded call.
fn hold_guard(env: &Env, vault: &Address) {
    env.as_contract(vault, || {
        env.storage().instance().set(&GUARD_KEY, &true);
    });
}

/// Deploy the vault, register a stellar-asset token, and mint `supply` base
/// units directly into the vault — the orphaned balance an emergency
/// withdrawal recovers.
fn setup() -> (Env, Address, VestingVaultClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin).address();
    let vault_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &vault_id);
    let admin = Address::generate(&env);

    (env, vault_id, client, admin, token_id)
}

fn fund_vault(env: &Env, token_id: &Address, vault_id: &Address, amount: i128) {
    StellarAssetClient::new(env, token_id).mint(vault_id, &amount);
}

#[test]
fn test_emergency_withdraw_recovers_orphaned_tokens() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &500i128);
    fund_vault(&env, &token_id, &vault_id, 1000);

    client.emergency_withdraw(&admin, &token_id, &to, &300i128);

    let token = TokenClient::new(&env, &token_id);
    assert_eq!(token.balance(&to), 300);
    assert_eq!(token.balance(&vault_id), 700);

    let window = client.get_emergency_withdraw_window_state().unwrap();
    assert_eq!(window.withdrawn_in_window, 300);
}

#[test]
fn test_withdraw_rejects_unregistered_caller() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);
    let stranger = Address::generate(&env);
    fund_vault(&env, &token_id, &vault_id, 1000);

    // No admin registered at all.
    let res = client.try_emergency_withdraw(&stranger, &token_id, &to, &10i128);
    assert_eq!(res, Err(Ok(Error::NotInitialized)));

    // Admin registered, but the caller is someone else.
    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &500i128);
    let res = client.try_emergency_withdraw(&stranger, &token_id, &to, &10i128);
    assert_eq!(res, Err(Ok(Error::AdminNotRegistered)));
}

#[test]
fn test_withdraw_rejects_allocated_tokens() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &1000i128);
    fund_vault(&env, &token_id, &vault_id, 1000);
    client.record_total_allocated(&admin, &800i128);

    // 300 would leave only 700 against 800 of obligations.
    let res = client.try_emergency_withdraw(&admin, &token_id, &to, &300i128);
    assert_eq!(res, Err(Ok(Error::InsufficientUnallocatedBalance)));

    // Exactly the unallocated surplus is fine.
    client.emergency_withdraw(&admin, &token_id, &to, &200i128);
    assert_eq!(TokenClient::new(&env, &token_id).balance(&vault_id), 800);
}

#[test]
fn test_withdraw_rolling_limit_enforced_and_resets() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &100i128);
    fund_vault(&env, &token_id, &vault_id, 1000);

    client.emergency_withdraw(&admin, &token_id, &to, &60i128);

    // 60 + 50 exceeds the 100-per-30-days limit.
    let res = client.try_emergency_withdraw(&admin, &token_id, &to, &50i128);
    assert_eq!(res, Err(Ok(Error::WithdrawalLimitExceeded)));

    // After the window rolls, the full limit is available again.
    env.ledger()
        .set_timestamp(env.ledger().timestamp() + WINDOW_SECS + 1);
    client.emergency_withdraw(&admin, &token_id, &to, &50i128);
}

#[test]
fn test_withdraw_limit_is_fail_closed() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    fund_vault(&env, &token_id, &vault_id, 1000);

    // No limit configured -> withdrawals disabled by default.
    let res = client.try_emergency_withdraw(&admin, &token_id, &to, &1i128);
    assert_eq!(res, Err(Ok(Error::WithdrawalLimitExceeded)));
}

#[test]
fn test_withdraw_rejects_self_target() {
    let (env, vault_id, client, admin, token_id) = setup();

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &500i128);
    fund_vault(&env, &token_id, &vault_id, 1000);

    let res = client.try_emergency_withdraw(&admin, &token_id, &vault_id, &10i128);
    assert_eq!(res, Err(Ok(Error::InvalidWithdrawalTarget)));

    let res = client.try_emergency_withdraw(&admin, &vault_id, &vault_id, &10i128);
    assert_eq!(res, Err(Ok(Error::InvalidWithdrawalTarget)));
}

#[test]
fn test_withdraw_rejects_reentry_while_guard_held() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &500i128);
    fund_vault(&env, &token_id, &vault_id, 1000);

    hold_guard(&env, &vault_id);

    let res = client.try_emergency_withdraw(&admin, &token_id, &to, &10i128);
    assert_eq!(res, Err(Ok(Error::ReentrancyDetected)));
}

#[test]
fn test_withdraw_releases_guard() {
    let (env, vault_id, client, admin, token_id) = setup();
    let to = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &500i128);
    fund_vault(&env, &token_id, &vault_id, 1000);

    client.emergency_withdraw(&admin, &token_id, &to, &10i128);

    // The token callback ran while the lock was held; a normal claim
    // afterwards proves the lock was released.
    let res = client.try_claim(&user, &1u32, &100i128);
    assert!(res.is_ok());
}

#[test]
fn test_rotate_emergency_admin() {
    let (env, vault_id, client, admin, token_id) = setup();
    let successor = Address::generate(&env);
    let to = Address::generate(&env);

    client.initialize_emergency_admin(&admin);
    client.set_emergency_withdrawal_limit(&admin, &500i128);
    client.rotate_emergency_admin(&admin, &successor);

    // Old admin is no longer registered.
    let res = client.try_emergency_withdraw(&admin, &token_id, &to, &10i128);
    assert_eq!(res, Err(Ok(Error::AdminNotRegistered)));

    fund_vault(&env, &token_id, &vault_id, 1000);
    client.emergency_withdraw(&successor, &token_id, &to, &10i128);
    assert_eq!(TokenClient::new(&env, &token_id).balance(&to), 10);
}

#[test]
fn test_config_functions_require_registered_admin() {
    let (env, _vault_id, client, admin, _token_id) = setup();
    let stranger = Address::generate(&env);

    client.initialize_emergency_admin(&admin);

    let res = client.try_set_emergency_withdrawal_limit(&stranger, &100i128);
    assert_eq!(res, Err(Ok(Error::AdminNotRegistered)));

    let res = client.try_record_total_allocated(&stranger, &100i128);
    assert_eq!(res, Err(Ok(Error::AdminNotRegistered)));
}
