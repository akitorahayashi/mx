---
label: "refacts"
---

## Goal

Consolidate duplicate path validation logic into a shared single source of truth and encapsulate this validated state within strong domain types, e.g., `SafePath`.

## Current State

Duplicate code exists across the codebase for path traversal and absolute path validation, creating security risks due to diverging implementations. Functions perform invariant checks but return untyped `PathBuf` or `String` values. The type system thus does not guarantee that a path has been validated, leading to repeated checks or implicit trust across module boundaries.

- `src/domain/context_file/path_policy.rs`: `validate_relative_components` duplicates traversal checks. `validate_path` returns `Result<(), AppError>`, leaving the path uncertified.
- `src/domain/snippet/query.rs`: `normalize_query` returns an untyped `String`. `ensure_safe_segments` duplicates traversal checks.
- `src/app/commands/add/mod.rs`: `extract_relative_path` duplicates traversal checks inside its loop.
- `src/app/commands/touch/mod.rs`: Relies on `resolve_validated_context_path` returning a raw `PathBuf` instead of a strongly typed path.
- `src/domain/context_file/key.rs`: Defines `resolve_validated_context_path` returning a raw `PathBuf`.

## Plan

1. **Create strongly typed SafePath**
   - Create `src/domain/path_policy.rs` to house a `SafePath` struct containing a validated `PathBuf`.
   - Implement constructor `SafePath::try_from_path(path: &Path) -> Result<SafePath, AppError>` that encapsulates the standard traversal validations (checking `std::path::Component::Normal` and `CurDir`).
   - Implement `AsRef<Path>` for `SafePath` for seamless stdlib IO integration.
2. **Refactor Context File Path Policy**
   - Delete `src/domain/context_file/path_policy.rs` as its functionality is moving to the domain level `path_policy.rs`.
   - Update `src/domain/context_file/mod.rs` to stop exporting the old `path_policy`.
   - Update `src/domain/mod.rs` to export the new `path_policy`.
3. **Refactor Context File Key Resolution**
   - In `src/domain/context_file/key.rs`, update `resolve_validated_context_path` to use `SafePath::try_from_path` to do validation and return `Result<SafePath, AppError>`.
4. **Refactor Snippet Query**
   - In `src/domain/snippet/query.rs`, remove `ensure_safe_segments`.
   - Update `normalize_query` to construct a `SafePath` from the string it normalized, replacing its current `String` return type to return `Result<SafePath, AppError>`.
5. **Refactor Commands**
   - In `src/app/commands/add/mod.rs`, update `extract_relative_path` to leverage `SafePath::try_from_path` rather than rolling its own validation loop. Update to return `Result<SafePath, AppError>`.
   - In `src/app/commands/touch/mod.rs`, update `execute` to handle `SafePath` returned by `resolve_validated_context_path`. Update `TouchOutcome` to store the inner `PathBuf` or the `SafePath`.
6. **Update Dependent Code**
   - Search for usages of the changed functions (`normalize_query`, `extract_relative_path`, `resolve_validated_context_path`) and update them to handle `SafePath`.
7. **Test Updates**
   - Update tests in `src/domain/context_file/path_policy.rs` to target `SafePath::try_from_path`.
   - Update tests in `src/domain/snippet/query.rs` to check the strong `SafePath` type.
   - Update dependent component tests (such as app commands `add` and `touch`) to accommodate the safe path usage without relying on duplicated logic.
8. **Documentation Updates**
   - Update architectural documentation (such as `AGENTS.md` if any path validation policy is described) to declare `SafePath` as the sole boundary for validated paths.

## Acceptance Criteria

- Path traversal validation logic is consolidated into a single source of truth.
- Functions like `validate_path` and `normalize_query` are replaced or updated to return a strong type (e.g., `SafePath`).
- Functions requiring safe paths specify the new strong type in their signature, eliminating the need to re-validate.

## Risks

- Widespread compilation errors due to `PathBuf` vs `SafePath` mismatch in callers that aren't updated.
- Missed validation checks if `SafePath` bypasses existing implicit constraints.
