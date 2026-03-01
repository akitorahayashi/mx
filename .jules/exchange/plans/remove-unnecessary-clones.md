---
label: "refacts"
---

## Goal

Optimize performance by removing unnecessary clones in `SystemClipboard`.

## Problem

Unnecessary clones of `ClipboardCommand` strings occur in `src/adapters/clipboard/system_clipboard.rs`'s `detect()` method. Specifically, when handling the `MX_CLIPBOARD_CMD` custom command, the same command object is cloned just to be set as both the `copy_command` and `paste_command`.

## Affected Areas

### Clipboard Adapter

- `src/adapters/clipboard/system_clipboard.rs`

## Constraints

- Custom clipboard commands (defined via `MX_CLIPBOARD_CMD`) must still be correctly propagated to both copy and paste logic.
- Avoid any change in the external behavior or execution environment for custom commands.

## Risks

- We might introduce lifetime issues if we just switch from cloning to referencing. The simplest fix might just be generating distinct commands from the parsing step or redesigning the `SystemClipboard` struct to optionally hold a single command when they are identical.

## Acceptance Criteria

- Unnecessary `.clone()` calls in `SystemClipboard::detect()` are removed.
- Tests still pass, and `SystemClipboard` continues to function for `MX_CLIPBOARD_CMD`.

## Implementation Plan

1. Modify `src/adapters/clipboard/system_clipboard.rs`:
   - Redesign `SystemClipboard` to hold a single custom command if the fallback mechanism is used, or simply parse the `MX_CLIPBOARD_CMD` string twice to create two separate `ClipboardCommand` instances instead of parsing once and cloning the potentially large strings within.
   - Example fix:
     ```rust
     if let Ok(custom) = env::var("MX_CLIPBOARD_CMD") {
         let copy_command = ClipboardCommand::from_string(&custom)
             .ok_or_else(|| AppError::clipboard_error("MX_CLIPBOARD_CMD is empty"))?;
         let paste_command = ClipboardCommand::from_string(&custom).unwrap();
         return Ok(Self { copy_command, paste_command });
     }
     ```
   - Alternatively, make `paste_command` an `Option<ClipboardCommand>` and fallback to `copy_command` if `None`, thereby avoiding both parsing twice and cloning.
2. Run tests to ensure no regressions in clipboard behavior.
