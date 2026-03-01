---
created_at: "2026-03-01"
author_role: "rustacean"
confidence: "high"
---

## Statement

The `SystemClipboard` struct clones `ClipboardCommand` unnecessarily during structural assembly in `detect()`, which could be optimized. The `ClipboardCommand` instances only hold `String` allocations, making clones slightly expensive but mostly just noisy for what could be done with moves.

## Evidence

- path: "src/adapters/clipboard/system_clipboard.rs"
  loc: "58-63"
  note: "Clones `command` twice (explicit `.clone()` and implicit copy of String via move if one wasn't a clone) just to set `copy_command` and `paste_command` to the same configured fallback command. This could be simplified by creating distinct commands or implementing a cleaner fallback mechanism."
