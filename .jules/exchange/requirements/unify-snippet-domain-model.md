---
label: "refacts"
scope: "Consolidate redundant Snippet structs and prevent app layer from reading files directly"
---

## Goal

Create a single source of truth for the Snippet domain model and push implicit filesystem operations out of the application layer into the Snippet catalog/adapter.

## Problem

Redundant definitions of snippet metadata and path structures exist across layers (SnippetEntry, SnippetFrontmatter, ListEntry). Furthermore, implicit file reading in the application layer breaks the boundary sovereignty principle.

## Evidence

- source_event: "redundant-snippet-structs-data-arch.md"
  path: "src/domain/snippet/catalog_entry.rs"
  loc: "4-8"
  note: "Defines `SnippetEntry` containing only the key, relative_path, and absolute_path. This represents a partial state."

- source_event: "redundant-snippet-structs-data-arch.md"
  path: "src/domain/snippet/frontmatter.rs"
  loc: "57-65"
  note: "Defines `SnippetFrontmatter` representing snippet metadata (title, description). This is decoupled from `SnippetEntry`."

- source_event: "redundant-snippet-structs-data-arch.md"
  path: "src/app/commands/list/mod.rs"
  loc: "6-12"
  note: "Defines `ListEntry` as an ad-hoc DTO that merges the path concepts from `SnippetEntry` with the metadata from `SnippetFrontmatter`."

- source_event: "redundant-snippet-structs-data-arch.md"
  path: "src/app/commands/list/mod.rs"
  loc: "19-24"
  note: "Performs direct `fs::read_to_string` to fetch snippet contents within the application layer, violating the port/adapter boundary and demonstrating that `SnippetCatalog` is an incomplete abstraction."

- source_event: "file-leak-into-app-layer-data-arch.md"
  path: "src/app/commands/copy/mod.rs"
  loc: "19"
  note: "Directly uses `fs::read_to_string(&snippet_entry.absolute_path)` rather than delegating to a domain port or snippet representation."

- source_event: "file-leak-into-app-layer-data-arch.md"
  path: "src/domain/snippet/catalog_entry.rs"
  loc: "4-8"
  note: "`SnippetEntry` exposes `absolute_path` publicly, encouraging caller to bypass abstraction layers and directly access the filesystem."

## Change Scope

- `src/app/commands/copy/mod.rs`
- `src/app/commands/list/mod.rs`
- `src/domain/snippet/catalog_entry.rs`
- `src/domain/snippet/frontmatter.rs`

## Constraints

- The application layer must not perform explicit std::fs operations.
- A single unified Snippet domain model should serve as the source of truth.

## Acceptance Criteria

- Snippet metadata and path concepts are unified under a single Snippet domain model.
- Filesystem operations are removed from the application layer and handled by the Snippet adapter.
