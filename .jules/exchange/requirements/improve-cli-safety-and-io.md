---
label: "bugs"
scope: "Enforce safety contracts for destructive CLI commands and separate stdout/stderr"
---

## Goal

Improve CLI safety by requiring explicit confirmation or override flags for destructive operations and ensure diagnostic/warning messages are sent to stderr.

## Problem

Destructive CLI commands lack uniform safety contracts, increasing the risk of operational accidents. Additionally, diagnostic and warning messages are printed to stdout, violating I/O separation principles and potentially breaking automation.

## Evidence

- source_event: "missing-destructive-safety-cli-sentinel.md"
  path: "src/app/cli/mod.rs"
  loc: "Commands enum"
  note: "The `Clean` and `Remove` subcommands, which perform deletion, do not require a `--force` flag or equivalent explicit opt-in, unlike `Touch` and `CreateCommand` which include `--force` for overwriting."

- source_event: "missing-destructive-safety-cli-sentinel.md"
  path: "src/app/commands/clean/mod.rs"
  loc: "execute function"
  note: "Deletes the `.mx` directory or specified context files directly without dry-run or confirmation."

- source_event: "missing-destructive-safety-cli-sentinel.md"
  path: "src/app/commands/remove/mod.rs"
  loc: "execute function"
  note: "Deletes snippet files directly from the snippet store without explicit user confirmation."

- source_event: "io-separation-warning-stdout-cli-sentinel.md"
  path: "src/app/cli/touch.rs"
  loc: "line 8"
  note: "The warning message '⚠️ Context file already exists: ...' is emitted using `println!` (stdout) instead of `eprintln!` (stderr)."

## Change Scope

- `src/app/cli/mod.rs`
- `src/app/cli/touch.rs`
- `src/app/commands/clean/mod.rs`
- `src/app/commands/remove/mod.rs`

## Constraints

- Changes must not break existing non-interactive usage if --force is provided.

## Acceptance Criteria

- Destructive commands require confirmation prompts or explicit --force overrides.
- Diagnostic and warning messages are routed to stderr.
