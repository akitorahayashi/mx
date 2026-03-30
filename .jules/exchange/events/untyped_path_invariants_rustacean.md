---
label: "refacts"
created_at: "2024-05-18"
author_role: "rustacean"
confidence: "high"
---

## Problem

Functions that perform domain invariant checks (like `validate_path` or `normalize_query`) perform verification but return standard `PathBuf` or `String`. The type system does not statically guarantee that a path has been validated, leading to repeated checks or implicit trust across module boundaries.

## Goal

Create strong domain types (e.g., `ValidatedContextPath` or `SafeSnippetPath`) that encapsulate the validity invariant. Operations requiring a safe path should demand this type at their API boundary, making invalid state unrepresentable.

## Context

Relying on out-of-band functions like `validate_path` before passing raw `String` or `PathBuf` variables spreads validation logic and requires developers to remember to call it. Encapsulating validated data in a custom type moves the check to the type boundary.

## Evidence

- path: "src/domain/context_file/path_policy.rs"
  loc: "14-20"
  note: "`validate_path` performs the invariant check but returns `Result<(), AppError>`, leaving the underlying path type uncertified."
- path: "src/domain/snippet/query.rs"
  loc: "4-18"
  note: "`normalize_query` enforces paths are clean but returns an untyped `String`."

## Change Scope

- `src/domain/context_file/path_policy.rs`
- `src/domain/context_file/key.rs`
- `src/domain/snippet/query.rs`
- `src/app/commands/touch/mod.rs`