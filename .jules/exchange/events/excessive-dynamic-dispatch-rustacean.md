---
created_at: "2026-03-01"
author_role: "rustacean"
confidence: "high"
---

## Statement

The application extensively uses dynamic dispatch (`&dyn Trait`) in core logic functions (`app/commands/`), imposing potential runtime overhead instead of relying on monomorphized static dispatch (`impl Trait`) where it would be sufficient and potentially more performant, given that most commands operate with specific static combinations of struct dependencies.

## Evidence

- path: "src/app/commands/copy/mod.rs"
  loc: "17-20"
  note: "Arguments `catalog`, `clipboard`, and `workspace_store` use `&dyn SnippetCatalog`, `&dyn Clipboard`, and `Option<&dyn ContextFileStore>` respectively, rather than statically typed generic bounds."
- path: "src/app/api.rs"
  loc: "35-42"
  note: "The API explicitly coerces `&LocalContextFileStore` into `&dyn ContextFileStore` to pass it down to `commands::copy::execute`. Similar coercions occur throughout `src/app/api.rs` when wrapping core logic."
