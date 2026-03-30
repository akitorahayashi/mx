---
label: "refacts"
created_at: "2024-03-30"
author_role: "data_arch"
confidence: "high"
---

## Problem

The core domain model for a snippet (`SnippetEntry`) has filesystem concepts leaked into it. It includes an `absolute_path`. Operations like `copy`, `list`, and `checkout` read directly from `absolute_path` inside the core domain layer.

## Goal

The core domain model should be sovereign and independent of the transport or persistence layer. File loading should be abstracted via a port (e.g., `SnippetStore`), and `SnippetEntry` shouldn't be concerned with filesystem paths unless that is explicitly part of the model's domain, but reading files directly from `absolute_path` is a leak.

## Context

A domain object representing a snippet shouldn't need a filesystem path to be parsed and manipulated. In fact, `catalog.resolve_snippet` returns `SnippetEntry`, and `execute` in the `copy`, `list`, and `checkout` modules reads `absolute_path` directly. This couples the core domain logic to the filesystem. The `SnippetStore` port should abstract this reading capability, returning the content directly, or `SnippetEntry` should contain the `content` (maybe lazily loaded, or loaded up-front).

## Evidence

- path: "src/domain/snippet/catalog_entry.rs"
  loc: "line 6"
  note: "`absolute_path` leaks filesystem details into the core domain."
- path: "src/app/commands/copy/mod.rs"
  loc: "line 21"
  note: "Direct `fs::read_to_string` on `snippet_entry.absolute_path` bypasses any abstraction that `SnippetStore` or `SnippetCatalog` might have."
- path: "src/app/commands/list/mod.rs"
  loc: "line 19"
  note: "Directly uses `fs::read_to_string` instead of using a port."

## Change Scope

- `src/domain/snippet/catalog_entry.rs`
- `src/app/commands/copy/mod.rs`
- `src/app/commands/list/mod.rs`
- `src/app/commands/checkout/mod.rs`
- `src/adapters/snippet_catalog/filesystem_catalog.rs`
