set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default: help

help:
    @echo "Usage: just [recipe]"
    @echo ""
    @echo "Development tasks for mx CLI:"
    @just --list | tail -n +2 | awk '{printf "  \033[36m%-20s\033[0m %s\n", $1, substr($0, index($0, $2))}'

# Initialize project: install dependencies
setup:
    @echo "🪄 Installing tools with mise..."
    @mise trust
    @mise install --locked

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
