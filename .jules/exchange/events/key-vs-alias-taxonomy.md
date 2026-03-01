---
created_at: "2024-05-24"
author_role: "taxonomy"
confidence: "high"
---

## Statement

The identifier used to resolve a context file path is referred to as both a "key" and an "alias" without a clear distinction or formal glossary. The CLI uses `<key>` (e.g., `mx touch <key>`), and the code has `src/domain/context_file/key.rs` to handle these. However, the hardcoded dictionary of identifiers is defined in an `alias_registry.rs` and the user documentation explicitly refers to them as "Context Management Keys (Aliases)". This violates the "One Concept, One Preferred Term" principle.

## Evidence

- path: "src/app/cli/mod.rs"
  loc: "Touch { key: String }, Cat { key: String }"
  note: "CLI arguments use 'key' to refer to the context file identifier."
- path: "src/domain/context_file/alias_registry.rs"
  loc: "resolve_alias(key: &str)"
  note: "The internal registry and lookup function uses the term 'alias' instead of 'key'."
- path: "README.md"
  loc: "Context Management Keys (Aliases)"
  note: "Documentation conflates the two terms without establishing a canonical one."
