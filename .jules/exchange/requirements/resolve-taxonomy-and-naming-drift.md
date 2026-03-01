---
label: "refacts"
scope: "Standardize domain terminology across the CLI, codebase, and documentation"
---

## Goal

Eliminate confusing terminology drift by standardizing 'Snippet' over 'Command' and clearly defining the difference between keys and aliases, as well as fixing the documented 'copy' vs 'command' command.

## Problem

The system suffers from terminology drift. 'Snippet' and 'Command' are used interchangeably across domains, CLI, and docs. 'Key' and 'Alias' are used without clear distinction. Furthermore, the CLI design introduces naming synonyms for the same conceptual action (deletion) and structural drift exists between documented commands and implementation.

## Evidence

- source_event: "snippet-vs-command-taxonomy.md"
  path: "src/domain/snippet/catalog_entry.rs"
  loc: "SnippetEntry"
  note: "Domain uses 'Snippet' as the canonical type name."

- source_event: "snippet-vs-command-taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "CreateCommand vs Add { path, title... }, Copy { snippet }"
  note: "CLI mixes 'snippet' argument names with 'create_command' action, and uses `mx command <snippet>`."

- source_event: "snippet-vs-command-taxonomy.md"
  path: "README.md"
  loc: "Storage layout: `~/.config/mx/commands/`"
  note: "Filesystem uses 'commands' to store what the domain calls 'snippets'."

- source_event: "snippet-vs-command-taxonomy.md"
  path: "README.md"
  loc: "`MX_COMMANDS_ROOT`"
  note: "Environment variables use the term 'commands'."

- source_event: "key-vs-alias-taxonomy.md"
  path: "src/app/cli/mod.rs"
  loc: "Touch { key: String }, Cat { key: String }"
  note: "CLI arguments use 'key' to refer to the context file identifier."

- source_event: "key-vs-alias-taxonomy.md"
  path: "src/domain/context_file/alias_registry.rs"
  loc: "resolve_alias(key: &str)"
  note: "The internal registry and lookup function uses the term 'alias' instead of 'key'."

- source_event: "key-vs-alias-taxonomy.md"
  path: "README.md"
  loc: "Context Management Keys (Aliases)"
  note: "Documentation conflates the two terms without establishing a canonical one."

- source_event: "naming-inconsistency-synonyms-cli-sentinel.md"
  path: "src/app/cli/mod.rs"
  loc: "Commands enum"
  note: "The CLI uses two different verbs, `Clean` and `Remove`, for file deletion actions (deleting context files vs. snippet files). This violates the principle of avoiding synonyms."

- source_event: "naming-inconsistency-synonyms-cli-sentinel.md"
  path: "README.md"
  loc: "CLI usage section"
  note: "The documentation explicitly references `mx command wc` to copy a snippet, but the CLI implementation uses `copy` (with alias `c`), leading to an 'unrecognized subcommand' error when the user types `mx command`."

- source_event: "naming-inconsistency-synonyms-cli-sentinel.md"
  path: "src/app/cli/mod.rs"
  loc: "Commands enum"
  note: "The implemented subcommand is `Copy`, which does not have a `command` alias, confirming the structural drift from the README."

- source_event: "doc-cli-copy-command-drift-consistency.md"
  path: "README.md"
  loc: "line 5 and line 32"
  note: "Documents the command as `mx command <snippet>` and `mx command wc`."

- source_event: "doc-cli-copy-command-drift-consistency.md"
  path: "src/app/cli/mod.rs"
  loc: "lines 29-30"
  note: "Defines the command as `Copy` with `visible_alias = \"c\"`."

## Change Scope

- `README.md`
- `src/app/cli/mod.rs`
- `src/domain/context_file/alias_registry.rs`
- `src/domain/snippet/catalog_entry.rs`

## Constraints

- Domain boundary and user-facing boundary should use consistent terminology ('Snippet').
- The terms 'Key' and 'Alias' must be formally distinguished or unified.

## Acceptance Criteria

- Internal domain and external CLI uniformly use 'Snippet'.
- README.md and documentation are updated to reflect the correct CLI subcommands.
- CLI synonyms for deletion (e.g., Clean vs Remove) are unified.
- The terms 'Key' and 'Alias' are clarified and uniformly applied.
