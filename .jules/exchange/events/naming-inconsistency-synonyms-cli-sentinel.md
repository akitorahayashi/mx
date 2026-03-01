---
created_at: "2025-03-01"
author_role: "cli_sentinel"
confidence: "high"
---

## Statement

The CLI design introduces naming synonyms for the same conceptual action (deletion) and exhibits structural drift between documented commands and implementation.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "Commands enum"
  note: "The CLI uses two different verbs, `Clean` and `Remove`, for file deletion actions (deleting context files vs. snippet files). This violates the principle of avoiding synonyms."
- path: "README.md"
  loc: "CLI usage section"
  note: "The documentation explicitly references `mx command wc` to copy a snippet, but the CLI implementation uses `copy` (with alias `c`), leading to an 'unrecognized subcommand' error when the user types `mx command`."
- path: "src/app/cli/mod.rs"
  loc: "Commands enum"
  note: "The implemented subcommand is `Copy`, which does not have a `command` alias, confirming the structural drift from the README."