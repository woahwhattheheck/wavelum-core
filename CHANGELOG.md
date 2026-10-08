# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Entries are grouped by workspace member or repository area.

### vesting_vault

#### Added

- Admin and emergency-withdrawal events on the vault.

#### Fixed

- Explicit overflow checking in `i128` arithmetic.
- Comprehensive input validation and bounds checks.

### Contracts workspace (all members)

#### Added

- Standardized trait-based contract interfaces, interface standards
  documentation, and interface-compliance tests.
- Solidity ↔ Soroban interface synchronization validation
  (`scripts/sync-interfaces.sh`).

### Repository and CI

#### Added

- GitHub issue templates (bug report, feature request, contract audit) and a
  template validation workflow.
- `CONTRIBUTING.md` development guidelines and a pull request template.
- Code coverage reporting workflow and `Makefile` coverage targets.
- CI triggers for all branches, path filters, and fuzz testing.
- Contract safety checks in CI.
- `changelog-check` CI job validating this file.

#### Fixed

- Contact and Discussions links pointing at the actual repository.
