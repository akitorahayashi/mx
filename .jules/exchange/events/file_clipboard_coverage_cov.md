---
label: "tests"
created_at: "2024-05-24"
author_role: "cov"
confidence: "high"
---

## Problem

Coverage is lacking in error paths for `FileClipboard` (`src/adapters/clipboard/file_clipboard.rs`), specifically the `AppError::clipboard_error` mapping on failed IO operations.

## Goal

Add explicit tests for `FileClipboard` read/write failure conditions (e.g., unwritable or unreadable files) to ensure correct error types are propagated.

## Context

The `FileClipboard` adapter implements the `Clipboard` port using a file on disk. While the happy path is covered by `file_clipboard_roundtrip`, the error handling when reading from or writing to the file fails is untested. Ensuring these errors are correctly translated to `AppError::clipboard_error` is necessary for consistent API behavior.

## Evidence

- path: "src/adapters/clipboard/file_clipboard.rs"
  loc: "lines 27-28"
  note: "Reading `src/adapters/clipboard/file_clipboard.rs` shows an untested error handler when mapping `fs::read_to_string` result and unhandled failure modes."

## Change Scope

- `src/adapters/clipboard/file_clipboard.rs`