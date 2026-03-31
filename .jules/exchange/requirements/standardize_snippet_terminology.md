---
label: "refacts"
implementation_ready: false
---

## Goal

Standardize the terminology around the core managed entity (snippet vs. command) and its identifiers (snippet, path, query, key) to establish a single, unambiguous taxonomy throughout the codebase.

## Problem

The repository uses inconsistent terminology. The terms "snippet" and "command" are used interchangeably to refer to the core markdown files managed by the system. Additionally, the identifiers for these entities vary across boundaries: `snippet`, `path`, `query`, and `key` are all used to mean the same lookup string (e.g., "w/wc"). This ambiguity creates confusion, hampers readability, and reduces refactoring safety.

## Context

The system manages markdown snippets stored in `~/.config/mx/commands/` and checks them out into `.mx/commands/`. The CLI command is `mx create-command`. However, the internal domain types use `SnippetEntry`, `SnippetCatalog`, `SnippetStore`, and CLI arguments are often named `snippet`.
When locating a snippet, the string parameter's name changes depending on the context. `mx copy` uses `snippet`, `mx checkout` uses `path`, the API uses `query`, the adapter takes `raw_query`, and the context uses `key`. A unified terminology (e.g., consistently using "snippet" for the entity and "key" for the identifier) is required to clarify the domain model.

## Evidence

- path: "src/domain/snippet/catalog_entry.rs"
  loc: "struct SnippetEntry"
  note: "Domain type uses 'Snippet' but other places use 'command'."

- path: "src/app/cli/mod.rs"
  loc: "line 38, 40, 59"
  note: "`mx copy` CLI command uses parameter name `snippet`. `mx checkout` CLI command uses parameter name `path`. CLI command is `CreateCommand`."

- path: "src/app/api.rs"
  loc: "line 56"
  note: "`checkout_snippets` API method uses parameter name `query`."

- path: "src/domain/ports/snippet_catalog.rs"
  loc: "line 6"
  note: "`resolve_snippet` uses `raw_query`."

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

## Constraints

- Backward compatibility for existing snippet structures (e.g., reading from the legacy `commands/` directory if necessary) must be preserved during renaming, unless explicitly dropping legacy support. User-facing CLI flag names shouldn't change without deprecation warnings, although parameter documentation/help text should be updated.

## Acceptance Criteria

- A canonical term (either "snippet" or "command") is consistently used across all code, directories, and documentation.
- The term for identifying a snippet (e.g., "key") is standardized across CLI arguments, API boundaries, and internal methods (replacing `snippet`, `path`, `query`, and `raw_query`).