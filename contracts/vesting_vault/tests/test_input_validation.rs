#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, BytesN, Vec};
use vesting_vault::{VestingVault, VestingVaultClient};

// =============================================================================
// Issue #13: Input Validation & Bounds Checking -- vesting_vault
// =============================================================================

fn setup() -> (Env, Address, VestingVaultClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_addr = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_addr);
    let admin = Address::generate(&env);
    (env, admin, client)
}

/// Assert that a try_ call was rejected by input validation.
/// Works regardless of whether the error is on the outer or inner Result.
macro_rules! assert_rejected {
    ($expr:expr) => {
        let result = $expr;
        match result {
            Err(_) => { /* outer layer error -- contract error */ }
            Ok(inner) => {
                assert!(inner.is_err(), "Expected validation to reject the input, but call succeeded");
            }
        }
    };
}

// ---------------------------------------------------------------------------
// claim() -- vesting_id == 0, amount <= 0
// ---------------------------------------------------------------------------
#[test]
fn test_claim_rejects_zero_vesting_id() {
    let (env, user, client) = setup();
    assert_rejected!(client.try_claim(&user, &0u32, &100i128));
}

#[test]
fn test_claim_rejects_zero_amount() {
    let (_env, user, client) = setup();
    assert_rejected!(client.try_claim(&user, &1u32, &0i128));
}

#[test]
fn test_claim_rejects_negative_amount() {
    let (_env, user, client) = setup();
    assert_rejected!(client.try_claim(&user, &1u32, &(-100i128)));
}

// ---------------------------------------------------------------------------
// initialize_auditors() -- duplicate checks
// ---------------------------------------------------------------------------
#[test]
fn test_initialize_auditors_rejects_duplicate() {
    let (env, admin, client) = setup();
    let a1 = Address::generate(&env);
    let auditors = Vec::from_array(&env, [a1.clone(), a1.clone(), Address::generate(&env)]);
    assert_rejected!(client.try_initialize_auditors(&admin, &auditors));
}

#[test]
fn test_initialize_auditors_rejects_wrong_count() {
    let (env, admin, client) = setup();
    let a1 = Address::generate(&env);
    let a2 = Address::generate(&env);
    let auditors = Vec::from_array(&env, [a1, a2]);
    assert_rejected!(client.try_initialize_auditors(&admin, &auditors));
}

// ---------------------------------------------------------------------------
// configure_milestone_vesting() -- vesting_id == 0
// ---------------------------------------------------------------------------
#[test]
fn test_configure_milestone_rejects_zero_vesting_id() {
    let (env, admin, client) = setup();
    let pcts = Vec::from_array(&env, [50u32, 30, 20]);
    assert_rejected!(client.try_configure_milestone_vesting(&admin, &0u32, &pcts));
}

#[test]
fn test_configure_milestone_rejects_bad_percentages() {
    let (env, admin, client) = setup();
    let pcts = Vec::from_array(&env, [50u32, 30]); // sum != 100
    assert_rejected!(client.try_configure_milestone_vesting(&admin, &1u32, &pcts));
}

// ---------------------------------------------------------------------------
// complete_milestone() -- vesting_id == 0, milestone_number == 0
// ---------------------------------------------------------------------------
#[test]
fn test_complete_milestone_rejects_zero_vesting_id() {
    let (env, admin, client) = setup();
    assert_rejected!(client.try_complete_milestone(&admin, &0u32, &1u32));
}

#[test]
fn test_complete_milestone_rejects_zero_milestone_number() {
    let (env, admin, client) = setup();
    assert_rejected!(client.try_complete_milestone(&admin, &1u32, &0u32));
}

// ---------------------------------------------------------------------------
// create_commitment() -- amount == 0, vesting_id == 0
// ---------------------------------------------------------------------------
#[test]
fn test_create_commitment_rejects_zero_amount() {
    let (env, user, client) = setup();
    let hash = BytesN::from_array(&env, &[0u8; 32]);
    assert_rejected!(client.try_create_commitment(&user, &1u32, &0i128, &hash));
}

#[test]
fn test_create_commitment_rejects_zero_vesting_id() {
    let (env, user, client) = setup();
    let hash = BytesN::from_array(&env, &[1u8; 32]);
    assert_rejected!(client.try_create_commitment(&user, &0u32, &100i128, &hash));
}

// ---------------------------------------------------------------------------
// claim_with_path_payment() -- vesting_id == 0, amount <= 0
// ---------------------------------------------------------------------------
#[test]
fn test_claim_with_path_payment_rejects_zero_vesting_id() {
    let (env, user, client) = setup();
    assert_rejected!(client.try_claim_with_path_payment(&user, &0u32, &100i128, &None));
}

#[test]
fn test_claim_with_path_payment_rejects_zero_amount() {
    let (env, user, client) = setup();
    assert_rejected!(client.try_claim_with_path_payment(&user, &1u32, &0i128, &None));
}

// ---------------------------------------------------------------------------
// claim_with_lockup() -- vesting_id == 0, amount <= 0
// ---------------------------------------------------------------------------
#[test]
fn test_claim_with_lockup_rejects_zero_vesting_id() {
    let (env, user, client) = setup();
    assert_rejected!(client.try_claim_with_lockup(&user, &0u32, &100i128));
}

#[test]
fn test_claim_with_lockup_rejects_zero_amount() {
    let (env, user, client) = setup();
    assert_rejected!(client.try_claim_with_lockup(&user, &1u32, &0i128));
}

// ---------------------------------------------------------------------------
// cast_veto_vote() -- reassignment_id == 0, voting_power <= 0
// ---------------------------------------------------------------------------
#[test]
fn test_cast_veto_vote_rejects_zero_reassignment_id() {
    let (env, voter, client) = setup();
    assert_rejected!(client.try_cast_veto_vote(&voter, &0u32, &true, &100i128));
}

#[test]
fn test_cast_veto_vote_rejects_zero_voting_power() {
    let (env, voter, client) = setup();
    assert_rejected!(client.try_cast_veto_vote(&voter, &1u32, &true, &0i128));
}

#[test]
fn test_cast_veto_vote_rejects_negative_voting_power() {
    let (env, voter, client) = setup();
    assert_rejected!(client.try_cast_veto_vote(&voter, &1u32, &true, &(-10i128)));
}

// ---------------------------------------------------------------------------
// configure_tax_withholding() -- bps == 0, bps > 10000
// ---------------------------------------------------------------------------
#[test]
fn test_configure_tax_withholding_rejects_zero_bps() {
    let (env, admin, client) = setup();
    let treasury = Address::generate(&env);
    assert_rejected!(client.try_configure_tax_withholding(&admin, &treasury, &0u32));
}

#[test]
fn test_configure_tax_withholding_rejects_bps_over_10000() {
    let (env, admin, client) = setup();
    let treasury = Address::generate(&env);
    assert_rejected!(client.try_configure_tax_withholding(&admin, &treasury, &10001u32));
}

// ---------------------------------------------------------------------------
// configure_lst() -- vesting_id == 0
// ---------------------------------------------------------------------------
#[test]
fn test_configure_lst_rejects_zero_vesting_id() {
    let (env, admin, client) = setup();
    let lst_addr = Address::generate(&env);
    let base_addr = Address::generate(&env);
    assert_rejected!(client.try_configure_lst(&admin, &0u32, &lst_addr, &base_addr));
}

// ---------------------------------------------------------------------------
// create_vesting_grant() -- vesting_id == 0
// ---------------------------------------------------------------------------
#[test]
fn test_create_vesting_grant_rejects_zero_vesting_id() {
    let (env, admin, client) = setup();
    let beneficiary = Address::generate(&env);
    assert_rejected!(client.try_create_vesting_grant(&admin, &0u32, &beneficiary, &true));
}

// ---------------------------------------------------------------------------
// record_unvested_balance() -- negative amount
// ---------------------------------------------------------------------------
#[test]
fn test_record_unvested_balance_rejects_negative() {
    let (env, admin, client) = setup();
    let beneficiary = Address::generate(&env);
    assert_rejected!(client.try_record_unvested_balance(&admin, &beneficiary, &(-1i128)));
}

// ---------------------------------------------------------------------------
// update_oracle_price() -- zero/negative price
// ---------------------------------------------------------------------------
#[test]
fn test_update_oracle_price_rejects_zero() {
    let (env, admin, client) = setup();
    assert_rejected!(client.try_update_oracle_price(&admin, &0i128));
}

#[test]
fn test_update_oracle_price_rejects_negative() {
    let (env, admin, client) = setup();
    assert_rejected!(client.try_update_oracle_price(&admin, &(-5i128)));
}

// ---------------------------------------------------------------------------
// configure_path_payment() -- negative amount
// ---------------------------------------------------------------------------
#[test]
fn test_configure_path_payment_rejects_negative_amount() {
    let (env, admin, client) = setup();
    let dest = Address::generate(&env);
    let path = Vec::new(&env);
    assert_rejected!(client.try_configure_path_payment(&admin, &dest, &(-1i128), &path));
}

// ---------------------------------------------------------------------------
// configure_lockup() -- vesting_id == 0, duration == 0
// ---------------------------------------------------------------------------
#[test]
fn test_configure_lockup_rejects_zero_vesting_id() {
    let (env, admin, client) = setup();
    let token_addr = Address::generate(&env);
    assert_rejected!(client.try_configure_lockup(&admin, &0u32, &86400u64, &token_addr));
}

#[test]
fn test_configure_lockup_rejects_zero_duration() {
    let (env, admin, client) = setup();
    let token_addr = Address::generate(&env);
    assert_rejected!(client.try_configure_lockup(&admin, &1u32, &0u64, &token_addr));
}

// ---------------------------------------------------------------------------
// simulate_claim() -- vesting_id == 0 returns can_claim = false
// ---------------------------------------------------------------------------
#[test]
fn test_simulate_claim_zero_vesting_id() {
    let (_env, user, client) = setup();
    let sim = client.simulate_claim(&user, &0u32);
    assert!(!sim.can_claim);
}

// ---------------------------------------------------------------------------
// request_beneficiary_reassignment() -- zero vesting_id, negative amount
// ---------------------------------------------------------------------------
#[test]
fn test_reassignment_rejects_zero_vesting_id() {
    let (env, beneficiary, client) = setup();
    let new_beneficiary = Address::generate(&env);
    assert_rejected!(client.try_request_beneficiary_reassignment(
        &beneficiary, &new_beneficiary, &0u32, &100i128,
    ));
}

#[test]
fn test_reassignment_rejects_zero_amount() {
    let (env, beneficiary, client) = setup();
    let new_beneficiary = Address::generate(&env);
    assert_rejected!(client.try_request_beneficiary_reassignment(
        &beneficiary, &new_beneficiary, &1u32, &0i128,
    ));
}

#[test]
fn test_reassignment_rejects_negative_amount() {
    let (env, beneficiary, client) = setup();
    let new_beneficiary = Address::generate(&env);
    assert_rejected!(client.try_request_beneficiary_reassignment(
        &beneficiary, &new_beneficiary, &1u32, &(-1i128),
    ));
}
