## Description

<!-- Provide a clear and concise description of what this PR does and why. -->

Closes #

---

## Contract(s) Affected

<!-- List every contract crate touched by this PR. -->

- [ ] `vesting_contracts`
- [ ] `vesting_vault`
- [ ] `vesting_status_nft`
- [ ] `staking_contract`
- [ ] `grant_contracts`
- [ ] `deposit_to_yield_adapter`
- [ ] `insurance_treasury`
- [ ] `lending_contract`
- [ ] `collateral_bridge`
- [ ] `lockup_token`
- [ ] `analytics_adapter`
- [ ] Other: <!-- specify -->

---

## Type of Change

<!-- Check all that apply. -->

- [ ] ✨ Feature — new functionality
- [ ] 🐛 Bug fix — corrects incorrect behaviour
- [ ] 🔒 Security — addresses a vulnerability or hardens the codebase
- [ ] ⚡ Optimisation — improves gas efficiency or performance
- [ ] ♻️ Refactor — code restructure with no functional change
- [ ] 📖 Documentation — docs/comments only
- [ ] 🔧 Chore — tooling, CI, dependency updates

---

## Testing Done

<!-- Describe every test you ran or added. Check all that apply. -->

- [ ] Unit tests added / updated (`cargo test`)
- [ ] Integration tests added / updated
- [ ] Fuzz tests added / updated
- [ ] Formal verification checks run (`formal_reentrancy`)
- [ ] Manual testing on Stellar Testnet

**Test output summary:**

```
<!-- Paste relevant cargo test output here -->
```

---

## Security Implications

<!-- REQUIRED — every PR must address each item below, even if the answer is "N/A". -->

- [ ] **Reentrancy** — cross-contract call ordering is safe; no state written after an external call that could be exploited
- [ ] **Overflow / Underflow** — all arithmetic uses checked or saturating operations
- [ ] **Access Control** — only authorised callers can invoke sensitive entry-points; admin/owner checks verified
- [ ] **Input Validation** — all external inputs are validated before use
- [ ] **Storage / Data Integrity** — no unintended storage mutations; keys are collision-free
- [ ] **Event Emission** — all state-changing operations emit the appropriate events
- [ ] **Upgrade Safety** — storage layout is backwards-compatible (or migration is provided)
- [ ] **Denial of Service** — no unbounded loops, excessively large data structures, or vectors that can be grown by untrusted callers
- [ ] **No new security implications** — this change does not affect contract security in any way

**Security notes:**

<!-- Explain any security trade-offs, mitigations, or assumptions. -->

---

## Gas / Performance Impact

<!-- Describe the effect of this change on resource consumption. -->

- [ ] No significant gas impact
- [ ] Gas usage **increased** — reason: <!-- explain -->
- [ ] Gas usage **decreased** — reason: <!-- explain -->
- [ ] Benchmarks / profiling run

---

## Migration / Upgrade Considerations

<!-- Does this change require a migration step, storage schema change, or contract re-deployment? -->

- [ ] No migration required
- [ ] Migration script provided at: <!-- path -->
- [ ] Breaking change — downstream callers must update

**Migration notes:**

<!-- Describe steps required to safely upgrade deployed contracts. -->

---

## Documentation Updated

- [ ] Inline code comments updated
- [ ] `DOCUMENTATION.md` updated
- [ ] `README.md` updated
- [ ] Changelog entry added
- [ ] No documentation changes needed

---

## Reviewer Notes

<!-- Anything specific you want reviewers to pay attention to. -->
