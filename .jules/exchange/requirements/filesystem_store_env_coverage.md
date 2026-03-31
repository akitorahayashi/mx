---
label: "tests"
implementation_ready: false
---

## Goal

Add targeted tests for `FilesystemSnippetStore::from_env` to verify correct `MX_COMMANDS_ROOT` resolution, backward compatibility checks (`commands` subdirectory presence), and the default `.config/mx/commands` fallback behavior.

## Problem

`FilesystemSnippetStore::from_env` logic in `src/adapters/snippet_store/filesystem_store.rs` lacks coverage for fallback resolution and error handling when detecting the command root path from the environment.

## Context

The system discovers snippets starting from an environment-specified or user-local root directory. This logic contains conditionals for legacy setups (where an extra `commands` subfolder was implicitly required). If the resolution fails or produces incorrect paths, file reads/writes fail systemically. Adding coverage for this logic secures the filesystem resolution.

## Evidence

- path: "src/adapters/snippet_store/filesystem_store.rs"
  loc: "FilesystemSnippetStore::from_env logic"
  note: "Reading `src/adapters/snippet_store/filesystem_store.rs` reveals fallback and environment variable processing paths that require integration testing to prevent configuration breakage."

## Change Scope

- `src/adapters/snippet_store/filesystem_store.rs`

## Constraints

- Test setup must not interfere with real user configuration environments if run locally. Tests should override environment variables effectively within a controlled scope.

## Acceptance Criteria

- Tests exist to verify the default `.config/mx/commands` path resolution.
- Tests verify the `MX_COMMANDS_ROOT` resolution behavior.
- Tests verify the presence checking for the legacy `commands` subdirectory handling.