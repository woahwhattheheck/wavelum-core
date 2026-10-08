#![cfg(test)]

use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, Vec};
use vesting_vault::{ScheduleParams, VestingVault, VestingVaultClient};

fn setup() -> (Env, Address, VestingVaultClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, VestingVault);
    let client = VestingVaultClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    (env, admin, client)
}

fn schedule(env: &Env, beneficiary: Address, amount: i128, start_time: u64) -> ScheduleParams {
    ScheduleParams {
        beneficiary,
        amount,
        cliff: 86_400,
        duration: 31_536_000,
        start_time,
    }
}

#[test]
fn creates_one_hundred_schedules_in_one_batch() {
    let (env, admin, client) = setup();
    let mut schedules = Vec::new(&env);
    for _ in 0..100 {
        schedules.push_back(schedule(
            &env,
            Address::generate(&env),
            1_000,
            env.ledger().timestamp(),
        ));
    }

    let result = client.batch_create_schedules(&admin, &schedules).unwrap();
    assert_eq!(result.requested, 100);
    assert_eq!(result.succeeded, 100);
    assert_eq!(result.failed, 0);
    assert_eq!(result.results.len(), 100);

    let first_id = result.results.get(0).unwrap().vesting_id.unwrap();
    let last_id = result.results.get(99).unwrap().vesting_id.unwrap();
    assert_eq!(first_id, 1);
    assert_eq!(last_id, 100);

    assert_eq!(
        client.get_batch_schedule_info(&first_id).unwrap().amount,
        1_000
    );
    assert_eq!(
        client.get_batch_schedule_info(&last_id).unwrap().amount,
        1_000
    );

    let estimate = client.estimate_batch_create_resources(&100);
    assert!(estimate.within_limit);
    assert_eq!(estimate.estimated_storage_writes, 201);
    assert_eq!(estimate.estimated_events, 201);
    assert!(!client.estimate_batch_create_resources(&101).within_limit);
}

#[test]
fn schedules_can_be_claimed_independently() {
    let (env, admin, client) = setup();
    let first = Address::generate(&env);
    let second = Address::generate(&env);
    let schedules = Vec::from_array(
        &env,
        [
            schedule(&env, first.clone(), 500, env.ledger().timestamp()),
            schedule(&env, second.clone(), 700, env.ledger().timestamp()),
        ],
    );

    let result = client.batch_create_schedules(&admin, &schedules).unwrap();
    let first_id = result.results.get(0).unwrap().vesting_id.unwrap();
    let second_id = result.results.get(1).unwrap().vesting_id.unwrap();

    // Advance to full vesting before exercising independent claim accounting.
    env.ledger()
        .set_timestamp(env.ledger().timestamp() + 31_536_000);

    client.claim(&first, &first_id, &100).unwrap();
    client.claim(&second, &second_id, &200).unwrap();

    // Each batch schedule keeps its own cumulative claim state.
    assert_eq!(
        client
            .get_batch_schedule_info(&first_id)
            .unwrap()
            .claimed_amount,
        100
    );
    assert_eq!(
        client
            .get_batch_schedule_info(&second_id)
            .unwrap()
            .claimed_amount,
        200
    );

    let claims = client.get_all_claims();
    assert_eq!(claims.len(), 2);
    assert_eq!(claims.get(0).unwrap().vesting_id, first_id);
    assert_eq!(claims.get(1).unwrap().vesting_id, second_id);
}

#[test]
fn batch_schedule_claim_respects_cliff_and_linear_vesting() {
    let (env, admin, client) = setup();
    let beneficiary = Address::generate(&env);
    let start_time = env.ledger().timestamp();
    let schedules = Vec::from_array(
        &env,
        [ScheduleParams {
            beneficiary: beneficiary.clone(),
            amount: 1_000,
            cliff: 100,
            duration: 1_000,
            start_time,
        }],
    );

    let result = client.batch_create_schedules(&admin, &schedules).unwrap();
    let vesting_id = result.results.get(0).unwrap().vesting_id.unwrap();

    let before_cliff = client.try_claim(&beneficiary, &vesting_id, &1);
    assert_eq!(before_cliff, Err(Ok(vesting_vault::Error::CliffNotReached)));

    env.ledger().set_timestamp(start_time + 500);

    // Half the duration has elapsed, so no more than 500 is vested.
    let over_vested = client.try_claim(&beneficiary, &vesting_id, &501);
    assert_eq!(over_vested, Err(Ok(vesting_vault::Error::InvalidAmount)));

    client.claim(&beneficiary, &vesting_id, &500).unwrap();
    let no_more_vested = client.try_claim(&beneficiary, &vesting_id, &1);
    assert_eq!(no_more_vested, Err(Ok(vesting_vault::Error::InvalidAmount)));

    env.ledger().set_timestamp(start_time + 1_000);
    client.claim(&beneficiary, &vesting_id, &500).unwrap();
    assert_eq!(
        client
            .get_batch_schedule_info(&vesting_id)
            .unwrap()
            .claimed_amount,
        1_000
    );
}

#[test]
fn invalid_item_does_not_abort_valid_neighbors() {
    let (env, admin, client) = setup();
    let schedules = Vec::from_array(
        &env,
        [
            schedule(&env, Address::generate(&env), 500, env.ledger().timestamp()),
            schedule(&env, Address::generate(&env), 0, env.ledger().timestamp()),
            schedule(&env, Address::generate(&env), 900, env.ledger().timestamp()),
        ],
    );

    let result = client.batch_create_schedules(&admin, &schedules).unwrap();
    assert_eq!(result.requested, 3);
    assert_eq!(result.succeeded, 2);
    assert_eq!(result.failed, 1);

    let first = result.results.get(0).unwrap();
    let invalid = result.results.get(1).unwrap();
    let third = result.results.get(2).unwrap();

    assert!(first.success);
    assert!(!invalid.success);
    assert!(invalid.vesting_id.is_none());
    assert!(invalid.error_code.is_some());
    assert!(third.success);

    // Failed entries do not consume IDs.
    assert_eq!(first.vesting_id.unwrap(), 1);
    assert_eq!(third.vesting_id.unwrap(), 2);
    assert!(client.get_batch_schedule_info(&1).is_some());
    assert!(client.get_batch_schedule_info(&2).is_some());
}

#[test]
fn invalid_schedule_shape_is_reported_without_mutation() {
    let (env, admin, client) = setup();
    let bad = ScheduleParams {
        beneficiary: Address::generate(&env),
        amount: 1_000,
        cliff: 10,
        duration: 5,
        start_time: env.ledger().timestamp(),
    };
    let schedules = Vec::from_array(&env, [bad]);

    let result = client.batch_create_schedules(&admin, &schedules).unwrap();
    assert_eq!(result.succeeded, 0);
    assert_eq!(result.failed, 1);
    assert!(client.get_batch_schedule_info(&1).is_none());
}
