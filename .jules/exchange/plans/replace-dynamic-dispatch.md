---
label: "refacts"
---

## Goal

Optimize performance by reducing dynamic dispatch in core logic.

## Problem

The application extensively uses dynamic dispatch in core logic (`&dyn SnippetCatalog`, `&dyn Clipboard`, `Option<&dyn ContextFileStore>`), particularly in `src/app/commands/copy/mod.rs` and the wrapping logic in `src/app/api.rs`. This relies on runtime vtable lookups and prevents compiler optimizations (e.g., inlining) that would be available with static dispatch.

## Affected Areas

### API Boundary and Commands

- `src/app/commands/copy/mod.rs`
- `src/app/api.rs`

## Constraints

- Changing the function signatures should not alter the externally observable behavior of the `copy` command.
- The implementations provided in production (e.g., `SystemClipboard`, `LocalContextFileStore`) must still seamlessly plug into the generic parameters.

## Risks

- Changing to static dispatch will propagate generic bounds up the call stack. Need to make sure `api.rs` wrappers correctly implement or forward generic parameters.
- Could increase compile times or binary size slightly due to monomorphization, though performance usually outweighs this in core logic.

## Acceptance Criteria

- `copy::execute` uses generic parameters bounded by the respective traits (e.g., `<C: SnippetCatalog>`) instead of trait objects (`&dyn SnippetCatalog`).
- `api.rs` functions that call into the modified command use static dispatch.
- Test suite passes with these signature changes.

## Implementation Plan

1. Modify `src/app/commands/copy/mod.rs`:
   - Change the `execute` signature from taking trait objects (`&dyn SnippetCatalog`, `&dyn Clipboard`, `Option<&dyn ContextFileStore>`) to generic bounds (`<C: SnippetCatalog, L: Clipboard, S: ContextFileStore>`).
   - Also change `expand_placeholders` and `render_placeholder` to use static dispatch for the `ContextFileStore`.
2. Modify `src/app/api.rs`:
   - Update `copy_snippet` and any other affected functions to remove trait object coercion (e.g., `.as_ref().map(|store| store as &dyn ContextFileStore)`). Instead, pass the structures directly or via reference appropriately.
3. Fix test signatures:
   - Update any tests in `src/app/commands/copy/mod.rs` to ensure they compile with the new generic signature.
