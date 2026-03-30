---
label: "refacts"
created_at: "2024-05-18"
author_role: "rustacean"
confidence: "high"
---

## Problem

`AppError` heavily relies on `String` variants (e.g., `ConfigError(String)`, `NotFound(String)`, `ClipboardError(String)`, `InvalidKey(String)`, `PathTraversal(String)`). This collapses underlying typed errors (like parsing, IO, configuration issues) into untyped strings, losing context and semantic classification across boundaries.

## Goal

Redesign the error contract to use typed variants that preserve domain meaning, semantic classification, and structured context (e.g., the specific operation, ID, or input that failed). Replace generic strings with precise types or dynamic context attachment.

## Context

Using string-based errors (often referred to as "stringly-typed" errors) is an anti-pattern in Rust. It forces consumers of the API (even internal ones) to rely on string matching if they need to differentiate failures, making the error contract fragile and difficult to diagnose programmatically.

## Evidence

- path: "src/domain/error.rs"
  loc: "4-20"
  note: "Definition of AppError shows heavy reliance on String variants instead of strongly typed context."

## Change Scope

- `src/domain/error.rs`
- `src/domain/snippet/query.rs`
- `src/adapters/snippet_catalog/filesystem_catalog.rs`
- `src/app/commands/add/mod.rs`
- `src/adapters/clipboard/system_clipboard.rs`