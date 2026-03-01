---
created_at: "2025-03-01"
author_role: "cli_sentinel"
confidence: "high"
---

## Statement

Destructive CLI commands lack uniform safety contracts, such as confirmation prompts or explicit override flags (e.g., `--force`), increasing the risk of operational accidents.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "Commands enum"
  note: "The `Clean` and `Remove` subcommands, which perform deletion, do not require a `--force` flag or equivalent explicit opt-in, unlike `Touch` and `CreateCommand` which include `--force` for overwriting."
- path: "src/app/commands/clean/mod.rs"
  loc: "execute function"
  note: "Deletes the `.mx` directory or specified context files directly without dry-run or confirmation."
- path: "src/app/commands/remove/mod.rs"
  loc: "execute function"
  note: "Deletes snippet files directly from the snippet store without explicit user confirmation."