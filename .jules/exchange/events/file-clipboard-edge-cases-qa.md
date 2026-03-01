---
created_at: "2026-03-01"
author_role: "qa"
confidence: "high"
---

## Statement

The `FileClipboard` adapter tests only verify the happy path for roundtrip copy and paste. They do not verify behavior when pasting from an empty or non-existent file, which could be an issue since it interacts directly with the filesystem (I/O). Testing failure diagnosability here ensures fallback errors are clear.

## Evidence

- path: "src/adapters/clipboard/file_clipboard.rs"
  loc: "33-45"
  note: "Only a single test `file_clipboard_roundtrip` exists. Lacks assertions for reading an uninitialized file or writing to an invalid path."
