#![cfg(test)]

use soroban_sdk::{Address, Bytes, BytesN, Env, Error, IntoVal, Symbol, Val};
use soroban_sdk::testutils::Address as _;
use vesting_vault::{VestingVault, VestingVaultClient};

/// Locate a locally-built vesting_vault WASM for the real-upgrade path.
/// When absent (e.g. no wasm target built), the upgrade test only covers
/// the gating behavior.
fn load_wasm() -> Option<std::vec::Vec<u8>> {
    for p in [
        "target/wasm32v1-none/release/vesting_vault.wasm",
        "target/wasm32-unknown-none/release/vesting_vault.wasm",
        "../target/wasm32v1-none/release/vesting_vault.wasm",
        "../target/wasm32-unknown-none/release/vesting_vault.wasm",
    ] {
        if let Ok(b) = std::fs::read(p) {
            return Some(b);
        }
    }
    None
}

#[test]
fn test_version_reports_current_build() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(VestingVault, ());
    let client = VestingVaultClient::new(&env, &contract_id);
    assert_eq!(client.version(), 1);
}

#[test]
fn test_upgrade_blocked_while_unvested_funds_remain() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(VestingVault, ());
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.update_total_unvested_balance(&admin, &10i128);

    let bogus = BytesN::from_array(&env, &[7u8; 32]);
    let result = env.try_invoke_contract::<Val, Error>(
        &contract_id,
        &Symbol::new(&env, "upgrade"),
        (admin.clone(), bogus).into_val(&env),
    );
    assert!(result.is_err());

    // unwind the guard for cleanliness
    client.update_total_unvested_balance(&admin, &0i128);
}

#[test]
fn test_upgrade_rejects_unknown_wasm_hash() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(VestingVault, ());
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    // Guard passes (zero unvested) but the hash was never uploaded.
    let bogus = BytesN::from_array(&env, &[9u8; 32]);
    let result = env.try_invoke_contract::<Val, Error>(
        &contract_id,
        &Symbol::new(&env, "upgrade"),
        (admin.clone(), bogus).into_val(&env),
    );
    assert!(result.is_err());
}

#[test]
fn test_upgrade_preserves_state() {
    let Some(wasm) = load_wasm() else {
        // No wasm build available in this environment; gating tests above
        // still cover the entry point.
        return;
    };
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(VestingVault, ());
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    // Record state that must survive the swap.
    let merkle_root = BytesN::from_array(&env, &[42u8; 32]);
    client.add_merkle_root_admin(&admin, &merkle_root);

    // Upload the WASM and upgrade (same build -> observable state must be
    // preserved and the contract must remain callable afterwards).
    let wasm_hash = env
        .deployer()
        .upload_contract_wasm(Bytes::from_slice(&env, &wasm));
    client.upgrade(&admin, &wasm_hash);

    assert!(client.get_merkle_roots().contains(merkle_root));
    assert_eq!(client.version(), 1);
}
