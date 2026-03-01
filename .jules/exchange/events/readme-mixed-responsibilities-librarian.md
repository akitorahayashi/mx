---
created_at: "2026-03-01"
author_role: "librarian"
confidence: "high"
---

## Statement

The root `README.md` suffers from mixed responsibilities. Instead of remaining purely map-level orientation (entry point), it acts as a catch-all document, aggregating CLI usage, detailed context alias tables, dynamic pathing rules, and the complete development guide. This flat accumulation hides structural intent and makes lookup branching inefficient. The file should be split into distinct canonical paths such as `docs/cli.md`, `docs/configuration.md`, and `docs/development.md`.

## Evidence

- path: "README.md"
  loc: "Lines 16-86"
  note: "Contains detailed CLI usage instructions and extensive reference tables for context management aliases and template placeholders, which should be in a separate specification or procedure document."
- path: "README.md"
  loc: "Lines 125-147"
  note: "Contains the complete development guide including commands, testing culture, and internal testing structural patterns, conflating the external user's entry point with the internal developer's procedural guidelines."