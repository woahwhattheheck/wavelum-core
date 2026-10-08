#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};
use vesting_vault::{errors::Error, VestingVault, VestingVaultClient};

fn setup() -> (Env, VestingVaultClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(VestingVault, ());
    let client = VestingVaultClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn upgrade_admin_is_set_once_and_enforced_before_wasm_update() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let other = Address::generate(&env);

    client.initialize_upgrade_admin(&admin);
    assert_eq!(client.get_upgrade_admin(), Some(admin.clone()));

    let second_init = client.try_initialize_upgrade_admin(&other);
    assert_eq!(second_init, Err(Ok(Error::AlreadyInitialized)));

    let fake_hash = BytesN::from_array(&env, &[7u8; 32]);
    let unauthorized = client.try_upgrade_logic_contract(&other, &fake_hash);
    assert_eq!(unauthorized, Err(Ok(Error::Unauthorized)));
}

#[test]
fn upgrade_preserves_unvested_funds_safety_gate() {
    let (env, client) = setup();
    let admin = Address::generate(&env);

    client.initialize_upgrade_admin(&admin);
    client.update_total_unvested_balance(&admin, &1);

    let fake_hash = BytesN::from_array(&env, &[9u8; 32]);
    let blocked = client.try_upgrade_logic_contract(&admin, &fake_hash);
    assert_eq!(blocked, Err(Ok(Error::UpgradeBlockedByUnvestedFunds)));
}
