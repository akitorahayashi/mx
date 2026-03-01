---
created_at: "2024-05-24"
author_role: "tactician"
confidence: "high"
---

## Statement

The "Tech Stack" section in `AGENTS.md` violates volatility control by restating volatile dependency details (e.g., `clap`, `serde`, `walkdir`, `assert_cmd`) that are authoritatively defined in `Cargo.toml`. These implementation details should be excluded unless they are execution-critical context for the agent.

## Evidence

- path: "AGENTS.md"
  loc: "26-36"
  note: "Restates core and testing libraries and their uses which change frequently and are managed by cargo."