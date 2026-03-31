---
label: "bugs"
implementation_ready: false
---

## Goal

Eliminate the silent fallback by explicitly handling I/O and parsing errors. Distinguish between a missing frontmatter (valid state) and an unreadable file or unparseable metadata block (invalid/error state), logging or propagating the error appropriately.

## Problem

In the `list` command (`execute`), retrieving snippet metadata uses `.ok()` chained after `fs::read_to_string` and later uses `.unwrap_or((None, None))`. This silently catches and drops all IO errors and potential parse failures, treating broken or inaccessible snippet files identically to those without frontmatter.

## Context

Silent fallback mechanisms that collapse operational failures (IO errors, permission denied) with expected defaults ("no data present") mask underlying bugs, corrupt configurations, or permission faults. Such operations should fail predictably and loudly.

## Evidence

- path: "src/app/commands/list/mod.rs"
  loc: "18-24"
  note: "`fs::read_to_string(...).ok()` discards underlying I/O error and `unwrap_or` defaults to `(None, None)` for all failures."

## Change Scope

- `src/app/commands/list/mod.rs`

## Constraints

- Ensure any explicitly surfaced error types integrate with the existing `AppError` mechanisms or establish a clear logging strategy if propagation is inappropriate for a bulk listing operation.

## Acceptance Criteria

- IO errors during file reading in the `list` command are no longer silently masked.
- Parse failures are similarly detected and handled.
- Valid snippets missing frontmatter are still processed correctly without triggering errors.