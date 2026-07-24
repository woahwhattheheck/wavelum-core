# Contributing to Lumina Core

Thank you for your interest in contributing to Lumina Core! This document provides guidelines and standards for contributing to the Soroban smart contracts for the Lumina Network.

## Table of Contents

- [Branch Naming](#branch-naming)
- [Commit Conventions](#commit-conventions)
- [Rust Coding Standards](#rust-coding-standards)
- [Testing Requirements](#testing-requirements)
- [PR Checklist](#pr-checklist)
- [Local Development Setup](#local-development-setup)
- [Pull Request Process](#pull-request-process)

## Branch Naming

All branches must follow the conventional commit format with a prefix:

- `feat/` - New features
- `fix/` - Bug fixes
- `chore/` - Maintenance tasks, dependency updates, configuration changes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring without functional changes
- `test/` - Test additions or modifications

**Examples:**
- `feat/add-auto-stake-integration`
- `fix/reentrancy-vulnerability`
- `chore/update-soroban-sdk`
- `docs/api-reference-update`
- `refactor/simplify-vault-logic`
- `test/add-unit-tests-for-claims`

## Commit Conventions

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. Commit messages must be formatted as:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

### Scopes

Common scopes include contract names:
- `vesting_contracts`
- `vesting_vault`
- `staking_contract`
- `grant_contracts`
- `deposit_to_yield_adapter`
- `insurance_treasury`
- `lending_contract`
- `collateral_bridge`
- `lockup_token`
- `analytics_adapter`

### Examples

```
feat(vault): add claim function for beneficiaries

Implement the claim function that allows beneficiaries to withdraw
their vested tokens according to the schedule.

Closes #123
```

```
fix(governance): resolve veto threshold calculation bug

The veto threshold was incorrectly calculated as 50% instead of
51%, allowing malicious proposals to pass.

Fixes #456
```

```
test(staking): add unit tests for auto-stake registration

Add comprehensive unit tests covering:
- Successful stake registration
- Invalid stake contract rejection
- Already registered error handling
```

## Rust Coding Standards

### Naming Conventions

Follow Rust's standard naming conventions:

- **Functions and methods**: `snake_case`
  ```rust
  fn claim_vault_tokens() { }
  fn get_vault_balance() { }
  ```

- **Structs and Enums**: `PascalCase`
  ```rust
  struct VaultInfo { }
  enum ProposalStatus { }
  ```

- **Constants and Statics**: `SCREAMING_SNAKE_CASE`
  ```rust
  const MAX_VAULTS: u32 = 1000;
  const CHALLENGE_PERIOD: u64 = 72 * 60 * 60;
  ```

- **Type Parameters**: `PascalCase`, typically single letter `T`
  ```rust
  fn process_result<T: Display>(result: T) { }
  ```

### Documentation

All public items must be documented with `///` doc comments:

```rust
/// Claims vested tokens from the vault for the beneficiary.
///
/// # Arguments
///
/// * `vault_id` - The unique identifier of the vault
/// * `beneficiary` - The address claiming the tokens
///
/// # Returns
///
/// Returns the amount of tokens claimed.
///
/// # Errors
///
/// Returns `Error::NotClaimable` if the vault is not yet claimable.
/// Returns `Error::Unauthorized` if the caller is not the beneficiary.
///
/// # Examples
///
/// ```ignore
/// let amount = claim_vault(env, vault_id, beneficiary);
/// ```
pub fn claim_vault(env: &Env, vault_id: u64, beneficiary: Address) -> u128 {
    // Implementation
}
```

### Error Handling

- Use the `soroban_sdk::Error` or custom error enums for contract errors
- Define errors in a dedicated `Error` enum
- Provide clear error messages

```rust
#[derive(Error)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Vault not found")]
    VaultNotFound,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Not claimable yet")]
    NotClaimable,
}
```

- Use `Result<T, Error>` for fallible functions
- Avoid `unwrap()` and `expect()` in production code
- Use `?` operator for error propagation

```rust
pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    // ... implementation
    Ok(())
}
```

### Code Style

- Use `cargo fmt` to format code before committing
- Run `cargo clippy` and address all warnings
- Keep functions focused and small (< 50 lines when possible)
- Extract complex logic into helper functions
- Use meaningful variable and function names

## Testing Requirements

### Unit Tests

All new functions must have corresponding unit tests:

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_claim_vault_success() {
        let env = Env::default();
        // Setup
        let vault_id = create_test_vault(&env);
        let beneficiary = Address::generate(&env);
        
        // Execute
        let amount = claim_vault(&env, vault_id, beneficiary);
        
        // Assert
        assert!(amount > 0);
    }

    #[test]
    fn test_claim_vault_unauthorized() {
        let env = Env::default();
        let vault_id = create_test_vault(&env);
        let unauthorized = Address::generate(&env);
        
        let result = claim_vault(&env, vault_id, unauthorized);
        assert_eq!(result, Err(Error::Unauthorized));
    }
}
```

### Test Coverage Requirements

- All public functions must have unit tests
- Test both success and error paths
- Edge cases and boundary conditions must be covered
- Integration tests for cross-contract calls

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests with all features
cargo test --all-features

# Run tests for a specific contract
cargo test -p vesting_contracts

# Run tests with output
cargo test -- --nocapture
```

## PR Checklist

Before submitting a pull request, ensure:

- [ ] Branch follows naming convention (`feat/`, `fix/`, `chore/`, `docs/`, `refactor/`, `test/`)
- [ ] Commits follow conventional commits format
- [ ] Code is formatted with `cargo fmt`
- [ ] `cargo clippy` passes without warnings
- [ ] All tests pass: `cargo test --workspace`
- [ ] New functions have unit tests with >80% coverage
- [ ] Documentation is updated for public API changes
- [ ] `CONTRIBUTING.md` is followed (this file!)
- [ ] PR description clearly describes the change
- [ ] Linked issues are referenced (e.g., `Closes #123`)

## Local Development Setup

### Prerequisites

1. **Rust Toolchain**
   - Install Rust from [rustup.rs](https://rustup.rs/)
   - The project uses Rust 1.91.0 (see `rust-toolchain.toml`)
   - Required targets: `wasm32v1-none`
   - Required components: `rustfmt`, `clippy`

   ```bash
   rustup install 1.91.0
   rustup target add wasm32v1-none
   rustup component add rustfmt clippy
   ```

2. **Stellar CLI (Soroban)**
   - Install Stellar CLI v25.1.0 or later
   - Download from [GitHub releases](https://github.com/stellar/stellar-cli/releases)

   ```bash
   # Linux
   wget https://github.com/stellar/stellar-cli/releases/download/v25.1.0/stellar-cli-25.1.0-x86_64-unknown-linux-gnu.tar.gz
   tar -xzf stellar-cli-25.1.0-x86_64-unknown-linux-gnu.tar.gz
   sudo mv stellar /usr/local/bin/
   
   # Verify installation
   stellar --version
   ```

3. **Cargo Workspace Setup**
   ```bash
   # Clone the repository
   git clone https://github.com/stellar-network-builders/lumina-core.git
   cd lumina-core
   
   # Build the workspace
   cargo build --target wasm32v1-none --release
   ```

### Development Workflow

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test --workspace

# Build contracts
cargo build --target wasm32v1-none --release

# Run specific contract tests
cargo test -p vesting_contracts
```

### Useful Commands

```bash
# Check for unused dependencies
cargo +nightly udeps

# Check for outdated dependencies
cargo outdated

# Generate documentation
cargo doc --no-deps --open

# Run formal verification tests
cargo test -p vesting_contracts formal_reentrancy -- --nocapture
```

## Pull Request Process

1. **Fork and Branch**
   - Fork the repository
   - Create a branch following naming conventions
   - Make your changes

2. **Test Locally**
   - Run `cargo fmt`
   - Run `cargo clippy`
   - Run `cargo test --workspace`
   - Ensure all checks pass

3. **Commit**
   - Follow conventional commits format
   - Write clear, descriptive commit messages

4. **Push and Create PR**
   - Push your branch to your fork
   - Create a pull request to `main`
   - Fill in the PR template
   - Reference related issues

5. **Address Feedback**
   - Respond to reviewer comments
   - Make requested changes
   - Update tests if needed

6. **Merge**
   - Once approved and CI passes, the PR will be merged

## CI/CD Pipeline

The project uses GitHub Actions for continuous integration:

- **Smart Contracts CI** runs on:
  - Push to `main`
  - Pull requests to `main`

- **CI Checks:**
  - Rust toolchain installation (1.91.0)
  - Stellar CLI installation (v25.1.0)
  - Contract build for `wasm32v1-none` target
  - Formal reentrancy verification
  - Unit tests for all workspace members
  - Tests with all features enabled

## Additional Resources

- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Project README](./README.md)
- [Full Documentation](./DOCUMENTATION.md)

## Questions?

For questions or clarifications, please:
- Open an issue with the `question` label
- Reach out in project discussions
- Check existing documentation first

Happy contributing! 🚀
