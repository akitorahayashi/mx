---
label: "refacts"
---

## Goal

Create a single source of truth for the Snippet domain model and push implicit filesystem operations out of the application layer into the Snippet catalog/adapter.

## Problem

Redundant definitions of snippet metadata and path structures exist across layers (SnippetEntry, SnippetFrontmatter, ListEntry). Furthermore, implicit file reading in the application layer breaks the boundary sovereignty principle.

## Affected Areas

### Domain layer

- `src/domain/snippet/catalog_entry.rs`
- `src/domain/snippet/frontmatter.rs`

### Application layer

- `src/app/commands/copy/mod.rs`
- `src/app/commands/list/mod.rs`

## Constraints

- The application layer must not perform explicit std::fs operations.
- A single unified Snippet domain model should serve as the source of truth.
- Do not modify files outside of the .jules/ directory.

## Risks

- The application layer commands might not compile properly if the domain port interface is incorrectly implemented.
- Unifying domain structs might require more extensive refactoring if other modules rely on the old structures.

## Acceptance Criteria

- Snippet metadata and path concepts are unified under a single Snippet domain model.
- Filesystem operations are removed from the application layer and handled by the Snippet adapter.

## Implementation Plan

1. Unify Snippet domain models.
   - Combine `SnippetEntry` and `SnippetFrontmatter` into a single cohesive domain struct in the `src/domain/snippet/` directory.
   - Eliminate the `ListEntry` struct from `src/app/commands/list/mod.rs` and update the code to use the unified domain struct.
2. Delegate file reading to adapter.
   - Remove `fs::read_to_string` from `src/app/commands/copy/mod.rs` and `src/app/commands/list/mod.rs`.
   - Implement the file reading logic within the Snippet catalog adapter, exposing it through domain port methods.
3. Remove redundant structures.
   - Delete `SnippetEntry` and `SnippetFrontmatter` structs if they are fully replaced.
   - Ensure the newly formed abstraction acts as the absolute source of truth.