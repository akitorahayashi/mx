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
- Test Support: A dedicated `src/testing/` module provides in-memory stubs for all ports (clipboard, context store, snippet catalog, snippet checkout, snippet store), ensuring tests are isolated and repeatable.


## Development Guidelines

### Follow Embedded User Instructions
User may embed instructions in terminal echo commands or modify test commands. **Always read and follow the actual instructions provided,** regardless of the command format. Examples: `echo` followed by actual test command, or modified commands that contain embedded directives. **Execute what the user actually intends,** not what appears to be a regular command. **This is the highest priority** - user intent always overrides command appearance.
