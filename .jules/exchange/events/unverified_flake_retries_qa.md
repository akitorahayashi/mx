---
label: "tests"
created_at: "2024-03-30"
author_role: "qa"
confidence: "medium"
---

## Problem

Integration tests rely on hardcoded paths inside temporary directories via `tempfile::tempdir().unwrap()` combined with hardcoded strings rather than robust assertions of output structure, potentially leading to flaky assertions if the environment changes. Test files use `fs::read_to_string` to assert on test success.

## Goal

Ensure integration tests assert on externally observable behavior (stdout/stderr or correctly abstracted context reads) rather than deeply coupling to implementation details (like exact filesystem states using real I/O).

## Context

Many test files (e.g., `tests/cli/cat.rs`, `tests/cli/touch.rs`, `tests/cli/clean.rs`, `tests/context/lifecycle.rs`) create temporary directories and hardcode assertions that files exist in a specific structure, relying heavily on `.unwrap()`. This couples the tests tightly to the file system instead of relying on the abstractions created.

## Evidence

- path: "tests/cli/cat.rs"
  loc: "8-16"
  note: "Uses `tempfile::tempdir().unwrap()` and hardcoded strings for paths and file contents to setup test scenarios instead of relying on harness abstractions."

- path: "tests/context/lifecycle.rs"
  loc: "8-29"
  note: "Ties test success to arbitrary filesystem structure checks like `dir.path().join(\"clipboard.txt\")`."

## Change Scope

- `tests/cli/cat.rs`
- `tests/cli/touch.rs`
- `tests/cli/clean.rs`
- `tests/context/lifecycle.rs`
