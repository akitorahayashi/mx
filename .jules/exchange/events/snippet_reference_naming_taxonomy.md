---
label: "refacts"
created_at: "2024-03-30"
author_role: "taxonomy"
confidence: "high"
---

## Problem

Identifiers used to locate a given snippet (or command) are inconsistently named. `snippet`, `path`, `query`, and `key` are used across various API boundaries, CLI arguments, and internal methods to mean the same thing.

## Goal

Standardize the terminology for a snippet identifier to improve code readability and refactoring safety.

## Context

The system needs to identify snippets by some string (e.g., "w/wc"). The name of this string parameter changes depending on where you look in the code. Some commands use `snippet`, others use `path` (even though it's resolved via a query and isn't strictly an OS filesystem path from the user's perspective). The catalog adapter takes a `raw_query`, and the resulting struct has a `key` and a `relative_path`. Context files, on the other hand, strictly use `key`.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "line 38"
  note: "`mx copy` CLI command uses parameter name `snippet`."
- path: "src/app/cli/mod.rs"
  loc: "line 40"
  note: "`mx checkout` CLI command uses parameter name `path`."
- path: "src/app/api.rs"
  loc: "line 56"
  note: "`checkout_snippets` API method uses parameter name `query`."
- path: "src/domain/ports/snippet_catalog.rs"
  loc: "line 6"
  note: "`resolve_snippet` uses `raw_query`."
- path: "src/domain/snippet/catalog_entry.rs"
  loc: "line 6"
  note: "`SnippetEntry` differentiates between `key`, `relative_path`, and `absolute_path`."

## Change Scope

- `src/app/cli/`
- `src/app/commands/`
- `src/app/api.rs`
- `src/domain/ports/snippet_catalog.rs`
- `src/adapters/snippet_catalog/`
