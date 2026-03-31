---
label: "refacts"
---

## Goal

Standardize terminology across the codebase by adopting "snippet" as the canonical entity and "key" as its identifier, replacing conflicting or confusing terms like "command", "path", "query", and "raw_query" while maintaining existing directory behavior (`~/.config/mx/commands`).

## Current State

The repository uses inconsistent terminology. "Snippet" and "command" are used interchangeably to refer to the core markdown files. Identifiers for these entities vary across boundaries: `snippet`, `path`, `query`, and `key` are all used to mean the same lookup string (e.g., "w/wc"). This ambiguity creates confusion, hampers readability, and reduces refactoring safety.

- `README.md`: Refers to "Snippet command" and mixes "snippet" and "command".
- `docs/cli-usage.md`: References "snippet command" but sometimes uses "snippet" and "command" interchangeably.
- `src/domain/snippet/catalog_entry.rs`: Defines `SnippetEntry` but the type is used interchangeably with 'command' conceptually elsewhere.
- `src/domain/snippet/query.rs`: Uses `normalize_query` instead of key-oriented naming.
- `src/domain/ports/snippet_catalog.rs`: `resolve_snippet` uses `raw_query` parameter, which is inconsistent with `key`.
- `src/app/api.rs`: Function signatures use a mix of `query`, `path`, and `snippet` instead of a standardized `key` (e.g. `checkout_snippets` uses `query`).
- `src/app/cli/mod.rs`: `mx copy` CLI uses `snippet`, `mx checkout` CLI uses `path`, and there is a `CreateCommand` CLI variant.
- `src/app/commands/create_command/mod.rs`: Module and variable names use `command` instead of `snippet`.
- `tests/snippets/`: Tests use variable names with a mix of terms.

## Plan

1. Rename domain terminology and parameters:
   - Update `src/domain/ports/snippet_catalog.rs` to use `key` instead of `raw_query`.
   - Update `src/domain/snippet/query.rs` to rename `normalize_query` to `normalize_key`.
   - Update `src/app/api.rs` to use `key` instead of `query`, `snippet`, `path`, or `raw_query` for snippet lookups and creations.
2. Rename CLI arguments and structs:
   - In `src/app/cli/mod.rs`, change `snippet`, `path`, and `query` argument names to `key`.
   - Rename `CreateCommand` CLI command to `CreateSnippet` (preserving visible alias `cc` if needed).
   - Update help text in CLI to use "snippet" consistently instead of "command" (except when referring to `mx command` alias or legacy folders).
3. Refactor Internal Commands & Implementations:
   - Rename module/directory `src/app/commands/create_command/` to `src/app/commands/create_snippet/` and update `src/app/commands/mod.rs`.
   - Rename `src/assets/command_template.md` to `src/assets/snippet_template.md` and update its usage in code.
   - Update `src/adapters/snippet_catalog/filesystem_catalog.rs` and other adapters to match the new trait parameter names (`key`).
   - Update tests in `src/testing/ports/` and across the codebase to use `key` in variables and assertions.
4. Update Documentation:
   - Update `README.md` and `docs/cli-usage.md` to use "snippet" consistently instead of "command" when describing the entity, and "key" when describing the identifier. Preserve legacy commands/ path instructions as-is, just update narrative wording.

## Acceptance Criteria

- All internal logic, API signatures, and CLI parameters use `key` instead of `path`, `query`, `raw_query`, or `snippet_name` when referring to a snippet's identifier.
- The `create_command` module, asset template, and CLI struct are renamed to use `snippet` (`create_snippet`, `snippet_template.md`, `CreateSnippet`).
- Codebase no longer uses the term "command" to refer to a snippet, except for legacy directory names (`~/.config/mx/commands/`) or command-line aliases (`mx command`).
- All tests pass and verify the expected behavior.

## Risks

- Changing CLI parameter names might break scripts relying on specific clap argument names if they use `--path` or similar.
- Renaming the `create_command` command to `create_snippet` might surprise users, but command aliases can mitigate this.
