# Development guide

- `just setup`: install pinned development tools from `mise.toml`.
- `cargo fmt --check`: check formatting.
- `cargo clippy --all-targets --all-features -- -D warnings`: run clippy in strict mode.
- `RUST_TEST_THREADS=1 cargo test --all-targets --all-features`: run test suites.
- `just coverage`: run coverage with pinned tarpaulin.

The workspace follows the original template's testing culture:

- Unit tests live next to domain and adapter modules.
- Internal test doubles are centralized under `src/testing/ports/`.
- Integration tests are organized by concern at `tests/cli/`, `tests/context/`, `tests/snippets/`, and `tests/security/`.
- Shared integration fixtures are centralized under `tests/harness/`.
