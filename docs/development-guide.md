# Development guide

## Common Commands

- `just setup`: install pinned development tools from `mise.toml`.
- `cargo fmt --check`: check formatting.
- `cargo clippy --all-targets --all-features -- -D warnings`: run clippy in strict mode.
- `RUST_TEST_THREADS=1 cargo test --all-targets --all-features`: run test suites.
- `just coverage`: run coverage with pinned tarpaulin.

## Testing Culture

The workspace follows the original template's testing culture:

- Unit tests live next to owner modules.
- Internal test doubles are owner-local and exported directly through their respective `mod.rs` (`src/snippets/mod.rs`, `src/context_files/mod.rs`, `src/clipboard/mod.rs`, `src/project_fs/mod.rs`).
- Integration tests are organized by concern at `tests/cli/`, `tests/context/`, `tests/snippets/`, and `tests/security/`.
- Shared integration fixtures are centralized under `tests/harness/`.
