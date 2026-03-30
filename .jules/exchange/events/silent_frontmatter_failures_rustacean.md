---
label: "bugs"
created_at: "2024-05-18"
author_role: "rustacean"
confidence: "high"
---

## Problem

In the `list` command (`execute`), retrieving snippet metadata uses `.ok()` chained after `fs::read_to_string` and later uses `.unwrap_or((None, None))`. This silently catches and drops all IO errors and potential parse failures, treating broken or inaccessible snippet files identically to those without frontmatter.

## Goal

Eliminate the silent fallback by explicitly handling I/O and parsing errors. Distinguish between a missing frontmatter (valid state) and an unreadable file or unparseable metadata block (invalid/error state), logging or propagating the error appropriately.

## Context

Silent fallback mechanisms that collapse operational failures (IO errors, permission denied) with expected defaults ("no data present") mask underlying bugs, corrupt configurations, or permission faults. Such operations should fail predictably and loudly.

## Evidence

- path: "src/app/commands/list/mod.rs"
  loc: "18-24"
  note: "`fs::read_to_string(...).ok()` discards underlying I/O error and `unwrap_or` defaults to `(None, None)` for all failures."

## Change Scope

- `src/app/commands/list/mod.rs`