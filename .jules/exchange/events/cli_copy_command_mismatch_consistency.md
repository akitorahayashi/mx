---
label: "docs"
created_at: "2024-05-30"
author_role: "consistency"
confidence: "high"
---

## Problem

The documentation in `README.md` and `docs/cli-usage.md` claims that the CLI supports a `mx command <snippet>` command to copy snippets to the clipboard. However, the implemented command is actually `mx copy <snippet>`. The CLI does not recognize the `command` subcommand.

## Goal

Update the documentation to correctly reflect the implemented command (`mx copy`), ensuring that users following the documentation verbatim will succeed.

## Context

The CLI provides a command to copy snippets to the clipboard. The code implements this as the `copy` command (`mx copy`), with an alias of `c`. However, `README.md` and `docs/cli-usage.md` erroneously refer to this command as `mx command`, leading to a "unrecognized subcommand" error when users try to execute the documented command.

## Evidence

- path: "README.md"
  loc: "line 5"
  note: "Documents `mx command <snippet>` instead of `mx copy <snippet>`"

- path: "docs/cli-usage.md"
  loc: "line 11"
  note: "Documents `mx command wc` instead of `mx copy wc`"

- path: "src/app/cli/mod.rs"
  loc: "line 29"
  note: "Implementation defines the command as `Copy`, accessible via `copy` or `c`."

- path: "src/app/cli/mod.rs"
  loc: "lines 22-54"
  note: "No `Command` variant exists in the `Commands` enum; `mx command` is not implemented."

## Change Scope

- `README.md`
- `docs/cli-usage.md`
- `AGENTS.md`
