---
label: "tests"
implementation_ready: true
---

## Goal

Add comprehensive test coverage for clipboard adapters (`FileClipboard` and `SystemClipboard`), focusing on error paths, OS configurations, and dependency handling.

## Problem

Both `FileClipboard` and `SystemClipboard` adapters lack sufficient test coverage. `FileClipboard` is missing coverage for error paths (e.g., IO failures mapped to `AppError::clipboard_error`). `SystemClipboard` lacks coverage for different OS configurations and missing dependencies, leaving critical external system integration boundaries untested and prone to regressions.

## Context

The `FileClipboard` adapter implements the `Clipboard` port using a file on disk. While the happy path is covered, error handling when reading from or writing to the file fails is untested.
The `SystemClipboard` component detects and shells out to OS-specific tools (pbcopy, xclip, wl-copy, etc.) for reading and writing clipboard contents. Currently, coverage is absent for this complex conditional compilation and process-spawning logic.

## Evidence

- path: "src/adapters/clipboard/file_clipboard.rs"
  loc: "lines 27-28"
  note: "Untested error handler when mapping `fs::read_to_string` result and unhandled failure modes."

- path: "src/adapters/clipboard/system_clipboard.rs"
  loc: "SystemClipboard implementation block"
  note: "Complex conditional compilation and shell executions lack corresponding tests."

## Change Scope

- `src/adapters/clipboard/file_clipboard.rs`
- `src/adapters/clipboard/system_clipboard.rs`

## Constraints

- Tests must not alter the user's actual system clipboard where possible, or they should be carefully isolated.

## Acceptance Criteria

- `FileClipboard` tests include scenarios for unwritable or unreadable files, ensuring correct `AppError::clipboard_error` propagation.
- `SystemClipboard` tests verify behavior across different supported OS configurations or adequately mock the shell execution to verify the logic.
- `SystemClipboard` tests handle and verify behavior when underlying clipboard CLI tools are missing.