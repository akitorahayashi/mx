---
created_at: "2026-03-01"
author_role: "rustacean"
confidence: "high"
---

## Statement

The application propagates errors as transparent `std::io::Error` or stringly-typed variants, potentially losing semantic domain context. For instance, file operations fail with generic `std::io::Error` instead of custom context-rich errors that clarify *why* the read or write occurred. The `AppError` enum is flat and does not maintain an error boundary or domain hierarchy.

## Evidence

- path: "src/domain/error.rs"
  loc: "4-18"
  note: "The AppError enum mixes low-level I/O errors (via #[from] io::Error) and string-based domain errors (ConfigError, NotFound, etc.). This makes it difficult to reason about the exact source or recovery of an error without parsing strings."
- path: "src/app/api.rs"
  loc: "23-26"
  note: "Public API functions return `AppError` directly, exposing stringly typed error variants to callers rather than structured data."
