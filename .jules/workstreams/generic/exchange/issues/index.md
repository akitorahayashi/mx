# Issues Index

This registry tracks active issues in this workstream.
It serves as the central source of truth for the **Decider** to deduplicate observations.

## Feats
> New feature specifications in [`feats/`](./feats/).

| Issue | Summary |
| :--- | :--- |
| _No open issues_ | - |

## Refacts
> Code improvements and technical debt in [`refacts/`](./refacts/).

| Issue | Summary |
| :--- | :--- |
| [Decouple Context Logic from Touch Command](./refacts/decouple-context-logic.yml) | Move shared context logic from touch command to a dedicated domain module. |
| [Cleanup Dead Code and Redundant Models](./refacts/cleanup-models.yml) | Remove dead fields in ListEntry and unify duplicate snippet models. |
| [CI/CD Standardization and Hardening](./refacts/cicd-standardization.yml) | Align Rust versions, remove redundancy, and pin GitHub Actions. |
| [Improve Error Handling and Visibility](./refacts/improve-error-handling.yml) | Refactor AppError to preserve type information and fix hidden IO failures. |
| [Standardize Command Naming and Domain Terminology](./refacts/standardize-naming.yml) | Rename files and concepts to be consistent (e.g., 'Snippet' everywhere). |

## Bugs
> Defect reports and fixes in [`bugs/`](./bugs/).

| Issue | Summary |
| :--- | :--- |
| [Implicit File Extension Logic Flaw](./bugs/implicit-file-extension.yml) | mx touch incorrectly appends .md to filenames starting with a dot (e.g., .gitignore). |

## Tests
> Test coverage and infrastructure changes in [`tests/`](./tests/).

| Issue | Summary |
| :--- | :--- |
| [Improve Integration Test Isolation and Concurrency](./tests/improve-test-isolation.yml) | Refactor TestContext to avoid global state modification, remove redundant tests, and fix implicit dependencies. |
| [Untested Snippet Resolution Edge Cases and Traversal Risks](./tests/security-coverage-gap.yml) | Critical logic for snippet resolution and path sanitization lacks unit tests, exposing security risks. |

## Docs
> Documentation updates in [`docs/`](./docs/).

| Issue | Summary |
| :--- | :--- |
| [Sync Documentation with Implementation](./docs/sync-documentation.yml) | Update README and help text to include missing aliases and remove legacy feature references. |

<!--
Instructions for Decider:
1. Populate each section with issues from `feats/`, `refacts/`, `bugs/`, `tests/`, and `docs/` directories.
2. Format as `| [Title](./path/to/issue.yml) | Summary content |`.
3. Keep this index in sync with the file system.
-->
