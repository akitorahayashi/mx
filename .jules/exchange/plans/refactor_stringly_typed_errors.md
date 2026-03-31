---
label: "refacts"
---

## Goal

Redesign the error contract in `AppError` to replace generic "stringly-typed" variants with strongly typed variants that preserve domain meaning, semantic classification, and structured context.

## Current State

`AppError` heavily relies on `String` variants (e.g., `ConfigError(String)`, `NotFound(String)`, `ClipboardError(String)`, `InvalidKey(String)`, `PathTraversal(String)`). This collapses underlying typed errors (like parsing, IO, configuration issues) into untyped strings, losing context and semantic classification across boundaries.

### Implementation Targets

- `src/domain/error.rs`: Defines the central error contract. Currently relies on generic `String` encapsulation, losing structural context when errors cross boundaries.
- `src/domain/snippet/query.rs`: Uses stringly-typed configuration errors, losing details about empty segments or invalid UTF-8 encoding.
- `src/adapters/snippet_catalog/filesystem_catalog.rs`: Overloads generic configuration and not-found strings for distinct failures like missing environment variables, filesystem traversal issues, and duplicate file matches.
- `src/app/commands/add/mod.rs` (and other commands): Reuses stringly-typed invalid key and configuration errors for domain constraint violations.
- `src/adapters/clipboard/system_clipboard.rs`: Wraps distinct execution failures (missing command, platform unsupported, bad exit code, invalid UTF-8) into an opaque clipboard error string.

### Test Targets

- Unit and integration tests (e.g., in `src/app/commands/add/mod.rs`, `src/adapters/clipboard/system_clipboard.rs`, `tests/`) currently assert against stringly-typed enum variants using `matches!(err, AppError::ConfigError(_))` instead of validating specific, structured error properties that reflect the actual domain failure.

### Documentation Targets

- Error handling documentation and API guidelines (if any) need to be updated to reflect the new strongly-typed boundaries and how context should be preserved when crossing from adapters to domain models.

## Plan

### Redefine Domain Error Contracts

Redesign `src/domain/error.rs` to own the structural integrity of application failures.
Replace string-wrapped variants (`ConfigError`, `NotFound`, `ClipboardError`, `InvalidKey`, `PathTraversal`) with nested domain-specific enums or structs that capture the exact nature of the failure (e.g., `MissingEnvVar`, `DuplicateSnippet`, `CommandNotFound`, `UnsupportedPlatform`).

### Propagate Structured Context

Modify domain query logic (`src/domain/snippet/query.rs`) and application commands (`src/app/commands/...`) to construct and return these new strongly-typed structures, ensuring that domain invariants and constraints surface explicit contextual data (e.g., the specific path that failed, the environment variable that is missing) rather than opaque strings.

### Align Adapter Boundaries

Update adapters (`src/adapters/snippet_catalog/filesystem_catalog.rs`, `src/adapters/clipboard/system_clipboard.rs`) to translate underlying system, IO, or execution failures into the precise, typed domain error variants defined by the new contract. Ensure no fallback to generic string wrapping occurs.

### Realign Test Assertions

Update test suites to assert against the specific properties of the new structured error variants. Tests must verify the externally observable state of the failure (e.g., verifying a `DuplicateSnippet` error identifies the correct duplicate path) rather than asserting against an internal string implementation.

## Acceptance Criteria

- `AppError` variants no longer utilize plain `String` types for error metadata.
- Error creation across adapters and domain models utilizes strongly-typed structures to convey context.
- Tests validate structured error context instead of string variants.
- No string matching or generic fallback strings are used to differentiate failures across system boundaries.

## Risks

- Widespread changes to the error contract may cause cascading test failures if specific error context matching is improperly mapped.
