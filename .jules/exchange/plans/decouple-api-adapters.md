---
label: "refacts"
---

## Goal

Improve the architectural separation of concerns by ensuring the API layer does not directly instantiate specific adapter implementations.

## Problem

The API layer inconsistently mixes adapter instantiation with dependency injection, hardcoding specific adapters like `LocalContextFileStore` and `SymlinkCheckout`.

## Affected Areas

### API Layer
- `src/app/api.rs`

## Constraints

- API facade must remain infrastructure-agnostic.

## Risks

- Breaking changes in API function signatures may impact external callers and CLI initialization logic.
- Potential runtime errors if dependencies are incorrectly wired or injected.

## Acceptance Criteria

- API layer functions accept adapter dependencies via injection rather than hardcoded instantiation.

## Implementation Plan

1. Modify functions in `src/app/api.rs` (around lines 22, 27, 43, 53) to accept `ContextFileStore` and `SnippetCheckout` trait implementations as arguments instead of directly instantiating `LocalContextFileStore` and `SymlinkCheckout`.
2. Update the calling code (e.g., CLI layer in `src/cli.rs` or `src/main.rs`) to instantiate the specific adapters (`LocalContextFileStore::new()`, `SymlinkCheckout::new()`) and inject them into the API functions.
3. Ensure that trait bounds and lifetimes are correctly specified in `src/app/api.rs` to support the injected dependencies.
4. Run all unit and integration tests to verify no functionality is broken by the refactor.
