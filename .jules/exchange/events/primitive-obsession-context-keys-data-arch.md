---
created_at: "2026-03-01"
author_role: "data_arch"
confidence: "high"
---

## Statement

Context file keys are passed around as raw `&str` instead of a strongly-typed domain model (e.g., `ContextKey`), leading to primitive obsession. Invariants such as alias resolution, pending prefix logic, and dynamic path completion are evaluated procedurally rather than encoded in the type system. This allows invalid states to be expressed and processed throughout the application logic.

## Evidence

- path: "src/domain/context_file/key.rs"
  loc: "6-29"
  note: "`resolve_context_path` accepts a raw `&str` and parses structural invariants ad-hoc (like `pd-` prefixes or numeric suffixes) rather than using a type that enforces a valid, structural key."
- path: "src/domain/context_file/path_policy.rs"
  loc: "16-20"
  note: "`validate_path` performs late validation on `&str` and `&Path` instead of having a `ContextKey` type that rules out invalid traversal segments upon construction at the boundary."
