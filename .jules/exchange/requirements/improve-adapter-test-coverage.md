---
label: "tests"
scope: "Add test coverage for adapter components and boundary cases"
---

## Goal

Improve reliability and diagnosability of adapter implementations by adding test coverage for boundary cases and untested modules.

## Problem

Multiple adapters (LocalContextFileStore, SystemClipboard, FilesystemSnippetStore, FileClipboard, SymlinkCheckout) lack test coverage for critical operations, boundary cases, and edge cases, representing a major blind spot for core integration paths.

## Evidence

- source_event: "local-context-store-uncovered-cov.md"
  path: "src/adapters/context_file_store/local_context_store.rs"
  loc: "50/67 lines covered"
  note: "Coverage report shows only 50/67 lines covered. Specifically, paths dealing with `read_context_contents` error handling, `remove_context_root` successful and failed cleanups, and `remove_context_file` directory climbing are missing coverage. These are important side-effects that determine whether `.mx/` state accurately tracks the snippet system or leaves dangling artifacts or incorrect reads."

- source_event: "system-clipboard-uncovered-cov.md"
  path: "src/adapters/clipboard/system_clipboard.rs"
  loc: "0/93 lines covered"
  note: "Coverage report shows 0/93 lines covered for this module. The `SystemClipboard::detect`, `Clipboard::copy`, and `Clipboard::paste` functions are completely untested, meaning that failures in reading environment variables, identifying platform commands (like `pbcopy`/`pbpaste`, `wl-copy`/`xclip`, `clip`/`powershell`), or actual OS process spawning will only be discovered in production."

- source_event: "filesystem-snippet-store-uncovered-cov.md"
  path: "src/adapters/snippet_store/filesystem_store.rs"
  loc: "31/44 lines covered"
  note: "Coverage reports indicate that `FilesystemSnippetStore` has 31/44 lines covered. This suggests that error scenarios or specific branches, particularly related to directory cleanup up to `commands_root` upon snippet removal or handling missing environment variables, might lack test validation."

- source_event: "file-clipboard-edge-cases-qa.md"
  path: "src/adapters/clipboard/file_clipboard.rs"
  loc: "33-45"
  note: "Only a single test `file_clipboard_roundtrip` exists. Lacks assertions for reading an uninitialized file or writing to an invalid path."

- source_event: "symlink-checkout-diagnosability-qa.md"
  path: "src/adapters/snippet_checkout/symlink_checkout.rs"
  loc: "42-93"
  note: "Tests `creates_symlink_in_target_root` and `skips_existing_symlink` are present, but no test covers the error path when `target_path` exists as a normal file, which is an important boundary condition for filesystem interactions."

## Change Scope

- `src/adapters/clipboard/file_clipboard.rs`
- `src/adapters/clipboard/system_clipboard.rs`
- `src/adapters/context_file_store/local_context_store.rs`
- `src/adapters/snippet_checkout/symlink_checkout.rs`
- `src/adapters/snippet_store/filesystem_store.rs`

## Constraints

- Tests must cover boundary cases (e.g., empty files, non-existent files, destination exists as a regular file).
- Tests should not rely on manual interaction or real system clipboard if possible, or use explicit fallbacks.

## Acceptance Criteria

- SystemClipboard detect, copy, and paste are tested.
- FilesystemSnippetStore persistence and removal operations have test coverage.
- LocalContextFileStore critical file operations and cleanup paths are tested.
- FileClipboard test verifies behavior when pasting from an empty or non-existent file.
- SymlinkCheckout test covers error path when target_path exists as a normal file.
