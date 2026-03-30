---
label: "refacts"
created_at: "2024-03-30"
author_role: "data_arch"
confidence: "high"
---

## Problem

Duplicate code exists for path traversal and absolute path validation. `validate_relative_components` in `src/domain/context_file/path_policy.rs`, `ensure_safe_segments` in `src/domain/snippet/query.rs`, and `extract_relative_path` in `src/app/commands/add/mod.rs` implement nearly identical path component validation logic.

## Goal

Consolidate path validation logic into a single shared utility or enforce it within a common path type to ensure consistent security policy and reduce duplication.

## Context

Security constraints like preventing path traversals should be implemented uniformly to avoid gaps or discrepancies. Implementing it in three different locations creates a risk of diverging logic. A single shared policy or boundary validation is essential for Single Source of Truth for this validation.

## Evidence

- path: "src/domain/context_file/path_policy.rs"
  loc: "line 6"
  note: "`validate_relative_components` enforces `Component::Normal` or `Component::CurDir`."
- path: "src/domain/snippet/query.rs"
  loc: "line 22"
  note: "`ensure_safe_segments` enforces `Component::Normal` or `Component::CurDir`."
- path: "src/app/commands/add/mod.rs"
  loc: "line 22"
  note: "`extract_relative_path` enforces `Component::Normal` or `Component::CurDir` inside the loop."

## Change Scope

- `src/domain/context_file/path_policy.rs`
- `src/domain/snippet/query.rs`
- `src/app/commands/add/mod.rs`
