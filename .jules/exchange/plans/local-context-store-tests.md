---
label: "tests"
---

## Goal

Improve test coverage for LocalContextFileStore.

## Problem

`read_context_contents` error handling and cleanup paths lack coverage. Specifically, paths dealing with `read_context_contents` error handling, `remove_context_root` successful and failed cleanups, and `remove_context_file` directory climbing are missing coverage. These are important side-effects that determine whether `.mx/` state accurately tracks the snippet system or leaves dangling artifacts or incorrect reads.

## Affected Areas

### LocalContextFileStore

- `src/adapters/context_file_store/local_context_store.rs`

## Constraints

- Cover boundary cases like empty/non-existent files.
- Tests should not rely on manual interaction.

## Risks

- State corruption if `.mx/` state doesn't accurately track snippet system or leaves dangling artifacts or incorrect reads.

## Acceptance Criteria

- Critical file operations and cleanup paths are tested.

## Implementation Plan

1. Write tests for `read_context_contents` error handling (e.g. non-existent files or permission errors).
2. Write tests for `remove_context_root` to verify successful cleanups.
3. Write tests for `remove_context_root` to verify failed cleanup handling.
4. Write tests for `remove_context_file` directory climbing.
