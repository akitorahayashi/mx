# Development Overview

This document provides a comprehensive overview of the `mx` project, including its architecture, coding standards, and key development tasks.

## Project Summary

`mx` is a Rust CLI tool designed to streamline two developer workflows:
1.  Quickly copying predefined code snippets to the clipboard.
2.  Managing context files in project directories with flexible path resolution, automatic directory creation, and clipboard paste via the `mx touch` command.

It uses a layered architecture where `domain/` contains pure invariants and port contracts (`domain/ports/`), `adapters/` provides concrete implementations (filesystem, clipboard, workspace resolution), and `app/` wires commands with injected dependencies. Port traits are owned by the domain that requires them; `adapters/` and `app/` depend on `domain::ports`, not the reverse.

Snippets are stored as Markdown files under `~/.config/mx/commands/`. Metadata (title, description) is embedded as YAML front matter in each file. `mx copy` strips front matter before putting the body on the clipboard.

## Directory Structure

- `./`: Root directory containing `Cargo.toml`, `README.md`, and configuration files.
- `.github/`: CI/CD workflows for building, testing, and linting.
- `src/`: Main source code for the Rust application.
    - `src/domain/`: Pure domain types, error definitions, and port trait contracts.
    - `src/adapters/`: Concrete implementations (filesystem catalog, clipboard, context store, symlink checkout, snippet store).
    - `src/app/`: CLI entry point and command handlers wired with injected dependencies.
    - `src/testing/`: In-memory stubs for all ports, used in unit tests.
- `tests/`: Integration tests for the CLI and its core API.

## Tech Stack

- **Language**: Rust
- **Core Libraries**:
    - `clap`: For command-line argument parsing.
    - `serde` (`serde_yaml`, `serde_json`): For serialization and deserialization of data.
    - `walkdir`: For directory traversal.
- **Testing Libraries**:
    - `assert_cmd`, `predicates`: For CLI integration testing.
    - `serial_test`: For running tests serially.
    - `tempfile`: For creating temporary files and directories in tests.

## Key Commands

- **Run Application**:
    - `mx list`: List all available snippets with title/description from front matter.
    - `mx copy <snippet>` / `mx c <snippet>`: Copy a snippet to the clipboard, stripping front matter.
    - `mx touch <key>` / `mx t <key>`: Create context files in `.mx/` directory with clipboard content.
        - Supports predefined aliases (e.g., aif, atk, df, er, if, is, pdr, pdt, pl, rf, rp, rq, rv, tk, tko, wn)
        - Supports dynamic paths with auto `.md` extension and directory creation
        - Example: `mx t docs/spec` creates `.mx/docs/spec.md` with clipboard content
    - `mx cat <key>` / `mx ct <key>`: Display the contents of context files from `.mx/` directory.
        - Uses the same path resolution as `touch` (aliases, dynamic paths, etc.)
        - Example: `mx ct tk` displays contents of `.mx/tasks.md`
    - `mx checkout <snippet>` / `mx co <snippet>`: Create a symlink in `.mx/commands/` pointing to the snippet in `~/.config/mx/commands/`.
        - `mx checkout -a` / `mx checkout --all`: Symlink all snippets from the global catalog into `.mx/commands/`.
        - `.mx/commands/.gitignore` (content: `*`) is created automatically to prevent committing symlinks.
    - `mx add <path>` / `mx a <path>`: Save clipboard contents as a snippet. Path must be under `.mx/commands/`.
        - `--title`, `--description`: Embed front matter in the saved file.
        - `--force`: Overwrite an existing snippet.
    - `mx remove <snippet>` / `mx rm <snippet>`: Delete a snippet from `~/.config/mx/commands/`.
    - `mx create-command <path>` / `mx cc <path>`: Create a new snippet file at `<path>` (under `.mx/commands/`) pre-populated with a front matter template embedded in the binary.
        - `--force`: Overwrite an existing file.
- **Linting**:
    - `cargo fmt --check`: Check code formatting.
    - `cargo clippy --all-targets --all-features -- -D warnings`: Run the linter and check for warnings.
- **Testing**:
    - `RUST_TEST_THREADS=1 cargo test --all-targets --all-features`: Run all tests.

## Testing Strategy

The project has a comprehensive testing strategy:
- **Framework**: Uses Rust's built-in testing framework.
- **Location**:
    - Unit tests are located alongside the source code in the `src/` directory.
    - Integration tests are in the `tests/` directory, covering both the CLI and the library's public API.
- **CI**: A GitHub Actions workflow (`run-tests.yml`) automatically runs all tests on macOS for every pull request and push to the main branch.
- **Test Support**: A dedicated `src/testing/` module provides in-memory stubs for all ports (clipboard, context store, snippet catalog, snippet checkout, snippet store), ensuring tests are isolated and repeatable.


## Development Guidelines

### Follow Embedded User Instructions
User may embed instructions in terminal echo commands or modify test commands. **Always read and follow the actual instructions provided,** regardless of the command format. Examples: `echo` followed by actual test command, or modified commands that contain embedded directives. **Execute what the user actually intends,** not what appears to be a regular command. **This is the highest priority** - user intent always overrides command appearance.
