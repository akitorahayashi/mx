use crate::domain::error::AppError;
use crate::domain::ports::SnippetStore;
use std::path::{Path, PathBuf};

const TEMPLATE: &str = include_str!("../../../assets/command_template.md");

#[derive(Debug, Clone)]
pub struct CreateCommandOutcome {
    pub key: String,
    pub path: PathBuf,
}

fn extract_relative_path(raw_path: &str) -> Result<PathBuf, AppError> {
    let normalized = raw_path.trim_start_matches("./");
    let stripped = normalized.strip_prefix(".mx/commands/").ok_or_else(|| {
        AppError::invalid_key(format!("Path must be under .mx/commands/ (got '{raw_path}')"))
    })?;

    if stripped.is_empty() {
        return Err(AppError::invalid_key("Path cannot be empty after .mx/commands/"));
    }

    // Reject raw dot/double-dot segments before path normalization erases them.
    for segment in stripped.split('/') {
        if segment == "." || segment == ".." || segment.is_empty() {
            return Err(AppError::path_traversal(format!(
                "Path contains unsafe segments: '{raw_path}'"
            )));
        }
    }

    let rel = Path::new(stripped);
    if rel.extension().map(|e| e != "md").unwrap_or(false) {
        return Err(AppError::invalid_key(format!(
            "Path must have a .md extension (got '{raw_path}')"
        )));
    }

    Ok(rel.to_path_buf())
}

pub fn execute(
    raw_path: &str,
    force: bool,
    store: &dyn SnippetStore,
) -> Result<CreateCommandOutcome, AppError> {
    let relative = extract_relative_path(raw_path)?;

    if store.snippet_exists(&relative) && !force {
        return Err(AppError::config_error(format!(
            "Snippet already exists: '{}'. Use --force to overwrite.",
            relative.display()
        )));
    }

    let path = store.write_snippet(&relative, TEMPLATE)?;
    let key = relative
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            AppError::invalid_key(format!("Path must end with a valid filename: '{raw_path}'"))
        })?
        .to_string();

    Ok(CreateCommandOutcome { key, path })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::InMemorySnippetStore;

    #[test]
    fn creates_template_at_given_path() {
        let store = InMemorySnippetStore::new();
        let outcome = execute(".mx/commands/my-cmd.md", false, &store).expect("should succeed");
        assert_eq!(outcome.key, "my-cmd");
        assert!(store.has("my-cmd.md"));
        let content = store.read("my-cmd.md");
        assert!(content.starts_with("---\ntitle:"), "template should start with frontmatter");
    }

    #[test]
    fn fails_on_duplicate_without_force() {
        let store = InMemorySnippetStore::new();
        store.seed("my-cmd.md", "existing");
        let err = execute(".mx/commands/my-cmd.md", false, &store).unwrap_err();
        assert!(err.to_string().contains("already exists"));
    }

    #[test]
    fn force_overwrites_existing() {
        let store = InMemorySnippetStore::new();
        store.seed("my-cmd.md", "old content");
        execute(".mx/commands/my-cmd.md", true, &store).expect("should succeed with --force");
        let content = store.read("my-cmd.md");
        assert!(content.starts_with("---\n"), "template should be written");
    }

    #[test]
    fn rejects_path_outside_mx_commands() {
        let store = InMemorySnippetStore::new();
        let err = execute("other/path.md", false, &store).unwrap_err();
        assert!(err.to_string().contains("must be under .mx/commands/"));
    }

    #[test]
    fn rejects_path_with_dot_segments() {
        let store = InMemorySnippetStore::new();
        let err = execute(".mx/commands/sub/./bad.md", false, &store).unwrap_err();
        assert!(err.to_string().contains("unsafe segments"));
    }

    #[test]
    fn rejects_non_markdown_extension() {
        let store = InMemorySnippetStore::new();
        let err = execute(".mx/commands/not-markdown.txt", false, &store).unwrap_err();
        assert!(err.to_string().contains(".md"));
    }
}
