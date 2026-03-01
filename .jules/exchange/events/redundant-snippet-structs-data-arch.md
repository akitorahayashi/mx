---
created_at: "2026-03-01"
author_role: "data_arch"
confidence: "high"
---

## Statement

Redundant definitions of snippet metadata and path structures exist across layers. `SnippetEntry` (in `domain`) captures paths, `SnippetFrontmatter` (in `domain`) captures metadata, while `ListEntry` (in `app`) redundantly unifies these. This indicates a missing single source of truth for a complete `Snippet` domain model. Furthermore, implicit file reading is happening in the application layer (`app/commands/list/mod.rs`), breaking the boundary sovereignty principle by leaking filesystem operations into domain orchestration.

## Evidence

- path: "src/domain/snippet/catalog_entry.rs"
  loc: "4-8"
  note: "Defines `SnippetEntry` containing only the key, relative_path, and absolute_path. This represents a partial state."
- path: "src/domain/snippet/frontmatter.rs"
  loc: "57-65"
  note: "Defines `SnippetFrontmatter` representing snippet metadata (title, description). This is decoupled from `SnippetEntry`."
- path: "src/app/commands/list/mod.rs"
  loc: "6-12"
  note: "Defines `ListEntry` as an ad-hoc DTO that merges the path concepts from `SnippetEntry` with the metadata from `SnippetFrontmatter`."
- path: "src/app/commands/list/mod.rs"
  loc: "19-24"
  note: "Performs direct `fs::read_to_string` to fetch snippet contents within the application layer, violating the port/adapter boundary and demonstrating that `SnippetCatalog` is an incomplete abstraction."
