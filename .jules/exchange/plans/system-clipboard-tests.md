---
label: "tests"
---

## Goal

Add system clipboard coverage to `SystemClipboard`.

## Problem

Coverage report shows 0/93 lines covered for this module. The `SystemClipboard::detect`, `Clipboard::copy`, and `Clipboard::paste` functions are completely untested, meaning that failures in reading environment variables, identifying platform commands (like `pbcopy`/`pbpaste`, `wl-copy`/`xclip`, `clip`/`powershell`), or actual OS process spawning will only be discovered in production.

## Affected Areas

### SystemClipboard

- `src/adapters/clipboard/system_clipboard.rs`

## Constraints

- Avoid real OS clipboard when possible, or use explicit fallbacks.
- Tests should not rely on manual interaction.

## Risks

- Failures in identifying platform commands, reading environment variables, or process spawning may only be discovered in production.

## Acceptance Criteria

- `SystemClipboard` detect, copy, and paste are tested.

## Implementation Plan

1. Write tests for environment reading to ensure correct variable parsing.
2. Write tests for identifying platform commands (`pbcopy`/`pbpaste`, `wl-copy`/`xclip`, `clip`/`powershell`).
3. Write tests for actual OS process spawning.
