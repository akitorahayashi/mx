---
label: "bugs"
---

## Goal

Ensure diagnostic and warning messages are sent to stderr to maintain proper I/O separation.

## Problem

Diagnostic and warning messages are currently printed to stdout, which violates I/O separation principles and potentially breaks automation that relies on standard output for structured data. The warning message '⚠️ Context file already exists: ...' in `src/app/cli/touch.rs` is emitted using `println!` (stdout) instead of `eprintln!` (stderr).

## Affected Areas

### CLI Touch Command

- `src/app/cli/touch.rs`

## Constraints

- Error messages and diagnostics must not interfere with valid stdout responses expected by pipelining scripts.

## Risks

- None expected, as this standardizes I/O streams which automation typically expects.

## Acceptance Criteria

- Diagnostic and warning messages are routed to stderr consistently.

## Implementation Plan

1. Locate the `println!` macro used for the warning '⚠️ Context file already exists: ...' in `src/app/cli/touch.rs` line 8.
2. Change the `println!` macro to `eprintln!` to route the warning to stderr.
3. Review `src/app/cli/touch.rs` for any other instances of `println!` that should be routed to stderr and update them appropriately.
4. Update or write tests to verify that warnings are correctly sent to stderr.
