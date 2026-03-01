---
created_at: "2024-05-24"
author_role: "taxonomy"
confidence: "high"
---

## Statement

The domain concept of a reusable text fragment is referred to inconsistently across the system's boundaries. Internally, the domain and types consistently use "Snippet" (e.g., `SnippetEntry`, `SnippetCatalog`, `SnippetStore`, `add_snippet`). Externally, the CLI, environment variables, and filesystem layout use "Command" (e.g., `mx command <snippet>`, `create-command`, `MX_COMMANDS_ROOT`, `~/.config/mx/commands/`). This dual vocabulary creates confusion between the domain boundary and user-facing boundary, and violates the "One Concept, One Preferred Term" and "Naming shape consistency across boundaries" principles.

## Evidence

- path: "src/domain/snippet/catalog_entry.rs"
  loc: "SnippetEntry"
  note: "Domain uses 'Snippet' as the canonical type name."
- path: "src/app/cli/mod.rs"
  loc: "CreateCommand vs Add { path, title... }, Copy { snippet }"
  note: "CLI mixes 'snippet' argument names with 'create_command' action, and uses `mx command <snippet>`."
- path: "README.md"
  loc: "Storage layout: `~/.config/mx/commands/`"
  note: "Filesystem uses 'commands' to store what the domain calls 'snippets'."
- path: "README.md"
  loc: "`MX_COMMANDS_ROOT`"
  note: "Environment variables use the term 'commands'."
