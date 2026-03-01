---
created_at: "2024-05-24"
author_role: "structural_arch"
confidence: "high"
---

## Statement

The API layer functions act as an orchestration point but inconsistently mix adapter instantiation with dependency injection. Hardcoding adapters (e.g., `LocalContextFileStore`, `SymlinkCheckout`) ties the API facade to specific infrastructure instead of remaining agnostic.

## Evidence

- path: "src/app/api.rs"
  loc: "lines 22, 27, 43"
  note: "Directly instantiates `LocalContextFileStore::new(find_workspace_root()?)`, hardwiring local filesystem assumptions."
- path: "src/app/api.rs"
  loc: "line 53"
  note: "Directly instantiates `SymlinkCheckout::new()`, coupling the checkout workflow specifically to a symlink strategy instead of accepting the `SnippetCheckout` port."