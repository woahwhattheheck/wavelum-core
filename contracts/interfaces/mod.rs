//! Standardized Contract Interfaces
//!
//! Trait-based interfaces for each contract type.
//! All contracts must implement their expected interface traits.
//! Compile-time compliance is verified via interface_compliance test.

use soroban_sdk::{Address, Env, Symbol, String};

/// Interface for Vesting Vault contracts
///
/// All vesting vaults must implement this trait to ensure
/// a consistent API for vesting schedule management.
pub trait IVestingVault {
    /// Create a new vesting schedule for a beneficiary
    fn create_vesting_schedule(
        env: Env,
        beneficiary: Address,
        total_amount: i128,
        start_time: u64,
        cliff_end: u64,
        end_time: u64,
    ) -> Result<Symbol, String>;

    /// Get the claimable amount for a beneficiary at the current time
    fn get_claimable(env: Env, beneficiary: Address) -> i128;

    /// Claim vested tokens
    fn claim(env: Env, beneficiary: Address) -> Result<i128, String>;

    /// Get the total vested amount for a beneficiary
    fn get_total_vested(env: Env, beneficiary: Address) -> i128;

    /// Cancel a vesting schedule (admin only)
    fn cancel_vesting(env: Env, admin: Address, beneficiary: Address) -> Result<(), String>;
}

/// Interface for Staking contracts
///
/// All staking contracts must implement this trait to ensure
/// a consistent API for staking operations.
pub trait IStaking {
    /// Stake tokens into the staking pool
    fn stake(env: Env, staker: Address, amount: i128) -> Result<(), String>;

    /// Unstake tokens from the staking pool
    fn unstake(env: Env, staker: Address, amount: i128) -> Result<(), String>;

    /// Claim staking rewards
    fn claim_rewards(env: Env, staker: Address) -> Result<i128, String>;

    /// Get the total staked amount for a staker
    fn get_staked_amount(env: Env, staker: Address) -> i128;

    /// Get pending rewards for a staker
    fn get_pending_rewards(env: Env, staker: Address) -> i128;

    /// Get the total value locked in the staking pool
    fn get_total_value_locked(env: Env) -> i128;
}

/// Interface for Grant contracts
///
/// All grant contracts must implement this trait to ensure
/// a consistent API for grant distribution.
pub trait IGrant {
    /// Create a new grant
    fn create_grant(
        env: Env,
        admin: Address,
        beneficiary: Address,
        amount: i128,
        start_time: u64,
        end_time: u64,
    ) -> Result<Symbol, String>;

    /// Get grant details
    fn get_grant(env: Env, grant_id: Symbol) -> Option<(Address, i128, u64, u64, bool)>;

    /// Claim grant funds
    fn claim_grant(env: Env, beneficiary: Address, grant_id: Symbol) -> Result<i128, String>;

    /// Cancel a grant (admin only)
    fn cancel_grant(env: Env, admin: Address, grant_id: Symbol) -> Result<(), String>;

    /// Get total grants for a beneficiary
    fn get_beneficiary_grants(env: Env, beneficiary: Address) -> Vec<Symbol>;
}

/// Interface compliance marker trait
///
/// Contracts implement this to signal that they comply with
/// a specific interface. Used for compile-time verification.
pub trait InterfaceCompliant {
    /// Returns the interface name this contract implements
    fn interface_name() -> &'static str;
}
