---
label: "refacts"
---

## Goal

Fix the structural drift between documented commands (`command`) and the actual implementation (`copy`) in the CLI.

## Problem

The documentation explicitly references commands that do not map to the CLI implementation, causing an "unrecognized subcommand" error. For example:
- The `README.md` references `mx command <snippet>` and `mx command wc` (lines 5 and 32).
- The actual implementation in `src/app/cli/mod.rs` (lines 29-30) defines the command as `Copy` with a `visible_alias = "c"`.
- The implemented subcommand `Copy` does not have a `command` alias, meaning the documentation is structurally drifting from the implementation.

## Affected Areas

### CLI and App

- `src/app/cli/mod.rs`

### Documentation

- `README.md`

## Constraints

- The documentation must strictly reflect the actual implementation without users running into unrecognized subcommand errors.
- Based on the principles, "The documentation must conform to the implementation, and the implementation must not be modified to conform to the documentation."

## Risks

- Documentation changes might require users to re-learn commands if they previously relied on the broken documented commands (which wouldn't have worked anyway).

## Acceptance Criteria

- `README.md` correctly references `mx copy <snippet>` (or the appropriate snippet term after standardization) instead of `mx command <snippet>`.
- The alias `c` is properly documented for the `copy` subcommand.

## Implementation Plan

1. Update `README.md` to replace all references to `mx command` with `mx copy` (or the equivalent command mapped in `src/app/cli/mod.rs`).
2. Update the `README.md` usage section to highlight the correct `Copy` command and its alias `c`.
3. Verify no other commands in `src/app/cli/mod.rs` have similar structural drift with the documentation.
