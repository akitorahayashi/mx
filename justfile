set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

setup:
    mise install

# Format code with rustfmt
fmt:
    cargo fmt

# Check code formatting and run clippy
check: fmt
    cargo check
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
test:
    cargo test --all-targets --all-features

coverage:
    rm -rf target/tarpaulin coverage
    env -u RUSTC_WRAPPER -u SCCACHE_IGNORE_SERVER_IO_ERROR -u SCCACHE_ERROR_LOG mise exec -- cargo tarpaulin --engine llvm --out Xml --output-dir coverage --all-features --fail-under 30
