---
label: "tests"
scope: "Shift repetitive CLI integration tests to unit tests at the app boundary"
---

## Goal

Improve testing feedback speed by shifting overlapping domain logic tests out of slow CLI integration tests into fast unit tests, leaving CLI tests strictly for arg parsing.

## Problem

CLI integration tests test multiple overlapping paths that are already unit-tested at the app::commands boundary, resulting in redundant slow tests.

## Evidence

- source_event: "cli-integration-tests-redundancy-qa.md"
  path: "tests/cli/copy.rs"
  loc: "25-36"
  note: "Tests the `unknown` snippet failure, duplicating `execute_surfaces_missing_snippet_errors` in `src/app/commands/copy/mod.rs`."

- source_event: "cli-integration-tests-redundancy-qa.md"
  path: "tests/cli/create_command.rs"
  loc: "47-55"
  note: "Tests path boundary rejection `.mx/commands/` which is identically tested in `src/app/commands/create_command/mod.rs` at line 100."

## Change Scope

- `tests/cli/copy.rs`
- `tests/cli/create_command.rs`

## Constraints

- CLI integration tests should only focus on arg parsing and adapter plumbing.

## Acceptance Criteria

- Overlapping domain logic tests are shifted from CLI integration tests to unit tests.
- CLI integration tests are focused purely on argument parsing and plumbing.
