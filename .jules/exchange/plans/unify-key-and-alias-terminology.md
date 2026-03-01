---
label: "refacts"
---

## Goal

Clearly define and uniformly apply the terminology distinguishing 'Key' and 'Alias' across the CLI, internal domain registry, and documentation.

## Problem

The terms 'Key' and 'Alias' are used interchangeably without a clear distinction, leading to ambiguity regarding context file identifiers. For example:
- CLI arguments use 'key' to refer to context file identifiers (e.g., `Touch { key: String }`, `Cat { key: String }` in `src/app/cli/mod.rs`).
- The internal registry and lookup functions use the term 'alias' instead of 'key' (e.g., `resolve_alias(key: &str)` in `src/domain/context_file/alias_registry.rs`).
- Documentation conflates the two terms without establishing a canonical definition (e.g., "Context Management Keys (Aliases)" in `README.md`).

## Affected Areas

### CLI and App

- `src/app/cli/mod.rs`

### Domain

- `src/domain/context_file/alias_registry.rs`

### Documentation

- `README.md`

## Constraints

- The terminology for 'Key' and 'Alias' must be formally distinguished, or unified under a single canonical term if they refer to exactly the same concept.
- If they are distinct concepts, this distinction must be visible at the domain boundary and correctly reflected in documentation.

## Risks

- Renaming CLI arguments might affect users passing arguments by name if the CLI parser uses the argument name explicitly.
- Inconsistent renaming could lead to further drift.

## Acceptance Criteria

- Internal domain (`alias_registry.rs`) and external CLI (`mod.rs`) uniformly apply the selected terminology (either clearly separating 'Key' and 'Alias' or unifying them).
- The `README.md` and documentation clearly define the chosen terminology and eliminate the conflated reference.
- All code structures, functions, and arguments consistently use the chosen term.

## Implementation Plan

1. Determine the canonical term (either 'Key' or 'Alias') or establish clear definitions for both if they are distinct. Assume 'Alias' for context file identifiers for consistency with `alias_registry`.
2. Update `src/app/cli/mod.rs` to replace CLI arguments like `key: String` with `alias: String` for `Touch` and `Cat` commands.
3. Update `src/domain/context_file/alias_registry.rs` to rename parameters like `key: &str` to `alias: &str` in `resolve_alias` and similar functions.
4. Update `README.md` to remove the conflated "Context Management Keys (Aliases)" and instead use "Context Management Aliases".
5. Verify no other usages of 'key' representing 'alias' exist in the codebase.
