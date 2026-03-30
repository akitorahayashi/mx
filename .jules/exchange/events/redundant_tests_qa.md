---
label: "tests"
created_at: "2024-03-30"
author_role: "qa"
confidence: "medium"
---

## Problem

Duplicate test configurations exist within `src/app/commands` app tests and `tests/cli` CLI integration tests.

## Goal

Ensure `tests/cli` only exercises the CLI boundaries (args parsing, process return codes, and system boundaries) while the exhaustive business logic paths are tested in the lightweight in-memory `src/app/commands/*/mod.rs` tests.

## Context

For commands like `add`, `touch`, `copy` and `list`, the application tests cover detailed validation paths and side effects, but the integration tests (`tests/cli/add.rs`, etc.) often replicate similar checks (e.g. `add_fails_on_duplicate_without_force`, `add_rejects_path_outside_mx_commands` in `tests/cli/add.rs` replicate checks that should exist solely at the unit level).

## Evidence

- path: "tests/cli/add.rs"
  loc: "55-63"
  note: "Tests duplicate failure condition (`add_fails_on_duplicate_without_force`) that is purely domain logic and best verified in `src/app/commands/add/mod.rs`."

- path: "tests/cli/add.rs"
  loc: "80-88"
  note: "Tests reject condition (`add_rejects_path_outside_mx_commands`) duplicated between integration and unit testing."

## Change Scope

- `tests/cli/add.rs`
- `tests/cli/touch.rs`
- `tests/cli/copy.rs`
