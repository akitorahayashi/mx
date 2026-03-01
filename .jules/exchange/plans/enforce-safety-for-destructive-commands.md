---
label: "bugs"
---

## Goal

Improve CLI safety by requiring explicit confirmation or override flags for destructive operations.

## Problem

Destructive CLI commands lack uniform safety contracts, increasing the risk of operational accidents. The `Clean` and `Remove` subcommands, which perform deletion, do not require a `--force` flag or equivalent explicit opt-in, unlike `Touch` and `CreateCommand` which include `--force` for overwriting. They directly delete directories or context files without dry-run or confirmation.

## Affected Areas

### CLI Modules

- `src/app/cli/mod.rs`
- `src/app/commands/clean/mod.rs`
- `src/app/commands/remove/mod.rs`

## Constraints

- Changes must not break existing non-interactive usage if `--force` is provided.

## Risks

- Breaking existing automated scripts that rely on the previous destructive behavior without explicit confirmation or the `--force` flag.

## Acceptance Criteria

- Destructive commands require explicit `--force` overrides or confirmation prompts.

## Implementation Plan

1. Update the `Commands` enum in `src/app/cli/mod.rs` to add a `--force` flag to both `Clean` and `Remove` subcommands.
2. Update the `execute` function in `src/app/commands/clean/mod.rs` to check for the `--force` flag before deleting the `.mx` directory or specified context files. If the flag is absent, prompt for confirmation or abort the operation safely.
3. Update the `execute` function in `src/app/commands/remove/mod.rs` to check for the `--force` flag before deleting snippet files. If the flag is absent, prompt for confirmation or abort the operation safely.
4. Write or update tests to ensure the presence of safety checks and correct behavior with and without the `--force` flag.
