---
created_at: "2026-03-01"
author_role: "qa"
confidence: "medium"
---

## Statement

The integration tests for the CLI subcommands (`add.rs`, `cat.rs`, `clean.rs`, `copy.rs`, `create_command.rs`, `remove.rs`, `touch.rs`, `checkout.rs`, `list.rs`) test multiple overlapping paths that are already unit-tested at the `app::commands` boundary. E.g., `tests/cli/copy.rs` verifies that a missing snippet prints a failure, which is also unit-tested in `src/app/commands/copy/mod.rs`. Shifting these to pure unit tests for logic, and leaving CLI tests to focus strictly on arg parsing and adapter plumbing, could improve feedback speed.

## Evidence

- path: "tests/cli/copy.rs"
  loc: "25-36"
  note: "Tests the `unknown` snippet failure, duplicating `execute_surfaces_missing_snippet_errors` in `src/app/commands/copy/mod.rs`."
- path: "tests/cli/create_command.rs"
  loc: "47-55"
  note: "Tests path boundary rejection `.mx/commands/` which is identically tested in `src/app/commands/create_command/mod.rs` at line 100."
