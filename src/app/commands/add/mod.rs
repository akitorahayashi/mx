use crate::domain::error::AppError;
use crate::domain::ports::{Clipboard, SnippetStore};
use crate::domain::snippet::SnippetFrontmatter;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AddOutcome {
    pub key: String,
    pub path: PathBuf,
}

/// Validate that `raw_path` starts with `.mx/commands/` (with optional `./` prefix)
/// and return the relative portion after that prefix.
fn extract_relative_path(raw_path: &str) -> Result<PathBuf, AppError> {
    let normalized = raw_path.trim_start_matches("./");
    let stripped = normalized.strip_prefix(".mx/commands/").ok_or_else(|| {
        AppError::invalid_key(format!("Path must be under .mx/commands/ (got '{raw_path}')"))
    })?;

    if stripped.is_empty() {
        return Err(AppError::invalid_key("Path cannot be empty after .mx/commands/"));
    }

    let rel = Path::new(stripped);
    for component in rel.components() {
        use std::path::Component::*;
        match component {
            Normal(_) | CurDir => {}
            _ => {
                return Err(AppError::path_traversal(format!(
                    "Path contains unsafe segments: '{raw_path}'"
                )))
            }
        }
    }

    Ok(rel.to_path_buf())
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
        return Err(AppError::config_error(format!(
            "Snippet already exists: '{}'. Use --force to overwrite.",
            relative.display()
        )));
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
        assert!(matches!(err, AppError::ConfigError(_)));
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
        assert!(matches!(err, AppError::InvalidKey(_)));
    }
}

use crate::adapters::snippet_store::FilesystemSnippetStore;
use crate::app::api;

#[derive(clap::Args)]
pub struct Cli {
    pub path: String,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(short = 'f', long)]
    pub force: bool,
}

pub fn run(args: Cli) -> Result<(), AppError> {
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = api::add_snippet(&args.path, args.title.as_deref(), args.description.as_deref(), args.force, &store)?;
    println!("✅ Added snippet '{}' at {}", outcome.key, outcome.path.display());
    Ok(())
}
