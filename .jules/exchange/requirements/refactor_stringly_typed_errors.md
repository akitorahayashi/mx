---
label: "refacts"
implementation_ready: false
---

## Goal

Redesign the error contract in `AppError` to replace generic "stringly-typed" variants with strongly typed variants that preserve domain meaning, semantic classification, and structured context.

## Problem

`AppError` heavily relies on `String` variants (e.g., `ConfigError(String)`, `NotFound(String)`, `ClipboardError(String)`, `InvalidKey(String)`, `PathTraversal(String)`). This collapses underlying typed errors (like parsing, IO, configuration issues) into untyped strings, losing context and semantic classification across boundaries.

## Context

Using string-based errors (often referred to as "stringly-typed" errors) is an anti-pattern in Rust. It forces consumers of the API (even internal ones) to rely on string matching if they need to differentiate failures, making the error contract fragile and difficult to diagnose programmatically. Proper typed variants should include the specific operation, ID, or input that failed, either via native Rust enums or `thiserror`/`anyhow` crates.

## Evidence

- path: "src/domain/error.rs"
  loc: "4-20"
  note: "Definition of `AppError` shows heavy reliance on `String` variants instead of strongly typed context."

## Change Scope

- `src/domain/error.rs`
- `src/domain/snippet/query.rs`
- `src/adapters/snippet_catalog/filesystem_catalog.rs`
- `src/app/commands/add/mod.rs`
- `src/adapters/clipboard/system_clipboard.rs`

## Constraints

- Ensure the error contract clearly delineates user-facing actionable errors from systemic/environmental faults.

## Acceptance Criteria

- `AppError` definition no longer utilizes plain `String` types for error metadata. Instead, strongly-typed domain structs or enums accurately convey structured context.
- Error creation across adapters and domain models (e.g., in `system_clipboard`, `add` command) uses the new, correctly typed `AppError` variants.