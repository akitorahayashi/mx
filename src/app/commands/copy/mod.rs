use crate::domain::context_file::validate_path;
use crate::domain::error::AppError;
use crate::ports::{Clipboard, ContextFileStore, SnippetCatalog};
use std::borrow::Cow;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CopyOutcome {
    pub snippet: String,
    pub relative_path: String,
    pub absolute_path: std::path::PathBuf,
}

pub fn execute(
    snippet: &str,
    catalog: &dyn SnippetCatalog,
    clipboard: &dyn Clipboard,
    workspace_store: Option<&dyn ContextFileStore>,
) -> Result<CopyOutcome, AppError> {
    let snippet_entry = catalog.resolve_snippet(snippet)?;
    let content = fs::read_to_string(&snippet_entry.absolute_path)?;
    let expanded = expand_placeholders(&content, workspace_store);
    clipboard.copy(expanded.as_ref())?;

    Ok(CopyOutcome {
        snippet: snippet_entry.key,
        relative_path: snippet_entry.relative_path,
        absolute_path: snippet_entry.absolute_path,
    })
}

fn expand_placeholders<'a>(
    content: &'a str,
    workspace_store: Option<&dyn ContextFileStore>,
) -> Cow<'a, str> {
    let Some(store) = workspace_store else {
        return Cow::Borrowed(content);
    };

    if !content.contains("{{") {
        return Cow::Borrowed(content);
    }

    let mut remainder = content;
    let mut output = String::with_capacity(content.len());

    while let Some(start) = remainder.find("{{") {
        output.push_str(&remainder[..start]);
        let tail = &remainder[start + 2..];

        match tail.find("}}") {
            Some(end) => {
                let token = &tail[..end];
                output.push_str(&render_placeholder(token, store));
                remainder = &tail[end + 2..];
            }
            None => {
                output.push_str(&remainder[start..]);
                return Cow::Owned(output);
            }
        }
    }

    output.push_str(remainder);
    Cow::Owned(output)
}

fn render_placeholder(raw_token: &str, workspace_store: &dyn ContextFileStore) -> String {
    let trimmed = raw_token.trim();
    if trimmed.is_empty() {
        return format!("{{{{{raw_token}}}}}");
    }

    if let Err(err) = validate_path(trimmed, Path::new(trimmed)) {
        return format!("[mx error: {}]", err);
    }

    match workspace_store.read_workspace_file(Path::new(trimmed)) {
        Ok(contents) => contents,
        Err(err) => format!("[mx missing: {trimmed} ({})]", err.kind()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::snippet::SnippetEntry;
    use crate::testing::{InMemoryCatalog, InMemoryClipboard, InMemoryContextStore};
    use std::fs;
    use tempfile::TempDir;

    fn build_catalog_with_snippet(
        contents: &str,
    ) -> (InMemoryCatalog, TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let snippet_path = dir.path().join("commands/w/wc.md");
        fs::create_dir_all(snippet_path.parent().unwrap())
            .expect("snippet parent should be created");
        fs::write(&snippet_path, contents).expect("snippet file should be written");

        let catalog = InMemoryCatalog::new(vec![SnippetEntry {
            key: "wc".to_string(),
            relative_path: "w/wc".to_string(),
            absolute_path: snippet_path.clone(),
        }]);
        (catalog, dir, snippet_path)
    }

    #[test]
    fn execute_copies_snippet_with_placeholder_expansion() {
        let (catalog, _dir, snippet_path) = build_catalog_with_snippet("header {{.mx/info.md}}");
        let clipboard = InMemoryClipboard::default();
        let workspace_store = InMemoryContextStore::default();
        workspace_store.set_workspace_file(".mx/info.md", "expanded");

        let outcome = execute("wc", &catalog, &clipboard, Some(&workspace_store))
            .expect("copy command should succeed");

        assert_eq!(outcome.snippet, "wc");
        assert_eq!(outcome.relative_path, "w/wc");
        assert_eq!(outcome.absolute_path, snippet_path);
        assert_eq!(clipboard.contents(), "header expanded");
    }

    #[test]
    fn execute_preserves_placeholders_without_workspace_store() {
        let (catalog, _dir, _) = build_catalog_with_snippet("{{.mx/info.md}}");
        let clipboard = InMemoryClipboard::default();

        execute("wc", &catalog, &clipboard, None).expect("copy command should succeed");
        assert_eq!(clipboard.contents(), "{{.mx/info.md}}");
    }

    #[test]
    fn execute_surfaces_missing_snippet_errors() {
        let catalog = InMemoryCatalog::new(Vec::new());
        let clipboard = InMemoryClipboard::default();
        let workspace_store = InMemoryContextStore::default();

        let error = execute("unknown", &catalog, &clipboard, Some(&workspace_store))
            .expect_err("missing snippet should fail");
        assert!(matches!(error, AppError::NotFound(_)));
    }

    #[test]
    fn execute_marks_invalid_placeholder_as_error() {
        let (catalog, _dir, _) = build_catalog_with_snippet("{{../secret}}");
        let clipboard = InMemoryClipboard::default();
        let workspace_store = InMemoryContextStore::default();

        execute("wc", &catalog, &clipboard, Some(&workspace_store))
            .expect("copy command should succeed with placeholder marker");
        assert!(clipboard.contents().contains("[mx error:"));
    }
}
