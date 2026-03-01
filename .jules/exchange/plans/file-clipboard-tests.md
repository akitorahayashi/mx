---
label: "tests"
---

## Goal

Add test coverage for `FileClipboard` boundary cases.

## Problem

Only a single test `file_clipboard_roundtrip` exists. Lacks assertions for reading an uninitialized file or writing to an invalid path.

## Affected Areas

### FileClipboard

- `src/adapters/clipboard/file_clipboard.rs`

## Constraints

- Tests must cover empty and non-existent file states.
- Tests should not rely on manual interaction.

## Risks

- Failures or panic conditions might occur in production when encountering missing or empty clipboard files.

## Acceptance Criteria

- `FileClipboard` test verifies behavior when pasting from an empty or non-existent file.

## Implementation Plan

1. Write tests for reading from an empty or non-existent file path.
2. Write tests for writing to an invalid file path.
