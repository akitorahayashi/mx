---
label: "refacts"
---

## Goal

Unify CLI synonyms for deletion actions (`Clean` vs `Remove`) to avoid naming inconsistencies and structural drift.

## Problem

The CLI design introduces naming synonyms for the same conceptual action (deletion). Specifically:
- The CLI uses two different verbs, `Clean` and `Remove`, for file deletion actions in `src/app/cli/mod.rs` (Commands enum). For example, deleting context files vs. snippet files.
- This violates the design principle of avoiding synonyms.

## Affected Areas

### CLI and App

- `src/app/cli/mod.rs`

## Constraints

- The CLI verbs used for deletion actions must be uniform across all domains (context files, snippets, etc.).

## Risks

- Changing the command name from `Clean` to `Remove` (or vice versa) in the CLI may break user workflows relying on the old command name. Consider preserving the old command as a deprecated alias if necessary, or documenting the breaking change.

## Acceptance Criteria

- CLI synonyms for deletion (e.g., `Clean` vs `Remove`) are unified under a single canonical verb.
- The `src/app/cli/mod.rs` file updates the `Commands` enum to reflect the chosen verb.
- Documentation references to deletion actions consistently use the canonical verb.

## Implementation Plan

1. Choose a canonical verb for deletion actions (e.g., `Remove`).
2. Update `src/app/cli/mod.rs` to replace any occurrence of `Clean` with `Remove` in the `Commands` enum.
3. Update `src/app/cli/mod.rs` to adjust argument parsing and command matching logic.
4. Update any internal or external tests that rely on the old command name.
5. Update `README.md` to reflect the chosen canonical verb for deletion.
