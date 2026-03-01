---
label: "refacts"
---

## Goal

Standardize the term 'Snippet' over 'Command' across the CLI, codebase, and documentation to eliminate confusing terminology drift.

## Problem

The system suffers from terminology drift where 'Snippet' and 'Command' are used interchangeably across domains, CLI, and docs. For instance:
- Domain uses 'Snippet' as the canonical type name (`SnippetEntry` in `src/domain/snippet/catalog_entry.rs`).
- CLI mixes 'snippet' argument names with actions like `create_command` and subcommands like `mx command <snippet>`.
- The storage layout uses `~/.config/mx/commands/` while referring to what the domain calls 'snippets'.
- Environment variables use the term 'commands' (e.g., `MX_COMMANDS_ROOT`).

## Affected Areas

### CLI and App

- `src/app/cli/mod.rs`

### Domain

- `src/domain/snippet/catalog_entry.rs`

### Documentation

- `README.md`

## Constraints

- Domain boundary and user-facing boundary should uniformly use the terminology 'Snippet'.
- Environment variables and storage layout paths should ideally be transitioned to 'Snippet', or a fallback/migration provided if they are strictly required.

## Risks

- Changing environment variable names (e.g., `MX_COMMANDS_ROOT` to `MX_SNIPPETS_ROOT`) might break existing user setups or CI pipelines if not migrated properly.
- Changing storage layout (`~/.config/mx/commands/` to `~/.config/mx/snippets/`) might lose user data if not properly migrated.

## Acceptance Criteria

- Internal domain explicitly and uniformly uses 'Snippet'.
- External CLI uses 'Snippet' instead of 'Command' for subcommands and arguments.
- Documentation (`README.md`) refers to 'Snippets' correctly and uniformly.
- Storage layout and environment variables correctly refer to 'Snippets' or a migration plan is in place to support the new terminology.

## Implementation Plan

1. Update `src/app/cli/mod.rs` to replace all CLI `Command` references, argument names, and subcommands with `Snippet`.
2. Update `src/domain/snippet/catalog_entry.rs` to ensure any lingering references to 'Command' are replaced with 'Snippet'.
3. Refactor environment variables from `MX_COMMANDS_ROOT` to `MX_SNIPPETS_ROOT` (with backward compatibility if necessary).
4. Refactor storage layout path from `~/.config/mx/commands/` to `~/.config/mx/snippets/` and implement a migration routine if needed.
5. Update `README.md` to consistently refer to 'Snippets' instead of 'Commands' across all sections, storage layout descriptions, and CLI usage.
