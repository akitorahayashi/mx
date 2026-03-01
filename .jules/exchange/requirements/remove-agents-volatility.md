---
label: "docs"
scope: "Remove volatile technical details and CLI commands from AGENTS.md"
---

## Goal

Ensure AGENTS.md complies with volatility control principles by removing ephemeral technical details and focusing on scoped rules.

## Problem

AGENTS.md violates volatility control by listing detailed CLI commands, aliases, flags, and volatile dependency details that are authoritatively defined elsewhere.

## Evidence

- source_event: "volatile-tech-stack-tactician.md"
  path: "AGENTS.md"
  loc: "26-36"
  note: "Restates core and testing libraries and their uses which change frequently and are managed by cargo."

- source_event: "volatile-cli-commands-tactician.md"
  path: "AGENTS.md"
  loc: "38-58"
  note: "Lists volatile specific usage parameters and flags for commands instead of focusing on strict scoped rules."

## Change Scope

- `AGENTS.md`

## Constraints

- AGENTS.md should focus on strict scoped rules and essential execution-critical context.

## Acceptance Criteria

- Volatile specific usage parameters and flags are removed from AGENTS.md.
- Volatile dependency details (Cargo dependencies) are removed from AGENTS.md.
