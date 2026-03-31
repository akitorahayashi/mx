---
label: "refacts"
implementation_ready: false
---

## Goal

Consolidate duplicate path validation logic into a shared single source of truth and encapsulate this validated state within strong domain types (e.g., `ValidatedContextPath` or `SafeSnippetPath`).

## Problem

Duplicate code exists across the codebase for path traversal and absolute path validation, creating security risks due to diverging implementations. Additionally, functions like `validate_path` or `normalize_query` perform these invariant checks but return untyped `PathBuf` or `String` values. The type system thus does not guarantee that a path has been validated, leading to repeated checks or implicit trust across module boundaries.

## Context

Security constraints (like preventing path traversals) should be implemented uniformly. Currently, validation is duplicated across `validate_relative_components` (`src/domain/context_file/path_policy.rs`), `ensure_safe_segments` (`src/domain/snippet/query.rs`), and `extract_relative_path` (`src/app/commands/add/mod.rs`). By introducing strongly typed paths that enforce these checks at construction time, invalid state becomes unrepresentable. API boundaries can then demand these safe types instead of untyped strings or paths.

## Evidence

- path: "src/domain/context_file/path_policy.rs"
  loc: "line 6, 14-20"
  note: "`validate_relative_components` duplicates traversal checks. `validate_path` returns `Result<(), AppError>`, leaving the path uncertified."

- path: "src/domain/snippet/query.rs"
  loc: "line 4-18, 22"
  note: "`normalize_query` returns an untyped `String`. `ensure_safe_segments` duplicates traversal checks."

- path: "src/app/commands/add/mod.rs"
  loc: "line 22"
  note: "`extract_relative_path` duplicates traversal checks inside its loop."

## Change Scope

- `src/domain/context_file/path_policy.rs`
- `src/domain/context_file/key.rs`
- `src/domain/snippet/query.rs`
- `src/app/commands/add/mod.rs`
- `src/app/commands/touch/mod.rs`

## Constraints

- Ensure any new safe path types can easily interoperate with stdlib IO operations by implementing `AsRef<Path>` where appropriate.

## Acceptance Criteria

- Path traversal validation logic is consolidated into a single source of truth.
- Functions like `validate_path` and `normalize_query` are replaced or updated to return a strong type (e.g., `SafePath`).
- Functions requiring safe paths specify the new strong type in their signature, eliminating the need to re-validate.