---
label: "refacts"
scope: "Use static dispatch, reduce allocations, and structure error handling"
---

## Goal

Optimize performance by reducing dynamic dispatch and unnecessary clones, and improve error diagnostics by switching from transparent string errors to structured domain errors.

## Problem

The application extensively uses dynamic dispatch in core logic, unnecessarily clones structs (SystemClipboard), and propagates errors as transparent std::io::Error or stringly-typed variants.

## Evidence

- source_event: "excessive-dynamic-dispatch-rustacean.md"
  path: "src/app/commands/copy/mod.rs"
  loc: "17-20"
  note: "Arguments `catalog`, `clipboard`, and `workspace_store` use `&dyn SnippetCatalog`, `&dyn Clipboard`, and `Option<&dyn ContextFileStore>` respectively, rather than statically typed generic bounds."

- source_event: "excessive-dynamic-dispatch-rustacean.md"
  path: "src/app/api.rs"
  loc: "35-42"
  note: "The API explicitly coerces `&LocalContextFileStore` into `&dyn ContextFileStore` to pass it down to `commands::copy::execute`. Similar coercions occur throughout `src/app/api.rs` when wrapping core logic."

- source_event: "unnecessary-clone-clipboard-rustacean.md"
  path: "src/adapters/clipboard/system_clipboard.rs"
  loc: "58-63"
  note: "Clones `command` twice (explicit `.clone()` and implicit copy of String via move if one wasn't a clone) just to set `copy_command` and `paste_command` to the same configured fallback command. This could be simplified by creating distinct commands or implementing a cleaner fallback mechanism."

- source_event: "stringly-typed-errors-rustacean.md"
  path: "src/domain/error.rs"
  loc: "4-18"
  note: "The AppError enum mixes low-level I/O errors (via #[from] io::Error) and string-based domain errors (ConfigError, NotFound, etc.). This makes it difficult to reason about the exact source or recovery of an error without parsing strings."

- source_event: "stringly-typed-errors-rustacean.md"
  path: "src/app/api.rs"
  loc: "23-26"
  note: "Public API functions return `AppError` directly, exposing stringly typed error variants to callers rather than structured data."

## Change Scope

- `src/adapters/clipboard/system_clipboard.rs`
- `src/app/api.rs`
- `src/app/commands/copy/mod.rs`
- `src/domain/error.rs`

## Constraints

- Error variants must maintain an error boundary and domain hierarchy.

## Acceptance Criteria

- Dynamic dispatch in core logic is replaced with static dispatch.
- Unnecessary clones in SystemClipboard detect() are removed.
- Errors are propagated as custom context-rich domain errors.
