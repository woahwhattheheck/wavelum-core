# Lumina Core

[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://pre-commit.com/)

Soroban smart contracts for the Lumina Network â€” a blockchain-based vesting vault and token streaming infrastructure with governance, staking, inheritance, lending, and cross-chain capabilities on Stellar.

## Contracts

This workspace contains the following Soroban contract crates:

| Contract | Description |
|----------|-------------|
| `vesting_contracts` | Core vesting vault logic with token streaming schedules |
| `vesting_vault` | Individual vault management, claims, and revocation |
| `vesting_status_nft` | NFT representation of vesting vault ownership and status |
| `staking_contract` | Auto-stake integration for yield generation on locked tokens |
| `grant_contracts` | Grant-based token distribution contracts |
| `deposit_to_yield_adapter` | Yield adapter for depositing locked tokens into yield strategies |
| `insurance_treasury` | Insurance treasury for protocol risk management |
| `lending_contract` | Lending protocol integration for vault collateralization |
| `collateral_bridge` | Cross-chain collateral bridge |
| `lockup_token` | Lockup token with time-based release schedule |
| `analytics_adapter` | On-chain analytics data adapter |

## Key Features

### Defensive Governance
72-hour challenge period with 51% veto threshold, protecting beneficiaries from malicious admin actions. Governable actions include admin rotation, contract upgrade, and emergency pause.

### Auto-Stake
Tokens stay locked in the vault while registered as an active stake on whitelisted staking contracts via synchronous cross-contract calls. Yield can be claimed at any time without unstaking.

### Inheritance (Dead-Man's Switch)
Primary beneficiaries can nominate a backup address with an inactivity timer. If the primary is inactive beyond the configured duration, the backup can claim vault ownership through a challenge-based succession process.

### Governance Functions
- `propose_admin_rotation` / `propose_contract_upgrade` / `propose_emergency_pause`
- `vote_on_proposal` / `execute_proposal`
- `get_proposal_info` / `get_voter_power` / `get_total_locked`

## Tech Stack

- **Language:** Rust â€” with `soroban-sdk` v25.3.1
- **Network:** Stellar Soroban (Testnet)
- **Build System:** Cargo workspace with 7 member crates

## Getting Started

```bash
cargo test --workspace
```

### Prerequisites

- Rust toolchain (see `rust-toolchain.toml`)
- Soroban CLI

## Documentation

See [DOCUMENTATION.md](./DOCUMENTATION.md) for the complete protocol documentation including:
- Feature specifications and contract API reference
- Formal verification and security guidance
- Integration guides for staking and inheritance

## Project Structure

```
lumina-core/
â”œâ”€â”€ contracts/               # Soroban contract crates
â”‚   â”œâ”€â”€ vesting_contracts/   # Core vesting logic
â”‚   â”œâ”€â”€ vesting_vault/       # Vault management
â”‚   â”œâ”€â”€ vesting_status_nft/  # Vesting NFT
â”‚   â”œâ”€â”€ staking_contract/    # Auto-staking
â”‚   â”œâ”€â”€ grant_contracts/     # Grant distribution
â”‚   â”œâ”€â”€ deposit_to_yield_adapter/
â”‚   â”œâ”€â”€ insurance_treasury/
â”‚   â””â”€â”€ ...                  # Additional contracts
â”œâ”€â”€ analytics/               # Analytics service
â”œâ”€â”€ scripts/                 # Deployment and utility scripts
â”œâ”€â”€ social/                  # Social integration contracts
â”œâ”€â”€ doc_tests/               # Documentation test suites
â”œâ”€â”€ Cargo.toml               # Workspace manifest
â””â”€â”€ rust-toolchain.toml      # Toolchain configuration
```

## Related Repositories

- [lumina-frontend](https://github.com/stellar-network-builders/lumina-frontend) â€” Next.js web dashboard
- [lumina-backend](https://github.com/stellar-network-builders/lumina-backend) â€” Node.js API and services
