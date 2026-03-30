---
label: "refacts"
created_at: "2024-03-30"
author_role: "qa"
confidence: "high"
---

## Problem

Application commands `copy` and `list` unnecessarily perform direct filesystem reads from `std::fs` after resolving entities via the `SnippetCatalog` abstraction port, which couples core logic to side effects and degrades test isolation.

## Goal

Decouple the `copy` and `list` commands from direct file I/O by having the `SnippetCatalog` (or `SnippetStore`) provide snippet contents.

## Context

The `copy` and `list` commands isolate snippet lookup with `SnippetCatalog`, but then immediately fall back to `fs::read_to_string(&snippet_entry.absolute_path)` for the content. This forces pure logic testing to use `tempfile` and `fs::write` just to mock the contents. A better seam would abstract reading the contents, allowing tests to run entirely in memory.

## Evidence

- path: "src/app/commands/copy/mod.rs"
  loc: "23"
  note: "Direct `fs::read_to_string` couples `copy` command to the filesystem."

- path: "src/app/commands/copy/mod.rs"
  loc: "94-100"
  note: "Forces `tempfile` and `fs::write` into pure unit tests to satisfy the hardcoded `fs` read."

- path: "src/app/commands/list/mod.rs"
  loc: "21"
  note: "Direct `fs::read_to_string` couples `list` command to the filesystem to read frontmatter."

- path: "src/app/commands/list/mod.rs"
  loc: "48-60"
  note: "Forces `tempfile` and `fs::write` into pure unit tests to satisfy the hardcoded `fs` read."

## Change Scope

- `src/app/commands/copy/mod.rs`
- `src/app/commands/list/mod.rs`
- `src/domain/ports/snippet_store.rs`
- `src/domain/ports/snippet_catalog.rs`
- `src/testing/ports/in_memory_catalog.rs`
- `src/adapters/snippet_catalog/filesystem_catalog.rs`
