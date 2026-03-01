---
label: "refacts"
---

## Goal

Reorganize CLI commands into vertical feature slices rather than horizontally slicing them into arguments and execution logic.

## Problem

CLI commands are currently organized by horizontal layers, violating cohesion by change reason. This forces developers to navigate between parsing logic in `src/app/cli/` and execution logic in `src/app/commands/` to understand or modify a single feature.

## Affected Areas

### CLI and Application Commands

- `src/app/cli/`
- `src/app/cli/mod.rs`
- `src/app/commands/`

## Constraints

- Commands must be organized by vertical slices.
- The command implementations must be self-contained within single vertical slices where possible.

## Risks

- Extensive refactoring might break existing CLI functionality or routing.
- The merging of CLI argument definition and execution logic might introduce regressions in argument parsing behavior.

## Acceptance Criteria

- CLI command arguments and execution logic are reorganized into vertical slices in a unified directory structure (e.g., combining `src/app/cli/add.rs` and `src/app/commands/add/mod.rs`).
- The horizontal split between `src/app/cli/` and `src/app/commands/` is eliminated, and feature logic is co-located.
- The `match` statement mapping CLI variants to execution logic is updated to reflect the new vertical structure.

## Implementation Plan

1. Analyze `src/app/cli/` to map existing CLI commands (e.g., `add.rs`, `checkout.rs`) to their execution counterparts in `src/app/commands/` (e.g., `add/mod.rs`, `checkout/mod.rs`).
2. Create a unified command structure (e.g., `src/app/commands/add.rs` or `src/app/commands/add/`) that includes both the argument definition struct (from `src/app/cli/`) and the execution logic (from `src/app/commands/`).
3. Move the CLI specific structs from `src/app/cli/` into the newly created vertical slices in `src/app/commands/`.
4. Update `src/app/cli/mod.rs` to import the command structs directly from the new vertical slices and handle routing.
5. Update module declarations (`mod.rs`) in both directories to reflect the new structure, removing obsolete files from `src/app/cli/`.
6. Refactor the match statement in `src/app/cli/mod.rs` or main entry point to directly invoke the localized execution function of the parsed command struct.
7. Run tests to ensure CLI command structure and behavior remain correct.
