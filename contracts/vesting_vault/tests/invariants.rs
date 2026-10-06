#![cfg(test)]

use proptest::prelude::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, Vec,
};
use vesting_vault::{VestingVault, VestingVaultClient};

fn setup() -> (Env, Address, VestingVaultClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_addr = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_addr);
    let admin = Address::generate(&env);
    (env, admin, client)
}

proptest! {
    #[test]
    fn prop_governance_veto(
        supply in 10_000i128..1_000_000_000i128,
        threshold in 1u32..51u32,
    ) {
        let (env, admin, client) = setup();
        env.ledger().set_timestamp(1_000_000);

        client.initialize_token_supply(&admin, &supply);
        client.set_governance_veto_threshold(&admin, &threshold);

        let veto_threshold = supply * i128::from(threshold) / 100;
        prop_assert!(!client.requires_governance_veto(&veto_threshold));
        prop_assert!(client.requires_governance_veto(&(veto_threshold + 1)));

        let current_beneficiary = Address::generate(&env);
        let new_beneficiary = Address::generate(&env);
        client.request_beneficiary_reassignment(
            &current_beneficiary,
            &new_beneficiary,
            &1u32,
            &(veto_threshold + 1),
        );

        let reassignment = client
            .get_beneficiary_reassignment(&1u32)
            .expect("reassignment should be stored");
        prop_assert!(reassignment.requires_governance_veto);

        let voter_a = Address::generate(&env);
        let voter_b = Address::generate(&env);
        client.cast_veto_vote(
            &voter_a,
            &1u32,
            &true,
            &(veto_threshold - 1),
        );

        let (is_vetoed_before, power_before, required_power) =
            client.get_veto_status(&1u32);
        prop_assert!(!is_vetoed_before);
        prop_assert_eq!(power_before, veto_threshold - 1);
        prop_assert_eq!(required_power, veto_threshold);

        client.cast_veto_vote(&voter_b, &1u32, &true, &1i128);

        prop_assert!(
            client.get_beneficiary_reassignment(&1u32).is_none(),
            "reassignment must be removed exactly when the veto threshold is reached"
        );
    }

    #[test]
    fn prop_timelock_enforcement(
        supply in 10_000i128..1_000_000_000i128,
        threshold in 1u32..100u32,
    ) {
        let (env, admin, client) = setup();
        env.ledger().set_timestamp(2_000_000);

        client.initialize_token_supply(&admin, &supply);
        client.set_governance_veto_threshold(&admin, &threshold);

        let threshold_amount = supply * i128::from(threshold) / 100;
        let current_beneficiary = Address::generate(&env);
        let new_beneficiary = Address::generate(&env);

        client.request_beneficiary_reassignment(
            &current_beneficiary,
            &new_beneficiary,
            &1u32,
            &threshold_amount,
        );

        let pending = client
            .get_beneficiary_reassignment(&1u32)
            .expect("reassignment should be stored");
        prop_assert!(!pending.requires_governance_veto);

        let early = client.try_execute_beneficiary_reassignment(&1u32);
        let rejected = match early {
            Err(_) => true,
            Ok(inner) => inner.is_err(),
        };
        prop_assert!(rejected, "reassignment must not execute before its timelock");

        env.ledger().set_timestamp(pending.effective_at);
        client.execute_beneficiary_reassignment(&1u32);

        let executed = client
            .get_beneficiary_reassignment(&1u32)
            .expect("executed reassignment should remain queryable");
        prop_assert!(executed.is_executed);
    }
}

#[test]
fn emergency_pause_requires_quorum_and_expires() {
    let (env, admin, client) = setup();
    env.ledger().set_timestamp(3_000_000);

    let auditor_a = Address::generate(&env);
    let auditor_b = Address::generate(&env);
    let auditor_c = Address::generate(&env);
    let auditors = Vec::from_array(
        &env,
        [auditor_a.clone(), auditor_b.clone(), auditor_c],
    );
    client.initialize_auditors(&admin, &auditors);

    let reason = String::from_str(&env, "invariant");
    client.request_emergency_pause(&auditor_a, &reason);
    assert!(
        !client.is_emergency_paused(),
        "one auditor must not pause the contract"
    );

    client.request_emergency_pause(&auditor_b, &reason);
    assert!(
        client.is_emergency_paused(),
        "two distinct auditors must meet the pause quorum"
    );

    let pause = client
        .get_emergency_pause_status()
        .expect("pause status should be stored");
    assert_eq!(pause.paused_by.len(), 2);

    env.ledger().set_timestamp(pause.expires_at);
    assert!(
        !client.is_emergency_paused(),
        "pause must no longer be active at its expiry boundary"
    );
}
