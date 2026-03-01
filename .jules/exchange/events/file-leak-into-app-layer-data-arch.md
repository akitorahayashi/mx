---
created_at: "2026-03-01"
author_role: "data_arch"
confidence: "high"
---

## Statement

The application layer (`app/commands/copy/mod.rs`) performs explicit `fs::read_to_string` operations. This violates the port/adapter boundary because the `SnippetCatalog` should abstract away the storage mechanism. A domain-focused `Snippet` concept would load its own content rather than forcing the caller to use `std::fs` on an absolute path.

## Evidence

- path: "src/app/commands/copy/mod.rs"
  loc: "19"
  note: "Directly uses `fs::read_to_string(&snippet_entry.absolute_path)` rather than delegating to a domain port or snippet representation."
- path: "src/domain/snippet/catalog_entry.rs"
  loc: "4-8"
  note: "`SnippetEntry` exposes `absolute_path` publicly, encouraging caller to bypass abstraction layers and directly access the filesystem."
