use crate::domain::error::{AppError, ConfigError, InvalidKeyError, PathTraversalError};
use crate::domain::ports::{Clipboard, SnippetStore};
use crate::domain::snippet::SnippetFrontmatter;
use crate::domain::SafePath;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AddOutcome {
    pub key: String,
    pub path: PathBuf,
}

/// Validate that `raw_path` starts with `.mx/commands/` (with optional `./` prefix)
/// and return the relative portion after that prefix.
fn extract_relative_path(raw_path: &str) -> Result<SafePath, AppError> {
    let normalized = raw_path.trim_start_matches("./");
    let stripped = normalized.strip_prefix(".mx/commands/").ok_or_else(|| {
        AppError::InvalidKey(InvalidKeyError::NotInCommands {
            expected: ".mx/commands/".to_string(),
            actual: raw_path.to_string(),
        })
    })?;

    if stripped.is_empty() {
        return Err(AppError::InvalidKey(InvalidKeyError::EmptyAfterPrefix(
            ".mx/commands/".to_string(),
        )));
    }

    let rel = Path::new(stripped);
    let safe_path = SafePath::try_from_path(rel).map_err(|_| {
        AppError::PathTraversal(PathTraversalError::Detected(format!(
            "Path contains unsafe segments: '{raw_path}'"
        )))
    })?;

    Ok(safe_path)
}

pub fn execute(
    raw_path: &str,
    title: Option<&str>,
    description: Option<&str>,
    force: bool,
    store: &dyn SnippetStore,
    clipboard: &dyn Clipboard,
) -> Result<AddOutcome, AppError> {
    let relative = extract_relative_path(raw_path)?;

    if store.snippet_exists(&relative) && !force {
        return Err(AppError::ConfigError(ConfigError::DuplicateSnippet(format!(
            "Snippet already exists: '{}'. Use --force to overwrite.",
            relative.display()
        ))));
    }

    let body = clipboard.paste()?;

    let contents = build_contents(&body, title, description);

    let path = store.write_snippet(&relative, &contents)?;
    let key = relative.file_stem().and_then(|s| s.to_str()).unwrap_or(raw_path).to_string();

    Ok(AddOutcome { key, path })
}

fn build_contents(body: &str, title: Option<&str>, description: Option<&str>) -> String {
    if title.is_none() && description.is_none() {
        return body.to_string();
    }

    let fm = SnippetFrontmatter {
        title: title.map(ToOwned::to_owned),
        description: description.map(ToOwned::to_owned),
        aliases: None,
    };

    let yaml = serde_yaml::to_string(&fm).unwrap_or_default();
    // serde_yaml serializes structs without the leading "---\n", add delimiters manually
    format!("---\n{}---\n{}", yaml, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{InMemoryClipboard, InMemorySnippetStore};

    #[test]
    fn adds_snippet_from_clipboard() {
        let store = InMemorySnippetStore::new();
        let clipboard = InMemoryClipboard::with_content("hello world");

        let outcome = execute(".mx/commands/hello.md", None, None, false, &store, &clipboard)
            .expect("add should succeed");
        assert_eq!(outcome.key, "hello");
        assert!(store.has("hello.md"));
        assert_eq!(store.read("hello.md"), "hello world");
    }

    #[test]
    fn adds_snippet_with_frontmatter() {
        let store = InMemorySnippetStore::new();
        let clipboard = InMemoryClipboard::with_content("body content");

        execute(
            ".mx/commands/foo.md",
            Some("My Title"),
            Some("My description"),
            false,
            &store,
            &clipboard,
        )
        .expect("add should succeed");

        let contents = store.read("foo.md");
        assert!(contents.starts_with("---\ntitle: My Title\n"));
        assert!(contents.contains("description: My description\n"));
        assert!(contents.contains("body content"));
    }

    #[test]
    fn rejects_duplicate_without_force() {
        let store = InMemorySnippetStore::new();
        let clipboard = InMemoryClipboard::with_content("content");

        execute(".mx/commands/dup.md", None, None, false, &store, &clipboard).unwrap();
        let err = execute(".mx/commands/dup.md", None, None, false, &store, &clipboard)
            .expect_err("should fail on duplicate");
        assert!(matches!(
            err,
            AppError::ConfigError(crate::domain::error::ConfigError::DuplicateSnippet(_))
        ));
    }

    #[test]
    fn force_overwrites_existing() {
        let store = InMemorySnippetStore::new();
        let clipboard = InMemoryClipboard::with_content("v2");

        execute(".mx/commands/foo.md", None, None, false, &store, &clipboard).unwrap();
        execute(".mx/commands/foo.md", None, None, true, &store, &clipboard)
            .expect("force should succeed");
        assert_eq!(store.read("foo.md"), "v2");
    }

    #[test]
    fn rejects_path_outside_mx_commands() {
        let store = InMemorySnippetStore::new();
        let clipboard = InMemoryClipboard::with_content("content");

        let err = execute("foo/bar.md", None, None, false, &store, &clipboard)
            .expect_err("should reject path outside .mx/commands/");
        assert!(matches!(
            err,
            AppError::InvalidKey(crate::domain::error::InvalidKeyError::NotInCommands { .. })
        ));
    }
}
