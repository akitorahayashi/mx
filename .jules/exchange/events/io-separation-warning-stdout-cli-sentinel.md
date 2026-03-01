---
created_at: "2025-03-01"
author_role: "cli_sentinel"
confidence: "high"
---

## Statement

Diagnostic and warning messages are being printed to standard output (`stdout`) rather than standard error (`stderr`), violating I/O separation principles and potentially breaking automation when standard output is piped.

## Evidence

- path: "src/app/cli/touch.rs"
  loc: "line 8"
  note: "The warning message '⚠️ Context file already exists: ...' is emitted using `println!` (stdout) instead of `eprintln!` (stderr)."