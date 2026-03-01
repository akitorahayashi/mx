---
label: "refacts"
---

## Goal

Improve error diagnostics by switching from transparent string errors to structured domain errors.

## Problem

The `AppError` enum mixes low-level I/O errors (via `#[from] io::Error`) and string-based domain errors (like `ConfigError(String)`, `NotFound(String)`). This makes it difficult to reason about the exact source or recovery of an error without parsing strings. Additionally, public API functions return `AppError` directly, exposing stringly-typed variants to callers rather than structured context-rich data.

## Affected Areas

### Domain Error Layer

- `src/domain/error.rs`
- `src/app/api.rs`

## Constraints

- Changing the `AppError` definition will likely require updating many instantiation points throughout the codebase that use the string-based constructors (like `AppError::not_found("...")`).
- The public API should expose structured, semantic errors.

## Risks

- Widespread changes needed across adapters and commands to construct new structured errors instead of strings.
- Tests that assert on exact error strings may break and require updating.

## Acceptance Criteria

- `AppError` variants use structured data rather than pure Strings. For instance, `NotFound` might include the item type and identifier (`{ resource_type: ResourceType, id: String }`) instead of just a formatted string.
- `src/app/api.rs` functions return the newly structured domain errors.
- Test suite passes with these signature changes.

## Implementation Plan

1. Modify `src/domain/error.rs`:
   - Redefine variants like `NotFound(String)` and `ConfigError(String)` to use structured records.
   - For example:
     ```rust
     #[error("Not found: {resource}")]
     NotFound { resource: String, details: String },
     ```
     or similarly for `ConfigError`, `ClipboardError`, `InvalidKey`, `PathTraversal`.
   - Update the constructor methods (e.g., `AppError::not_found`) to accept more granular arguments.
2. Update Call Sites:
   - Identify all places using `AppError::not_found`, `AppError::config_error`, `AppError::clipboard_error`, etc.
   - Update these calls to provide the required structured arguments.
3. Modify `src/app/api.rs`:
   - Ensure the API propagates the new structured errors properly.
4. Update Tests:
   - Fix all test cases asserting on `AppError` variants to match the new structured formats.
