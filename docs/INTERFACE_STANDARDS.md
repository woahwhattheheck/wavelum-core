# Contract Interface Standards

## Overview

All contracts in the wavelum-core project must implement standardized trait-based interfaces to ensure consistency and interoperability.

## Interfaces

### IVestingVault
Required for: `vesting_contracts`, `vesting_vault`, `vesting_nft_wrapper`

| Method | Description |
|--------|-------------|
| `create_vesting_schedule` | Create a new vesting schedule |
| `get_claimable` | Get claimable amount at current time |
| `claim` | Claim vested tokens |
| `get_total_vested` | Get total vested amount |
| `cancel_vesting` | Cancel a vesting schedule (admin) |

### IStaking
Required for: `staking_contract`

| Method | Description |
|--------|-------------|
| `stake` | Stake tokens into pool |
| `unstake` | Unstake tokens from pool |
| `claim_rewards` | Claim staking rewards |
| `get_staked_amount` | Get staked amount for a user |
| `get_pending_rewards` | Get pending rewards |
| `get_total_value_locked` | Get TVL |

### IGrant
Required for: `grant_contracts`

| Method | Description |
|--------|-------------|
| `create_grant` | Create a new grant |
| `get_grant` | Get grant details |
| `claim_grant` | Claim grant funds |
| `cancel_grant` | Cancel a grant (admin) |
| `get_beneficiary_grants` | Get all grants for a beneficiary |

## Compliance Verification

Run interface compliance tests:
```bash
cargo test --test interface_compliance
```

## Adding New Contracts

1. Create the contract crate under `contracts/<name>/`
2. Implement the appropriate interface trait from `contracts/interfaces/mod.rs`
3. Add the contract to `Cargo.toml` workspace members
4. Run `cargo test --test interface_compliance` to verify
