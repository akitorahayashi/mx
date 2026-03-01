---
label: "refacts"
---

## Goal

Prevent invalid context key states by introducing a strongly-typed model that enforces structural invariants during construction.

## Problem

Context file keys are passed around as raw `&str`, leading to primitive obsession. Invariants are evaluated procedurally rather than encoded in the type system.

## Affected Areas

### Domain

- `src/domain/context_file/key.rs`
- `src/domain/context_file/path_policy.rs`

## Constraints

- `ContextKey` type must enforce valid structural keys at the boundary.
- Do not use ambiguous names or responsibilities such as base, common, core, utils, or helpers.
- Systemic fixes are preferred over patches; invariants and owning components must address boundaries.
- No silent fallbacks.

## Risks

- Breaking existing key references or alias resolution behavior.
- Invalidating paths that were previously allowed by late validation.

## Acceptance Criteria

- Raw `&str` keys are replaced with a strongly-typed `ContextKey` domain model.
- Invariants (alias resolution, pending prefix, dynamic path) are validated upon `ContextKey` construction.
- `ContextKey` completely replaces the usage of raw `&str` and `&Path` at the boundaries in `resolve_context_path` and `validate_path`.

## Implementation Plan

1. Define a strongly-typed `ContextKey` domain model in `src/domain/context_file/key.rs`.
2. Move logic for structurally parsing prefixes, aliases, and suffixes into the initialization of `ContextKey`, preventing invalid states.
3. Update `resolve_context_path` to accept a `ContextKey` instead of `&str`.
4. Update `validate_path` in `src/domain/context_file/path_policy.rs` to rely on the validation already performed during `ContextKey` construction instead of raw strings and paths.
5. Identify and update all call sites passing raw `&str` context keys to parse into `ContextKey` at boundaries.
