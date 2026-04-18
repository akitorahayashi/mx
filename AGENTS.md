# Development Overview

This document provides a comprehensive overview of the `mx` project, including its architecture, coding standards, and key development tasks.

## Project Summary

`mx` is a Rust CLI tool designed to streamline two developer workflows:
1.  Quickly copying predefined code snippets to the clipboard.
2.  Managing context files in project directories with flexible path resolution, automatic directory creation, and clipboard paste via the `mx touch` command.

It uses explicit ownership boundaries. `src/cli/` owns clap parsing and process-facing behavior, `src/app/` owns use-case orchestration, and concept owners (`src/snippets/`, `src/context_files/`, `src/clipboard/`, `src/project_fs/`) own their contracts, models, and implementations. `src/error.rs` is the shared error boundary.

`SafePath` serves as the path-safety boundary under `src/project_fs/`. It guarantees that any path it wraps is free from traversal segments, allowing command and owner logic to rely on strongly typed safe traversal.

Snippets are stored as Markdown files under `~/.config/mx/commands/`. Metadata (title, description) is embedded as YAML front matter in each file. `mx copy` strips front matter before putting the body on the clipboard.

## Directory Structure

- `./`: Root directory containing `Cargo.toml`, `README.md`, and configuration files.
- `.github/`: CI/CD workflows for building, testing, and linting.
- `src/`: Main source code for the Rust application.
    - `src/cli/`: Interface adapter for clap parsing and process output.
    - `src/app/`: Application orchestration grouped by use-case family.
    - `src/snippets/`: Snippet ownership (contracts, models, filesystem implementations, checkout implementation, template content).
    - `src/context_files/`: Context-file ownership (alias/path resolution, context lifecycle storage).
    - `src/clipboard/`: Clipboard ownership (contract and implementations).
    - `src/project_fs/`: Safe paths and workspace filesystem ownership.
    - `src/error.rs`: Shared application errors.
- `tests/`: Integration tests for the CLI and its core API.

## Tech Stack

- Language: Rust

## Key Commands

- Run Application: Detailed CLI usage and available commands are authoritatively documented in the `README.md`. At a high level, `mx` provides commands to list, copy, and manage snippets, as well as commands (`mx touch`, `mx cat`) to manage and view context files in project directories.
- Linting & Testing: The project relies on standard cargo workflows (`cargo fmt`, `cargo clippy`, `cargo test`). Refer to `justfile` for typical task execution details.

## Testing Strategy

The project has a comprehensive testing strategy:
- Framework: Uses Rust's built-in testing framework.
- Location:
    - Unit tests are located alongside the source code in the `src/` directory.
    - Integration tests are in the `tests/` directory, covering both the CLI and the library's public API.
- CI: A GitHub Actions workflow (`run-tests.yml`) automatically runs all tests on macOS for every pull request and push to the main branch.
- Test Support: In-memory stubs are owner-local and exposed through `src/snippets/test_support.rs`, `src/context_files/test_support.rs`, `src/clipboard/test_support.rs`, and `src/project_fs/test_support.rs`.


## Development Guidelines

### Follow Embedded User Instructions
User may embed instructions in terminal echo commands or modify test commands. Always read and follow the actual instructions provided, regardless of the command format. Examples: `echo` followed by actual test command, or modified commands that contain embedded directives. Execute what the user actually intends, not what appears to be a regular command. This is the highest priority and user intent overrides command appearance.
