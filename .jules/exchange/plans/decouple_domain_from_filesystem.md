---
label: "refacts"
---

## Goal

Decouple the core domain logic and application commands from the filesystem by abstracting file I/O behind ports, removing `absolute_path` from the domain model, and improving test isolation.

## Current State

The core domain model for a snippet (`SnippetEntry`) leaks filesystem concepts by exposing an `absolute_path`. Application commands perform direct filesystem reads, coupling them to the filesystem and forcing tests to use `tempfile`.

- `src/domain/snippet/catalog_entry.rs`: `SnippetEntry` contains `absolute_path`, leaking filesystem concepts into the core domain.
- `src/app/commands/copy/mod.rs`: Performs direct filesystem read (`std::fs::read_to_string(&snippet_entry.absolute_path)`), coupling it to the filesystem and forcing tests to use `tempfile`.
- `src/app/commands/list/mod.rs`: Performs direct filesystem read, coupling to the filesystem and forcing tests to use `tempfile`.
- `src/app/commands/checkout/mod.rs`: Tests construct `SnippetEntry` with mock absolute paths. `symlink_checkout` adapter relies on `absolute_path` for creating symlinks.
- `src/domain/ports/snippet_store.rs`: Needs a way to read snippet contents.
- `src/adapters/snippet_catalog/filesystem_catalog.rs`: Exposes `absolute_path` when creating `SnippetEntry`.
- `src/testing/ports/in_memory_catalog.rs`: Tests that construct `SnippetEntry` provide absolute paths.
- `src/adapters/snippet_checkout/symlink_checkout.rs`: `symlink` operation uses `snippet.absolute_path`.

## Plan

1. **Add `read_snippet` and `get_snippet_path` to `SnippetStore` port**
   - In `src/domain/ports/snippet_store.rs`, add `fn read_snippet(&self, relative_path: &Path) -> Result<String, AppError>;`
   - In `src/domain/ports/snippet_store.rs`, add `fn get_snippet_path(&self, relative_path: &Path) -> Result<PathBuf, AppError>;`
   - Update `src/adapters/snippet_store/filesystem_store.rs` to implement `read_snippet` and `get_snippet_path`.
   - Update `src/testing/ports/in_memory_snippet_store.rs` to implement `read_snippet` and `get_snippet_path`.

2. **Remove `absolute_path` from `SnippetEntry`**
   - Remove `absolute_path` from `SnippetEntry` in `src/domain/snippet/catalog_entry.rs`.
   - Update `FilesystemSnippetCatalog::enumerate_snippets` in `src/adapters/snippet_catalog/filesystem_catalog.rs` to stop providing `absolute_path`.

3. **Update Application Command `copy` to use `SnippetStore`**
   - Update `src/app/api.rs` to pass `store: &impl SnippetStore` to `copy_snippet`.
   - Update `src/app/commands/copy/mod.rs` to accept `store: &dyn SnippetStore` in `execute`.
   - Replace `fs::read_to_string` with `store.read_snippet(Path::new(&snippet_entry.relative_path).with_extension("md"))`.
   - Update tests in `src/app/commands/copy/mod.rs` to use `InMemorySnippetStore` instead of `tempfile`.
   - Remove `absolute_path` from `CopyOutcome` in `src/app/commands/copy/mod.rs` and `src/app/cli/copy.rs`.

4. **Update Application Command `list` to use `SnippetStore`**
   - Update `src/app/api.rs` to pass `store: &impl SnippetStore` to `list_snippets`.
   - Update `src/app/commands/list/mod.rs` to accept `store: &dyn SnippetStore` in `execute`.
   - Iterate entries and use `store.read_snippet` instead of `fs::read_to_string` in `src/app/commands/list/mod.rs`.
   - Update tests in `src/app/commands/list/mod.rs` to use `InMemorySnippetStore` instead of `tempfile`.

5. **Update Application Command `checkout` to use `SnippetStore`**
   - Update `src/domain/ports/snippet_checkout.rs` to `fn checkout(&self, snippet: &SnippetEntry, store: &dyn SnippetStore, target_root: &Path) -> Result<CheckoutStatus, AppError>;`
   - Update `src/app/commands/checkout/mod.rs` to accept `store: &dyn SnippetStore` in `execute` and pass it to `checkout.checkout`.
   - Modify `src/app/api.rs` to pass `store: &impl SnippetStore` to `checkout_snippets`.
   - Remove `absolute_path` dependency from tests in `src/app/commands/checkout/mod.rs`.

6. **Update Ports and Adapters to handle `absolute_path` removal**
   - Update `src/adapters/snippet_checkout/symlink_checkout.rs` to use `store.get_snippet_path` to retrieve the absolute path for the symlink operation.
   - Update `src/testing/ports/in_memory_checkout.rs` to implement the updated `SnippetCheckout` trait.

7. **Refactor Tests**
   - Update tests in `src/testing/ports/in_memory_catalog.rs` to remove `absolute_path` from `SnippetEntry` instantiations.
   - Update tests in `src/adapters/snippet_catalog/filesystem_catalog.rs` to reflect `SnippetEntry` changes.
   - Update tests in `src/adapters/snippet_checkout/symlink_checkout.rs` to pass an `InMemorySnippetStore`.

## Acceptance Criteria

- `SnippetEntry` no longer contains `absolute_path`.
- Commands (`copy`, `list`, `checkout`) use a port (`SnippetStore` or updated `SnippetCatalog`) to fetch snippet content instead of `std::fs::read_to_string`.
- Application logic unit tests for these commands no longer require `tempfile` or direct filesystem interactions.

## Risks

- The `SymlinkCheckout` adapter relies on `absolute_path` to create a symlink. Adding `get_snippet_path` to `SnippetStore` mitigates this risk by providing a port-based way to resolve paths when needed by specific adapters.