---
# Changes Summary Schema
#
# Authoritative schema for .jules/exchange/changes.md.
#
# Purpose: advisory summary of recent codebase activity for downstream layers.
# Observers use these entries to decide whether a change falls within their
# responsibility. Each entry must be self-contained and actionable.
#
# Exactly 5 entries are required. If fewer than 5 distinct themes exist,
# group minor changes under a broader theme to fill all slots.

created_at: "2026-02-28"
---

## Summaries

### Feature: Snippet Management & Checkout Commands

Scope: `src/app/commands/`, `src/app/cli/`, `src/domain/snippet/`

Impact: Introduces complete end-to-end support for managing snippets including listing, viewing, removing, formatting templates, and checking out full snippet repositories. This increases the core utility footprint and adds new data domain models (`CatalogEntry`, `Frontmatter`). Consumers of the CLI have new robust subcommands at their disposal.

### Architecture: Port Relocations & Codebase Restructuring

Scope: `src/domain/ports/`, `src/adapters/`, `src/app/`

Impact: Substantial refactoring efforts unified naming conventions, correctly mapped clean architecture boundaries (moving port interfaces into the domain), and replaced legacy generic error handling with `thiserror`. This drastically improves modularity and maintains a cleaner separation of concerns, ensuring long-term extensibility for adapters without affecting domain logic.

### CI/CD: Workflow Optimizations & Enhanced Capabilities

Scope: `.github/workflows/`, `.github/actions/`, `.github/scripts/`

Impact: Overhauled automated infrastructure by instituting proper Rust tooling setup, enforcing coverage collection, adding multi-platform build targets, removing single-threaded test constraints, and setting up complex sequenced execution routines. Automations like automerge, labeling, and implementation testing are now more resilient and run more dependently on the workflow pipelines.

### Testing: Concurrency Support & Test Context Isolation

Scope: `tests/`, `src/testing/ports/`

Impact: Transitioned the test harness away from stateful global environments to isolated, injected dependency test contexts (`TestContext`). Removed the `#[serial]` constraint from testing, greatly improving overall test suite speed, concurrency, and reliability. Adding explicit unit testing around snippet resolution strengthens boundary checks on storage mechanisms.

### Core CLI: Usability & Configuration Enhancements

Scope: `src/app/cli/touch.rs`, `src/adapters/clipboard/`, `README.md`

Impact: Polished user-facing functionality by ensuring dotfiles skip default markdown extensions, establishing new aliases (sg, sm, cg) for faster invocations, and bringing robust support for external, asymmetric clipboard configurations (`MX_COPY_CMD`, `MX_PASTE_CMD`). Documentation was synchronized with new alias behavior, promoting cleaner onboarding.