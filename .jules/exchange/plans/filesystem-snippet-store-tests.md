---
label: "tests"
---

## Goal

Add test coverage to `FilesystemSnippetStore`.

## Problem

Coverage reports indicate that `FilesystemSnippetStore` has 31/44 lines covered. This suggests that error scenarios or specific branches, particularly related to directory cleanup up to `commands_root` upon snippet removal or handling missing environment variables, might lack test validation.

## Affected Areas

### FilesystemSnippetStore

- `src/adapters/snippet_store/filesystem_store.rs`

## Constraints

- Cover boundary cases (e.g. empty directories, missing env vars).
- Tests should not rely on manual interaction.

## Risks

- Clean up paths upon snippet removal might fail without warning, leaving artifacts in directories up to `commands_root`.

## Acceptance Criteria

- `FilesystemSnippetStore` persistence and removal operations have test coverage.

## Implementation Plan

1. Write tests for directory cleanup up to `commands_root` upon snippet removal.
2. Write tests for handling missing environment variables scenarios.
