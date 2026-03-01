---
label: "refacts"
scope: "Replace raw &str context keys with a strongly-typed domain model"
---

## Goal

Prevent invalid context key states by introducing a strongly-typed model that enforces structural invariants during construction.

## Problem

Context file keys are passed around as raw &str, leading to primitive obsession. Invariants are evaluated procedurally rather than encoded in the type system.

## Evidence

- source_event: "primitive-obsession-context-keys-data-arch.md"
  path: "src/domain/context_file/key.rs"
  loc: "6-29"
  note: "`resolve_context_path` accepts a raw `&str` and parses structural invariants ad-hoc (like `pd-` prefixes or numeric suffixes) rather than using a type that enforces a valid, structural key."

- source_event: "primitive-obsession-context-keys-data-arch.md"
  path: "src/domain/context_file/path_policy.rs"
  loc: "16-20"
  note: "`validate_path` performs late validation on `&str` and `&Path` instead of having a `ContextKey` type that rules out invalid traversal segments upon construction at the boundary."

## Change Scope

- `src/domain/context_file/key.rs`
- `src/domain/context_file/path_policy.rs`

## Constraints

- ContextKey type must enforce valid structural keys at the boundary.

## Acceptance Criteria

- Raw &str keys are replaced with a strongly-typed ContextKey domain model.
- Invariants (alias resolution, pending prefix, dynamic path) are validated upon ContextKey construction.
