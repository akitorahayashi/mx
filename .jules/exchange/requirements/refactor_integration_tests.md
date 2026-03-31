---
label: "tests"
implementation_ready: false
---

## Goal

Refactor integration tests in `tests/cli` to exclusively cover CLI boundaries, remove redundant business logic testing duplicated in `src/app/commands`, and decouple assertions from fragile, hardcoded filesystem states.

## Problem

Duplicate test configurations exist within `src/app/commands` app tests and `tests/cli` integration tests. Specifically, domain logic validations are repeatedly tested at the integration boundary, unnecessarily slowing down the test suite and blurring test scope. Additionally, integration tests rely heavily on `.unwrap()` and hardcoded temporary paths via `tempfile::tempdir()`, making assertions brittle and overly coupled to exact filesystem states rather than observable outputs (stdout/stderr).

## Context

For commands like `add`, `touch`, `copy`, and `list`, the application tests cover detailed validation paths and side effects. The integration tests (e.g., `tests/cli/add.rs`) should not duplicate these checks but instead ensure arguments map correctly and system calls return appropriate codes. Furthermore, files like `tests/cli/cat.rs` create temporary directories and hardcode assertions that files exist in a specific structure, tightly coupling tests to the filesystem. Assertions should target externally observable behavior.

## Evidence

- path: "tests/cli/add.rs"
  loc: "line 55-63, 80-88"
  note: "Tests duplicate failure conditions (`add_fails_on_duplicate_without_force`, `add_rejects_path_outside_mx_commands`) best verified in unit tests."

- path: "tests/cli/cat.rs"
  loc: "line 8-16"
  note: "Uses `tempfile::tempdir().unwrap()` and hardcoded strings for paths and file contents instead of relying on harness abstractions."

- path: "tests/context/lifecycle.rs"
  loc: "line 8-29"
  note: "Ties test success to arbitrary filesystem structure checks like `dir.path().join(\"clipboard.txt\")`."

## Change Scope

- `tests/cli/add.rs`
- `tests/cli/copy.rs`
- `tests/cli/cat.rs`
- `tests/cli/touch.rs`
- `tests/cli/clean.rs`
- `tests/context/lifecycle.rs`

## Constraints

- Refactoring should not inadvertently drop coverage of actual logic bugs; pure unit tests must adequately absorb any removed business logic scenarios from integration tests.

## Acceptance Criteria

- Integration tests in `tests/cli` no longer verify internal domain validation logic (like duplicate paths) if unit tests cover them.
- File system checks (`fs::read_to_string`, `tempdir()`) in integration tests are replaced with observable output assertions (e.g., stdout) or abstract harness methods where possible.
- `.unwrap()` statements are minimized in favor of robust error handling within tests.