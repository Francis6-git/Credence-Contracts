# `credence_bond` Crate Layout

This document describes the file structure and responsibilities of the `contracts/credence_bond` crate after moving the canonical bond implementation into the crate.

## Files

- `contracts/credence_bond/src/lib.rs`
  - Core contract entrypoint and public `CredenceBond` contract implementation.
  - Contains storage key definitions, bond data model, event helpers, reentrancy guard, and public contract methods.
  - Also includes pure Rust helpers for bond validation and unit tests.

- `contracts/credence_bond/src/types.rs`
  - Bond attestation types and deduplication key definitions.
  - Shared data shapes used by contract attestation flow.

- `contracts/credence_bond/src/weighted_attestation.rs`
  - Weighted attestation stake and config management.
  - Computes attester weights for attestation operations.

- `contracts/credence_bond/src/nonce.rs`
  - Nonce tracking utilities for replay prevention.
  - Implements persistent nonce storage, consume semantics, and TTL extension.

- `contracts/credence_bond/src/early_exit_penalty.rs`
  - Early exit penalty configuration and calculation.
  - Stores treasury address and penalty basis points.

- `contracts/credence_bond/src/rolling_bond.rs`
  - Rolling bond renewal helper logic.
  - Determines when a rolling period has ended and applies renewal.

- `contracts/credence_bond/src/slashing.rs`
  - Slashing helper logic for bond penalty execution.
  - Performs admin-controlled slashing and emits events.

- `contracts/credence_bond/src/tiered_bond.rs`
  - Tier derivation and tier-change event emission.
  - Encapsulates Bronze/Silver/Gold/Platinum threshold logic.

## Notes

- The contract now uses a consistent `notice_period_duration` field across the bond model.
- Root-level unresolved merge artifacts (`ours.rs`, `base.rs`, `theirs.rs`) have been migrated into the crate logic and will be removed after validation.
- The `credence_bond` crate is now self-contained with dedicated submodules for each domain concept.
