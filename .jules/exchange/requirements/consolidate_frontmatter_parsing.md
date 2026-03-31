---
label: "refacts"
implementation_ready: false
---

## Goal

Consolidate the frontmatter logic so the file content isn't parsed multiple times with duplicate logic, creating a unified parsed state.

## Problem

Duplicate code exists for parsing frontmatter. The `strip_frontmatter` and `parse_frontmatter` functions in `src/domain/snippet/frontmatter.rs` duplicate logic for finding frontmatter fences.

## Context

Having duplicate implementations for finding frontmatter logic is error-prone, violates the DRY principle, and creates unnecessary coupling when changing the format. A single parsed state should be derived and passed along instead.

## Evidence

- path: "src/domain/snippet/frontmatter.rs"
  loc: "line 8 and 32"
  note: "`strip_frontmatter` and `parse_frontmatter` both independently implement fence logic, instead of parsing once and representing as a single entity (facts + derivations)."

## Change Scope

- `src/domain/snippet/frontmatter.rs`

## Constraints

- Existing API signatures dependent on these functions should be updated to accept the new unified parsed state where applicable, or the original functions can act as thin wrappers over the unified logic if immediate signature changes are too disruptive.

## Acceptance Criteria

- `strip_frontmatter` and `parse_frontmatter` logic is consolidated to reuse the same fence detection mechanism.
- The file content is parsed only once for both pieces of data.