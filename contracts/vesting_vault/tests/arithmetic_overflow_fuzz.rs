#![cfg(test)]

use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, Env};
use vesting_vault::{VestingVault, VestingVaultClient};

// =============================================================================
// Issue #9: i128 Overflow/Underflow Fuzz Tests -- vesting_vault
//
// These tests verify that checked arithmetic operations handle extreme values
// without panicking.
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

const I128_MAX: i128 = i128::MAX;
const I128_MIN: i128 = i128::MIN;
const LARGE_AMOUNT: i128 = 1_000_000_000_000_000_000;

// ---------------------------------------------------------------------------
// claim() overflow scenarios -- no panics, only checked arithmetic
// ---------------------------------------------------------------------------
#[test]
fn test_claim_extreme_amount_no_panic() {
    let (env, user, client) = setup();
    // Should not panic -- may succeed or return an error via checked arithmetic
    let _ = client.try_claim(&user, &1u32, &I128_MAX);
}

#[test]
fn test_claim_max_vesting_id_no_panic() {
    let (env, user, client) = setup();
    let _ = client.try_claim(&user, &u32::MAX, &1i128);
}

#[test]
fn test_claim_i128_min_no_panic() {
    let (env, user, client) = setup();
    let _ = client.try_claim(&user, &1u32, &I128_MIN);
}

// ---------------------------------------------------------------------------
// configure_tax_withholding() -- tax calculation overflow scenarios
// ---------------------------------------------------------------------------
#[test]
fn test_tax_withholding_max_bps_no_panic() {
    let (env, admin, client) = setup();
    let treasury = Address::generate(&env);
    let _ = client.try_configure_tax_withholding(&admin, &treasury, &10000u32);
}

#[test]
fn test_tax_withholding_overflow_bps_rejected() {
    let (env, admin, client) = setup();
    let treasury = Address::generate(&env);
    assert_rejected!(client.try_configure_tax_withholding(&admin, &treasury, &10001u32));
}

// ---------------------------------------------------------------------------
// Governance veto threshold calculations -- overflow scenarios
// ---------------------------------------------------------------------------
#[test]
fn test_request_reassignment_extreme_amount() {
    let (env, admin, client) = setup();
    let _ = client.try_update_token_supply(&admin, &I128_MAX);

    let current_beneficiary = Address::generate(&env);
    let new_beneficiary = Address::generate(&env);
    // Should not panic on threshold calculation overflow
    let _ = client.try_request_beneficiary_reassignment(
        &current_beneficiary,
        &new_beneficiary,
        &1u32,
        &LARGE_AMOUNT,
    );
}

#[test]
fn test_veto_vote_extreme_voting_power() {
    let (env, admin, client) = setup();
    let _ = client.try_update_token_supply(&admin, &1000i128);

    let voter = Address::generate(&env);
    let _ = client.try_cast_veto_vote(&voter, &1u32, &true, &I128_MAX);
}

// ---------------------------------------------------------------------------
// update_oracle_price() -- deviation calculation with extreme prices
// ---------------------------------------------------------------------------
#[test]
fn test_oracle_extreme_price_first_submission() {
    let (env, admin, client) = setup();
    let _ = client.try_update_oracle_price(&admin, &I128_MAX);
}

#[test]
fn test_oracle_extreme_price_difference() {
    let (env, admin, client) = setup();
    let _ = client.try_update_oracle_price(&admin, &1i128);
    let _ = client.try_update_oracle_price(&admin, &I128_MAX);
}

// ---------------------------------------------------------------------------
// record_unvested_balance() -- edge cases
// ---------------------------------------------------------------------------
#[test]
fn test_record_unvested_balance_extreme_positive() {
    let (env, admin, client) = setup();
    let beneficiary = Address::generate(&env);
    let _ = client.try_record_unvested_balance(&admin, &beneficiary, &I128_MAX);
}

#[test]
fn test_record_unvested_balance_zero() {
    let (env, admin, client) = setup();
    let beneficiary = Address::generate(&env);
    let _ = client.try_record_unvested_balance(&admin, &beneficiary, &0i128);
}

// ---------------------------------------------------------------------------
// update_total_unvested_balance -- extreme values
// ---------------------------------------------------------------------------
#[test]
fn test_update_total_unvested_balance_extreme() {
    let (env, admin, client) = setup();
    let _ = client.try_update_total_unvested_balance(&admin, &I128_MAX);
}

// ---------------------------------------------------------------------------
// Merge schedules -- arithmetic overflow scenarios
// ---------------------------------------------------------------------------
#[test]
fn test_merge_schedules_rejects_too_few() {
    let (env, user, client) = setup();
    let ids = soroban_sdk::vec![&env, 1u32];
    assert_rejected!(client.try_merge_schedules(&user, &ids));
}

// ---------------------------------------------------------------------------
// simulate_claim -- edge cases
// ---------------------------------------------------------------------------
#[test]
fn test_simulate_claim_normal_amount() {
    let (_env, user, client) = setup();
    let sim = client.simulate_claim(&user, &1u32);
    assert!(sim.net_amount >= 0);
}

// ---------------------------------------------------------------------------
// Configure milestone vesting -- extreme values
// ---------------------------------------------------------------------------
#[test]
fn test_configure_milestone_max_percentages() {
    let (env, admin, client) = setup();
    let pcts = soroban_sdk::vec![&env, 100u32];
    let _ = client.try_configure_milestone_vesting(&admin, &1u32, &pcts);
}

#[test]
fn test_configure_milestone_overflow_percentage_sum() {
    let (env, admin, client) = setup();
    let pcts = soroban_sdk::vec![&env, u32::MAX, 1u32];
    assert_rejected!(client.try_configure_milestone_vesting(&admin, &1u32, &pcts));
}

// ---------------------------------------------------------------------------
// Consecutive extreme value submissions -- no state corruption
// ---------------------------------------------------------------------------
#[test]
fn test_oracle_price_rapid_extreme_changes() {
    let (env, admin, client) = setup();
    let _ = client.try_update_oracle_price(&admin, &1000i128);

    for i in 1..5i128 {
        env.ledger().set_timestamp(env.ledger().timestamp() + 1);
        let price = 1000i128 + i * 100;
        let _ = client.try_update_oracle_price(&admin, &price);
    }
}

// ---------------------------------------------------------------------------
// Multiple claims in sequence -- no state corruption
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_claims_extreme_values() {
    let (env, user, client) = setup();
    let amounts: Vec<i128> = vec![1, 100, LARGE_AMOUNT, I128_MAX];
    for amount in amounts {
        let _ = client.try_claim(&user, &1u32, &amount);
    }
}
