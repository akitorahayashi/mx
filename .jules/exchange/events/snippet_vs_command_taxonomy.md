---
label: "refacts"
created_at: "2024-03-30"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The repository uses both "snippet" and "command" interchangeably to refer to the same underlying concept: a markdown file managed by the system and output to the clipboard or checked out as a symlink. This creates ambiguity.

## Goal

Establish one canonical term ("snippet" or "command") for this concept and rename files, directories, configuration keys, and APIs consistently.

## Context

The repository stores these items in `~/.config/mx/commands/` and checks them out into `.mx/commands/`. The CLI command is `mx create-command`. However, the internal domain types use `SnippetEntry`, `SnippetCatalog`, `SnippetStore`, and CLI arguments are often named `snippet`. The application's purpose as a "snippet" manager is explicitly referenced in the `mx` CLI about string ("Unified CLI for mx snippets") but the data is stored in a `commands/` directory.

## Evidence

- path: "src/domain/snippet/catalog_entry.rs"
  loc: "struct SnippetEntry"
  note: "Domain type uses 'Snippet'."
- path: "src/app/cli/mod.rs"
  loc: "line 59"
  note: "CLI command is `CreateCommand` and its help text says 'Create a new command template in .mx/commands/'."
- path: "src/adapters/snippet_catalog/filesystem_catalog.rs"
  loc: "line 12"
  note: "The variable is `commands_root` but it is within `FilesystemSnippetCatalog`."
- path: "README.md"
  loc: "line 5"
  note: "Refers to 'Snippet command' and mentions 'any markdown snippet under ~/.config/mx/commands/'."

## Change Scope

- `src/domain/snippet/`
- `src/domain/ports/`
- `src/adapters/snippet_catalog/`
- `src/adapters/snippet_store/`
- `src/adapters/snippet_checkout/`
- `src/app/cli/`
- `src/app/commands/`
- `src/app/api.rs`
- `tests/snippets/`
