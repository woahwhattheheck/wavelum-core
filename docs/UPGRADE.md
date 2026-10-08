# Contract upgradeability (native WASM upgrade)

`VestingVault` implements the Soroban native upgrade mechanism (issue #8,
option B) instead of a data/logic contract split. `Env::deployer()
.update_current_contract_wasm()` swaps the executing WASM while keeping all
instance and persistent storage in place, which avoids a state migration
and keeps every existing storage key stable.

## Entry points

- `upgrade(admin, new_wasm_hash)` — admin-gated (`admin.require_auth()`)
  swap of the contract code. It first calls the existing
  `assert_safe_to_upgrade` guard: if `contract_total_unvested > 0` the call
  fails with `UpgradeBlockedByUnvestedFunds` so user funds can never be
  trapped by an upgrade.
- `version()` — returns the contract version constant; bump it in each
  released build so operators can verify an upgrade took effect.

## Upgrade procedure

1. Build the new contract WASM:
   `cargo build -p vesting_vault --target wasm32v1-none --release`
2. Upload the WASM to the ledger and obtain its SHA-256 hash (via the
   Stellar CLI `stellar contract upload`, or programmatically through
   `Deployer::upload_contract_wasm`).
3. Ensure `get_total_unvested_balance()` is `0`, or the call will be
   rejected by the unvested-funds guard.
4. Invoke `upgrade(admin, wasm_hash)` from the admin account.
5. Verify `version()` reports the new build and re-check a known storage
   value (e.g. `get_merkle_roots()`) to confirm state preservation.

## Compatibility requirements

- New code must keep every storage key/type layout stable, or perform its
  own migration on first invocation — `update_current_contract_wasm` does
  not transform storage.
- The unvested-funds guard means upgrades are only executable when no
  unclaimed vesting balances remain, or when the operator deliberately
  winds the contract down.
