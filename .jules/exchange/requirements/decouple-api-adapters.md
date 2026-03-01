---
label: "refacts"
scope: "Refactor API layer to use dependency injection instead of hardcoding adapters"
---

## Goal

Improve the architectural separation of concerns by ensuring the API layer does not directly instantiate specific adapter implementations.

## Problem

The API layer inconsistently mixes adapter instantiation with dependency injection, hardcoding specific adapters like LocalContextFileStore and SymlinkCheckout.

## Evidence

- source_event: "hardcoded-adapters-api-structural-arch.md"
  path: "src/app/api.rs"
  loc: "lines 22, 27, 43"
  note: "Directly instantiates `LocalContextFileStore::new(find_workspace_root()?)`, hardwiring local filesystem assumptions."

- source_event: "hardcoded-adapters-api-structural-arch.md"
  path: "src/app/api.rs"
  loc: "line 53"
  note: "Directly instantiates `SymlinkCheckout::new()`, coupling the checkout workflow specifically to a symlink strategy instead of accepting the `SnippetCheckout` port."

## Change Scope

- `src/app/api.rs`

## Constraints

- API facade must remain infrastructure-agnostic.

## Acceptance Criteria

- API layer functions accept adapter dependencies via injection rather than hardcoded instantiation.
