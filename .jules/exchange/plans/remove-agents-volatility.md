---
label: "docs"
---

## Goal

Ensure `AGENTS.md` complies with volatility control principles by removing ephemeral technical details and focusing on scoped rules.

## Problem

`AGENTS.md` violates volatility control by listing detailed CLI commands, aliases, flags, and volatile dependency details that are authoritatively defined elsewhere (like in `Cargo.toml` or CLI help commands).

## Affected Areas

### Documentation

- `AGENTS.md`

## Constraints

- `AGENTS.md` should focus on strict scoped rules and essential execution-critical context.
- Volatile specific usage parameters and flags should be removed from `AGENTS.md`.
- Volatile dependency details (Cargo dependencies) should be removed from `AGENTS.md`.

## Risks

- Removing too much information might leave future agents without enough context to build or understand the project. We should ensure only volatile information is removed, preserving essential procedures if they are non-standard.

## Acceptance Criteria

- Volatile specific usage parameters and flags are removed from `AGENTS.md`.
- Volatile dependency details (Cargo dependencies) are removed from `AGENTS.md`.
- `AGENTS.md` correctly guides LLMs and developers without duplicating definitions that exist in tools or manifests.

## Implementation Plan

1. Edit `AGENTS.md` to remove the list of core libraries and testing libraries from the "Tech Stack" section (lines 26-36). Just keep "Language: Rust".
2. Edit `AGENTS.md` to remove the specific, volatile CLI commands with their flags and aliases from the "Key Commands", "Linting", and "Testing" sections (lines 38-58). Keep only high-level conceptual descriptions of what `mx` does, rather than specific argument-level documentation.
3. Review `AGENTS.md` to ensure no other volatile information is present.
4. Run `cargo test` to ensure no changes broke anything.
