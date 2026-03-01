---
label: "tests"
---

## Goal

Improve testing feedback speed by shifting overlapping domain logic tests out of slow CLI integration tests into fast unit tests, leaving CLI tests strictly for arg parsing.

## Problem

CLI integration tests test multiple overlapping paths that are already unit-tested at the app::commands boundary, resulting in redundant slow tests. Specifically, `tests/cli/copy.rs` duplicates failure logic and `tests/cli/create_command.rs` duplicates boundary rejection logic.

## Affected Areas

### Tests

- `tests/cli/copy.rs`
- `tests/cli/create_command.rs`

## Constraints

- CLI integration tests should only focus on arg parsing and adapter plumbing.
- Unit tests at the app boundary must encompass the removed CLI integration test assertions.

## Risks

- Removing CLI tests might decrease overall end-to-end integration confidence if the app boundary unit tests do not adequately simulate the tested scenarios.

## Acceptance Criteria

- Overlapping domain logic tests are shifted from CLI integration tests to unit tests.
- CLI integration tests are focused purely on argument parsing and plumbing.

## Implementation Plan

1. Review the redundant test in `tests/cli/copy.rs` (lines 25-36) which tests the `unknown` snippet failure.
2. Ensure `execute_surfaces_missing_snippet_errors` in `src/app/commands/copy/mod.rs` sufficiently covers this case. Enhance the unit test if needed.
3. Delete the duplicate CLI test from `tests/cli/copy.rs`.
4. Review the redundant test in `tests/cli/create_command.rs` (lines 47-55) which tests path boundary rejection `.mx/commands/`.
5. Ensure it is covered in `src/app/commands/create_command/mod.rs` at line 100. Enhance the unit test if needed.
6. Delete the duplicate CLI test from `tests/cli/create_command.rs`.
7. Verify remaining CLI tests pass and strictly focus on arg parsing/plumbing.
