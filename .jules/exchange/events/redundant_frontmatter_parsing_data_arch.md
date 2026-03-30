---
label: "refacts"
created_at: "2024-03-30"
author_role: "data_arch"
confidence: "high"
---

## Problem

Duplicate code exists for parsing frontmatter. The `strip_frontmatter` and `parse_frontmatter` functions in `src/domain/snippet/frontmatter.rs` duplicate logic for finding frontmatter fences.

## Goal

Consolidate the frontmatter logic so the file content isn't parsed multiple times with duplicate logic, creating a unified parsed state.

## Context

Having duplicate implementations for finding frontmatter logic is error-prone, violates the DRY principle, and creates unnecessary coupling when changing the format. A single parsed state should be derived and passed along instead.

## Evidence

- path: "src/domain/snippet/frontmatter.rs"
  loc: "line 8 and 32"
  note: "`strip_frontmatter` and `parse_frontmatter` both independently implement fence logic, instead of parsing once and representing as a single entity (facts + derivations)."

## Change Scope

- `src/domain/snippet/frontmatter.rs`
