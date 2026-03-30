---
label: "tests"
created_at: "2024-05-24"
author_role: "cov"
confidence: "high"
---

## Problem

`FilesystemSnippetStore::from_env` logic in `src/adapters/snippet_store/filesystem_store.rs` lacks coverage for fallback resolution and error handling when detecting the command root path from the environment.

## Goal

Add targeted tests for `FilesystemSnippetStore::from_env` to verify correct `MX_COMMANDS_ROOT` resolution, backward compatibility checks (`commands` subdirectory presence), and the default `.config/mx/commands` fallback behavior.

## Context

The system discovers snippets starting from an environment-specified or user-local root directory. This logic contains conditionals for legacy setups (where an extra `commands` subfolder was implicitly required). If the resolution fails or produces incorrect paths, file reads/writes fail systemically. Adding coverage for this logic secures the filesystem resolution.

## Evidence

- path: "src/adapters/snippet_store/filesystem_store.rs"
  loc: "FilesystemSnippetStore::from_env logic"
  note: "Reading `src/adapters/snippet_store/filesystem_store.rs` reveals fallback and environment variable processing paths that require integration testing to prevent configuration breakage."

## Change Scope

- `src/adapters/snippet_store/filesystem_store.rs`