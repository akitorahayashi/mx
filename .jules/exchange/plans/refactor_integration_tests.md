---
label: "tests"
---

## Goal

Refactor integration tests in `tests/cli` and `tests/context` to exclusively verify observable CLI boundaries, eliminate redundant business logic coverage, and decouple test mechanics from arbitrary internal filesystem states.

## Current State

- `tests/cli/add.rs`: Contains duplicate verification of core domain logic (e.g., `add_fails_on_duplicate_without_force`, `add_rejects_path_outside_mx_commands`) which are already covered by unit tests in `src/app/commands/add/mod.rs`. Test assertions are tightly coupled to `fs::read_to_string` on expected output paths instead of standard `stdout`/`stderr` verification.
- `tests/cli/cat.rs`: Hardcodes `tempfile::tempdir()` and uses detailed `fs::create_dir_all` mechanics to build environments instead of relying on the existing `TestContext` harness abstractions.
- `tests/cli/clean.rs`: Couples success to low-level internal directory state checks (`mx_dir.join(".gitignore").exists()`) rather than verifying expected `stdout` output. Bypasses the test harness by directly initializing temporary directories.
- `tests/cli/touch.rs`: Same issues as `clean.rs`; avoids `TestContext`, checks `.gitignore` internals manually rather than verifying user-facing terminal outputs.
- `tests/cli/copy.rs`: Relatively clean but directly relies on `fs::read_to_string` using arbitrary clipboard contents validation rather than using standard test context assertions.
- `tests/context/lifecycle.rs`: Uses bare `tempdir()` and couples test success to internal filesystem paths like `clipboard.txt` instead of observable command pipelines or harness abstractions.

## Plan

### Refactor tests/cli/add.rs

1. Remove `add_fails_on_duplicate_without_force` and `add_rejects_path_outside_mx_commands` as they test internal domain rules identical to existing app tests.
2. Update remaining tests to verify side-effects using `TestContext` features instead of raw `fs::read_to_string` where applicable, or strengthen assertions against `stdout`/`stderr`.

### Refactor tests/cli/cat.rs

3. Replace direct `tempdir()` usage with `TestContext::new()`.
4. Remove explicit `fs::write` of test directories; use the harness to write files (e.g., through a context store mechanism if available, or simplified fixture setup).

### Refactor tests/cli/clean.rs

5. Replace direct `tempdir()` usage with `TestContext::new()`.
6. Remove internal structure assertions like `assert!(mx_dir.join(".gitignore").exists())`. Bounding tests to specific implementation details of the `.mx` directory violates boundary verification.

### Refactor tests/cli/touch.rs

7. Replace direct `tempdir()` usage with `TestContext::new()`.
8. Use `TestContext::clipboard_file()` instead of raw `fs::write(&clipboard_file)`.
9. Remove `.gitignore` existence checks. Ensure the test strictly checks observable behavior (the CLI output of "✅ Context file created" / overwritten).

### Refactor tests/cli/copy.rs

10. Remove raw string assertions reading directly from the test clipboard text file, checking outputs or behavior rather than explicitly verifying the filesystem artifact format.

### Refactor tests/context/lifecycle.rs

11. Replace direct `tempdir()` usage with `TestContext::new()`.
12. Migrate manual clipboard setup to `TestContext::clipboard_file()`.

### Cleanup unused imports

13. Remove `tempfile::tempdir` and `std::fs` imports from integration test files where they are no longer necessary.

## Acceptance Criteria

- All integration tests use `TestContext` instead of direct `tempfile::tempdir()`.
- Tests do not include structural assertions (e.g., asserting internal `.gitignore` paths).
- `tests/cli/add.rs` does not contain `add_fails_on_duplicate_without_force` or `add_rejects_path_outside_mx_commands`.
- Integration tests assert success/failure via stdout, stderr, and exit codes.
- Redundant `.unwrap()` calls are minimized in favor of harness methods.

## Risks

- Removing `add_fails_on_duplicate_without_force` could accidentally remove coverage for CLI parameter wiring.
- Adapting tests from `tempdir()` to `TestContext` may require modifications to the underlying `TestContext` if it currently lacks methods to set up specific directory structures needed by `cat` or `clean`.
