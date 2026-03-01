---
label: "tests"
---

## Goal

Add test coverage for `SymlinkCheckout` boundary conditions.

## Problem

Tests `creates_symlink_in_target_root` and `skips_existing_symlink` are present, but no test covers the error path when `target_path` exists as a normal file, which is an important boundary condition for filesystem interactions.

## Affected Areas

### SymlinkCheckout

- `src/adapters/snippet_checkout/symlink_checkout.rs`

## Constraints

- Cover boundary cases (e.g. destination exists as a regular file).
- Tests should not rely on manual interaction.

## Risks

- Failures or panic conditions might occur in production when creating symlinks in an occupied target path.

## Acceptance Criteria

- `SymlinkCheckout` test covers error path when `target_path` exists as a normal file.

## Implementation Plan

1. Write a test case where `target_path` already exists as a normal file, expecting a specific error return.
