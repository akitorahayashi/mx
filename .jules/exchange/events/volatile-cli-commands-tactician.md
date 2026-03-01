---
created_at: "2024-05-24"
author_role: "tactician"
confidence: "high"
---

## Statement

The "Key Commands" section in `AGENTS.md` violates volatility control by listing detailed CLI commands, aliases, and flags (e.g., `mx copy <snippet>`, `-a`, `--force`), which are ephemeral operational details better discovered via CLI help (`--help`) or examining the code directly.

## Evidence

- path: "AGENTS.md"
  loc: "38-58"
  note: "Lists volatile specific usage parameters and flags for commands instead of focusing on strict scoped rules."