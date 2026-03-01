---
created_at: "2024-05-24"
author_role: "structural_arch"
confidence: "high"
---

## Statement

The CLI commands are organized by horizontal layers (`src/app/cli/` for argument parsing and `src/app/commands/` for execution logic) rather than by vertical slices. This violates the "Cohesion by change reason" principle, forcing scattered edits across multiple files and directories to implement or modify a single feature.

## Evidence

- path: "src/app/cli/"
  loc: "Entire directory"
  note: "Contains CLI-specific logic (e.g., `add.rs`, `checkout.rs`) that is coupled to the corresponding command implementations, requiring changes here for any new feature."
- path: "src/app/commands/"
  loc: "Entire directory"
  note: "Contains the execution logic for each command (e.g., `add/mod.rs`, `checkout/mod.rs`), forcing developers to navigate between `src/app/cli/` and `src/app/commands/` to understand a single feature's flow."
- path: "src/app/cli/mod.rs"
  loc: "line 43"
  note: "The match statement directly maps CLI variants to the `src/app/commands/` logic, further cementing the horizontal split."