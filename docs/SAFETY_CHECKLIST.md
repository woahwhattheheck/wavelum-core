# Soroban Smart Contract Safety Checklist

This document serves as a standard safety checklist for all Soroban smart contract contributions to the project.

## Pre-Submission Checklist

Before submitting a PR for smart contract code, please ensure the following:

### 1. Reentrancy Protection ✓

- [ ] Contract uses guard patterns to prevent reentrancy attacks
- [ ] All external calls follow the "checks-effects-interactions" pattern
- [ ] Cross-contract invocations are protected by mutexes or guards
- [ ] Formal reentrancy tests pass: `cargo test -p vesting_contracts formal_reentrancy`
- [ ] No recursive state modifications without guard protection

### 2. Integer Overflow/Underflow Protection ✓

- [ ] All arithmetic uses checked operations (checked_add, checked_sub, checked_mul)
- [ ] Division operations handle zero divisor cases
- [ ] Token amounts respect u128 bounds
- [ ] Vesting calculations cannot cause overflow
- [ ] No unchecked arithmetic in production code

### 3. Access Control Validation ✓

- [ ] All privileged functions verify caller authorization
- [ ] Admin functions check for admin role
- [ ] Beneficiary operations verify vault ownership
- [ ] Public functions document their access requirements
- [ ] No default-to-public access patterns

### 4. Panic Safety (no unwrap/expect in production code) ✓

- [ ] Zero `unwrap()` calls in non-test production code
- [ ] Zero `expect()` calls in non-test production code
- [ ] All Result types handled with proper error propagation
- [ ] Option types converted to Result for error handling
- [ ] Panic only occurs in test code or due to invariant violations

### 5. Storage Leak Detection ✓

- [ ] No orphaned storage entries created
- [ ] All storage allocations have corresponding cleanup logic
- [ ] Deleted vaults properly remove all associated state
- [ ] No accumulation of unused temporary data
- [ ] Storage operations use formal storage patterns

## CI Enforcement

The following checks are **automatically enforced** in CI and **block merges on failure**:

### Clippy Linting (Strict Mode)

```bash
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all
```

- All clippy warnings treated as errors
- Pedantic lints enabled for code quality
- Security-focused lints enabled

### Unsafe Pattern Scanning

```bash
./scripts/contract-safety.sh
```

Checks for:

- unwrap/expect in production code
- Missing authorization patterns
- Reentrancy vulnerabilities
- Code formatting compliance

### Formal Verification

```bash
cargo test -p vesting_contracts formal_reentrancy -- --nocapture
```

- All reentrancy tests must pass
- Malicious token callback tests must pass
- Invariant violations must be caught

### Code Formatting

```bash
cargo fmt --all -- --check
```

- All code must follow Rust conventions
- Formatting failures block merges

## Safety Patterns - Examples

### Correct Access Control Pattern

```rust
pub fn claim_tokens(env: Env, vault_id: u64, amount: i128) -> Result<i128, Error> {
    let beneficiary = env.invoker();

    // Access control check
    let vault = get_vault(&env, vault_id)?;
    if vault.beneficiary != beneficiary {
        return Err(Error::UnauthorizedAccess);
    }

    // Safe state modifications
    vault.amount -= amount;
    env.storage().instance().set(&vault_key(vault_id), &vault);

    Ok(amount)
}
```

### Correct Arithmetic Pattern

```rust
// CORRECT: Use checked arithmetic
let new_balance = balance.checked_add(amount)
    .ok_or(Error::Overflow)?;

// INCORRECT: Unchecked arithmetic
let new_balance = balance + amount; // Can overflow!
```

### Correct Option Handling Pattern

```rust
// CORRECT: Use match or ok_or for Options
let value = maybe_value.ok_or(Error::NotFound)?;

// INCORRECT: Using expect/unwrap
let value = maybe_value.unwrap(); // Panics on None!
```

## Formal Verification Tests

The project includes formal reentrancy verification that tests:

1. **Basic BMC-level verification** - Ensures reentrancy guards work
2. **Malicious token callback attacks** - Tests behavior when tokens call back
3. **Multi-hop path payment attacks** - Tests complex callback chains
4. **Recursive state modifications** - Verifies atomic state updates

All formal tests must pass before code merges to main.

## Automated CI/CD Pipeline

The safety checks run automatically on all PRs:

```
1. Build contracts for WASM target
2. Run Clippy with strict settings (-D warnings)
3. Check code formatting (cargo fmt --check)
4. Scan for unwrap/expect in production
5. Run formal reentrancy verification
6. Run full test suite
```

**Merge blocker**: Any failing safety check prevents merge to main.

## References

- [Soroban Official Documentation](https://developers.stellar.org/docs/smart-contracts)
- [Security Best Practices for Smart Contracts](./SECURITY.md)
- [Formal Invariants Documentation](./INVARIANTS.md)
- [Reentrancy Testing](./contracts/vesting_contracts/tests/formal_reentrancy_verification.rs)

---

**Last Updated**: July 2024
**Maintained by**: Security Team
