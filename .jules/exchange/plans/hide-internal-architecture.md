---
label: "refacts"
---

## Goal

Hide internal architecture details from the public API by removing public exports of internal modules.

## Problem

The internal architecture is unnecessarily publicly exported in the root library file, exposing the internal repository structure to potential API consumers instead of providing a controlled facade.

## Affected Areas

### Core Library

- `src/lib.rs`

## Constraints

- Internal architecture modules must not be publicly exported.

## Risks

- External consumers or other internal components relying on the public visibility of these modules might break if they depend on the exposed structure.
- Changes might inadvertently hide functionality that should be part of the public API.

## Acceptance Criteria

- Internal architecture modules (e.g., adapters, domain) are no longer publicly exported from the root library file `src/lib.rs`.

## Implementation Plan

1. Open `src/lib.rs` and identify public module declarations (e.g., `pub mod adapters;`, `pub mod domain;`).
2. Change the visibility of internal architectural modules from `pub mod` to `pub(crate) mod` or `mod` to hide them from the public API while keeping them accessible internally.
3. Review and create a controlled public facade by explicitly exporting only the necessary types and functions using `pub use` statements, rather than exposing entire internal module structures.
4. Run tests and compilation checks to ensure that no internal code is broken by the reduced visibility.
