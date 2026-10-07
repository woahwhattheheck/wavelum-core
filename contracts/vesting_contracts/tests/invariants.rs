#![cfg(test)]

use proptest::prelude::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env,
};
use vesting_contracts::{DataKey, VestingContract, VestingContractClient, Vault};

const START_TS: u64 = 1_000_000;

fn setup_lazy_vault(amount: i128, duration: u64, start_time: u64) -> (Env, Address, u64) {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(start_time);

    let contract_id = env.register(VestingContract, ());
    let client = VestingContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let token = Address::generate(&env);

    client.initialize(&admin, &amount);
    client.set_token(&token);

    let end_time = start_time + duration;
    let vault_id = client.create_vault_lazy(
        &owner,
        &amount,
        &start_time,
        &end_time,
        &0i128,
        &true,
        &false,
        &0u64,
    );

    (env, contract_id, vault_id)
}

proptest! {
    #[test]
    fn prop_claimable_is_bounded_and_monotonic(
        amount in 1i128..1_000_000_000i128,
        duration in 2u64..1_000_000u64,
        first_seed in 0u64..1_000_000u64,
        second_seed in 0u64..1_000_000u64,
    ) {
        let (env, contract_id, vault_id) = setup_lazy_vault(amount, duration, START_TS);
        let client = VestingContractClient::new(&env, &contract_id);

        let first_elapsed = first_seed % (duration + 1);
        let second_elapsed = second_seed % (duration + 1);
        let (earlier, later) = if first_elapsed <= second_elapsed {
            (first_elapsed, second_elapsed)
        } else {
            (second_elapsed, first_elapsed)
        };

        env.ledger().set_timestamp(START_TS + earlier);
        let first = client.get_claimable_amount(&vault_id);

        env.ledger().set_timestamp(START_TS + later);
        let second = client.get_claimable_amount(&vault_id);

        prop_assert!(first >= 0);
        prop_assert!(first <= amount);
        prop_assert!(second >= first);
        prop_assert!(second <= amount);
    }

    #[test]
    fn prop_token_conservation(
        amount in 1i128..1_000_000_000i128,
        duration in 2u64..1_000_000u64,
        released_bps in 0u32..10_001u32,
    ) {
        let (env, contract_id, vault_id) = setup_lazy_vault(amount, duration, START_TS);
        let client = VestingContractClient::new(&env, &contract_id);
        env.ledger().set_timestamp(START_TS + duration);

        let released = amount * i128::from(released_bps) / 10_000;
        let mut vault: Vault = client.get_vault(&vault_id);
        let mut allocation = vault
            .allocations
            .get(0)
            .expect("lazy vault should have one allocation");
        allocation.released_amount = released;
        vault.allocations.set(0, allocation);

        env.as_contract(&contract_id, || {
            env.storage()
                .instance()
                .set(&DataKey::VaultData(vault_id), &vault);
        });

        let unclaimed = client.get_claimable_amount(&vault_id);
        prop_assert!(unclaimed >= 0);
        prop_assert_eq!(released + unclaimed, amount);
        prop_assert!(unclaimed <= amount - released);
    }

    #[test]
    fn prop_random_start_and_cliff_amount_are_bounded(
        amount in 1i128..1_000_000_000i128,
        duration in 172_801u64..31_536_001u64,
        start_time in 1_000_000u64..1_000_000_000u64,
        cliff_percentage in 0u32..10_001u32,
        cliff_seed in 0u64..31_536_001u64,
    ) {
        let (env, contract_id, vault_id) =
            setup_lazy_vault(amount, duration, start_time);
        let client = VestingContractClient::new(&env, &contract_id);

        let smoothing_duration = 86_400u64;
        let max_cliff_duration = duration - smoothing_duration;
        let cliff_duration = cliff_seed % (max_cliff_duration + 1);
        client.configure_cliff_smoothing(
            &vault_id,
            &cliff_duration,
            &smoothing_duration,
            &cliff_percentage,
        );

        let cliff_amount = amount * i128::from(cliff_percentage) / 10_000;
        prop_assert!(cliff_amount >= 0);
        prop_assert!(cliff_amount <= amount);

        env.ledger().set_timestamp(start_time);
        prop_assert_eq!(client.get_claimable_amount(&vault_id), 0);

        let cliff_end = start_time + cliff_duration;
        env.ledger().set_timestamp(cliff_end);
        let vested_at_cliff = client.get_claimable_amount(&vault_id);
        prop_assert!(vested_at_cliff >= 0);
        prop_assert!(vested_at_cliff <= amount - cliff_amount);

        let smoothing_end = cliff_end + smoothing_duration;
        env.ledger().set_timestamp(smoothing_end);
        let vested_after_smoothing = client.get_claimable_amount(&vault_id);
        prop_assert!(vested_after_smoothing >= vested_at_cliff);
        prop_assert!(vested_after_smoothing >= cliff_amount);
        prop_assert!(vested_after_smoothing <= amount);

        env.ledger().set_timestamp(start_time + duration);
        prop_assert_eq!(client.get_claimable_amount(&vault_id), amount);
    }
}
