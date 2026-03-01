---
label: "docs"
---

## Goal

Improve navigation and separation of concerns in documentation by extracting CLI usage, configuration, and development guides from `README.md` into a dedicated `docs/` structure.

## Problem

The repository lacks a dedicated `docs/` foundation, forcing the root `README.md` to act as a catch-all document with mixed responsibilities. This violates the principle that "If no structural foundation exists, architect and scaffold it from scratch rather than forcing content into unrelated files," and clutters the root namespace.

## Affected Areas

### Documentation

- `README.md`
- `docs/` (new directory)

## Constraints

- `README.md` remains a purely map-level orientation entry point.
- The new structure should reside under `docs/`.

## Risks

- Links to sections in the current `README.md` from external sources might break.

## Acceptance Criteria

- `docs/` directory is scaffolded.
- CLI usage, configuration, and development guides are extracted into specific `docs/*.md` files.
- `README.md` is slimmed down to an entry-level map.

## Implementation Plan

1. Create a `docs/` directory in the repository root.
2. Extract the "CLI usage", "Context Management Keys (Aliases)", "Dynamic Path Resolution", "Default Clipboard Paste Behavior", and "Template placeholders (dynamic context)" sections from `README.md` into `docs/cli-usage.md`.
3. Extract the "Environment overrides" section from `README.md` into `docs/configuration.md`.
4. Extract the "Development guide" section from `README.md` into `docs/development-guide.md`.
5. Update `README.md` to remove the extracted sections and replace them with short descriptions and links to the new files in the `docs/` directory.
