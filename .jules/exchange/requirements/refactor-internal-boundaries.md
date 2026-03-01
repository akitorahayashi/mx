---
label: "refacts"
scope: "Reduce public surface area and align CLI commands by vertical slices"
---

## Goal

Improve internal module organization by hiding internal architecture details and organizing CLI commands by feature slice rather than horizontal layer.

## Problem

The internal architecture is unnecessarily publicly exported. CLI commands are organized by horizontal layers, violating 'Cohesion by change reason' and forcing scattered edits.

## Evidence

- source_event: "excessive-public-surface-structural-arch.md"
  path: "src/lib.rs"
  loc: "lines 1-3"
  note: "Publicly exports `pub mod adapters;` and `pub mod domain;`, exposing the internal repository structure to potential API consumers instead of providing a controlled facade."

- source_event: "horizontal-slicing-commands-structural-arch.md"
  path: "src/app/cli/"
  loc: "Entire directory"
  note: "Contains CLI-specific logic (e.g., `add.rs`, `checkout.rs`) that is coupled to the corresponding command implementations, requiring changes here for any new feature."

- source_event: "horizontal-slicing-commands-structural-arch.md"
  path: "src/app/commands/"
  loc: "Entire directory"
  note: "Contains the execution logic for each command (e.g., `add/mod.rs`, `checkout/mod.rs`), forcing developers to navigate between `src/app/cli/` and `src/app/commands/` to understand a single feature's flow."

- source_event: "horizontal-slicing-commands-structural-arch.md"
  path: "src/app/cli/mod.rs"
  loc: "line 43"
  note: "The match statement directly maps CLI variants to the `src/app/commands/` logic, further cementing the horizontal split."

## Change Scope

- `src/app/cli/`
- `src/app/cli/mod.rs`
- `src/app/commands/`
- `src/lib.rs`

## Constraints

- Commands must be organized by vertical slices.

## Acceptance Criteria

- Internal architecture modules are no longer publicly exported from the root library file.
- CLI commands are reorganized into vertical slices (combining arg parsing and execution logic).
