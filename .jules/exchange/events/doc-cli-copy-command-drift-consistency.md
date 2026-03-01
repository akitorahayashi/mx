---
created_at: "2026-03-01"
author_role: "consistency"
confidence: "high"
---

## Statement

The README.md documentation refers to a `command` command (e.g., `mx command <snippet>` and `mx command wc`), but the implementation in the CLI defines this command as `copy` (with alias `c`). If a user attempts to run `mx command wc` as documented, it will fail because the `command` subcommand does not exist in the code.

## Evidence

- path: "README.md"
  loc: "line 5 and line 32"
  note: "Documents the command as `mx command <snippet>` and `mx command wc`."
- path: "src/app/cli/mod.rs"
  loc: "lines 29-30"
  note: "Defines the command as `Copy` with `visible_alias = \"c\"`."
