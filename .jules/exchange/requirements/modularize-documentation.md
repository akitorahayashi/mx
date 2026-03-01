---
label: "docs"
scope: "Split README.md into a structured docs/ directory"
---

## Goal

Improve navigation and separation of concerns in documentation by extracting CLI usage, configuration, and development guides from README.md into a dedicated docs/ structure.

## Problem

The repository lacks a dedicated docs/ foundation, forcing the root README.md to act as a catch-all document with mixed responsibilities.

## Evidence

- source_event: "missing-docs-directory-librarian.md"
  path: "."
  loc: "root directory"
  note: "No `docs/` directory exists. This violates the principle that 'If no structural foundation exists, architect and scaffold it from scratch rather than forcing content into unrelated files.' and that root namespace is scarce."

- source_event: "readme-mixed-responsibilities-librarian.md"
  path: "README.md"
  loc: "Lines 16-86"
  note: "Contains detailed CLI usage instructions and extensive reference tables for context management aliases and template placeholders, which should be in a separate specification or procedure document."

- source_event: "readme-mixed-responsibilities-librarian.md"
  path: "README.md"
  loc: "Lines 125-147"
  note: "Contains the complete development guide including commands, testing culture, and internal testing structural patterns, conflating the external user's entry point with the internal developer's procedural guidelines."

## Change Scope

- `.`
- `README.md`

## Constraints

- README.md remains a purely map-level orientation entry point.

## Acceptance Criteria

- docs/ directory is scaffolded.
- CLI usage, configuration, and development guides are extracted into specific docs/*.md files.
- README.md is slimmed down to an entry-level map.
