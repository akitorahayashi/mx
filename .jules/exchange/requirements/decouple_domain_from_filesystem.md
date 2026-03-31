---
label: "refacts"
implementation_ready: false
---

## Goal

Decouple the core domain logic and application commands from the filesystem by abstracting file I/O behind ports, removing `absolute_path` from the domain model, and improving test isolation.

## Problem

The core domain model for a snippet (`SnippetEntry`) leaks filesystem concepts by exposing an `absolute_path`. Consequently, application commands like `copy`, `list`, and `checkout` perform direct filesystem reads (`std::fs::read_to_string`) on `snippet_entry.absolute_path` after resolving entities via the `SnippetCatalog` abstraction port. This coupling degrades test isolation, forcing pure unit tests to rely on real filesystem interactions (e.g., `tempfile`).

## Context

A domain object representing a snippet shouldn't need a filesystem path to be manipulated or read. `SnippetEntry` should ideally represent the loaded snippet or the port (`SnippetStore`/`SnippetCatalog`) should be responsible for returning the snippet contents based on its domain identifier. The current approach forces logic tests to use `fs::write` just to mock contents, whereas a better seam would allow in-memory content retrieval.

## Evidence

- path: "src/domain/snippet/catalog_entry.rs"
  loc: "line 6"
  note: "`absolute_path` leaks filesystem details into the core domain."

- path: "src/app/commands/copy/mod.rs"
  loc: "line 21, 23, 94-100"
  note: "Direct `fs::read_to_string` couples `copy` command to the filesystem and forces `tempfile` use in unit tests."

- path: "src/app/commands/list/mod.rs"
  loc: "line 19, 21, 48-60"
  note: "Direct `fs::read_to_string` couples `list` command to the filesystem and forces `tempfile` use in unit tests."

## Change Scope

- `src/domain/snippet/catalog_entry.rs`
- `src/app/commands/copy/mod.rs`
- `src/app/commands/list/mod.rs`
- `src/app/commands/checkout/mod.rs`
- `src/domain/ports/snippet_store.rs`
- `src/domain/ports/snippet_catalog.rs`
- `src/testing/ports/in_memory_catalog.rs`
- `src/adapters/snippet_catalog/filesystem_catalog.rs`

## Constraints

- Backward compatibility for existing snippet structures must be preserved during loading.
- Test suites must be updated to take advantage of the newly created abstraction, utilizing purely in-memory representations when possible.

## Acceptance Criteria

- `SnippetEntry` no longer contains `absolute_path`.
- Commands (`copy`, `list`, `checkout`) use a port (`SnippetStore` or updated `SnippetCatalog`) to fetch snippet content instead of `std::fs::read_to_string`.
- Application logic unit tests for these commands no longer require `tempfile` or direct filesystem interactions.