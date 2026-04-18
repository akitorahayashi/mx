use crate::clipboard::Clipboard;
use crate::error::AppError;
use crate::project_fs::{SafePath, WorkspaceFileReader};
use crate::snippets::{strip_frontmatter, SnippetCatalog};
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
    workspace_files: Option<&dyn WorkspaceFileReader>,
) -> Result<CopyOutcome, AppError> {
    let snippet_entry = catalog.resolve_snippet(snippet)?;
    let raw = fs::read_to_string(&snippet_entry.absolute_path)?;
    let content = strip_frontmatter(&raw);
    let expanded = expand_placeholders(content, workspace_files);
    clipboard.copy(expanded.as_ref())?;

    Ok(CopyOutcome {
        snippet: snippet_entry.key,
        relative_path: snippet_entry.relative_path,
        absolute_path: snippet_entry.absolute_path,
    })
}

fn expand_placeholders<'a>(
    content: &'a str,
    workspace_files: Option<&dyn WorkspaceFileReader>,
) -> Cow<'a, str> {
    let Some(store) = workspace_files else {
        return Cow::Borrowed(content);
    };

    if !content.contains("{{") {
        return Cow::Borrowed(content);
    }

    let mut output = String::with_capacity(content.len());
    let mut parts = content.split("{{");

    if let Some(first) = parts.next() {
        output.push_str(first);
    } else {
        return Cow::Borrowed(content);
    }

    for part in parts {
        if let Some((token, rest)) = part.split_once("}}") {
            output.push_str(&render_placeholder(token, store));
            output.push_str(rest);
        } else {
            output.push_str("{{");
            output.push_str(part);
        }
    }

    Cow::Owned(output)
}

fn render_placeholder(raw_token: &str, workspace_files: &dyn WorkspaceFileReader) -> String {
    let trimmed = raw_token.trim();
    if trimmed.is_empty() {
        return format!("{{{{{raw_token}}}}}");
    }

    let safe_path = match SafePath::try_from_path(Path::new(trimmed)) {
        Ok(path) => path,
        Err(err) => return format!("[mx error: {}]", err),
    };

    match workspace_files.read_workspace_file(&safe_path) {
        Ok(contents) => contents,
        Err(err) => format!("[mx missing: {trimmed} ({})]", err.kind()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clipboard::test_support::InMemoryClipboard;
    use crate::project_fs::test_support::InMemoryWorkspaceFileReader;
    use crate::snippets::test_support::InMemoryCatalog;
    use crate::snippets::SnippetEntry;
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
        let workspace_files = InMemoryWorkspaceFileReader::default();
        workspace_files.set_file(".mx/info.md", "expanded");

        let outcome = execute("wc", &catalog, &clipboard, Some(&workspace_files))
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
        let workspace_files = InMemoryWorkspaceFileReader::default();

        let error = execute("unknown", &catalog, &clipboard, Some(&workspace_files))
            .expect_err("missing snippet should fail");
        assert!(matches!(error, AppError::NotFound(crate::error::NotFoundError::Snippet(_))));
    }

    #[test]
    fn execute_marks_invalid_placeholder_as_error() {
        let (catalog, _dir, _) = build_catalog_with_snippet("{{../secret}}");
        let clipboard = InMemoryClipboard::default();
        let workspace_files = InMemoryWorkspaceFileReader::default();

        execute("wc", &catalog, &clipboard, Some(&workspace_files))
            .expect("copy command should succeed with placeholder marker");
        assert!(clipboard.contents().contains("[mx error:"));
    }

    #[test]
    fn execute_keeps_unclosed_placeholder_literal() {
        let (catalog, _dir, _) = build_catalog_with_snippet("prefix {{.mx/info.md");
        let clipboard = InMemoryClipboard::default();
        let workspace_files = InMemoryWorkspaceFileReader::default();
        workspace_files.set_file(".mx/info.md", "expanded");

        execute("wc", &catalog, &clipboard, Some(&workspace_files))
            .expect("copy command should succeed");
        assert_eq!(clipboard.contents(), "prefix {{.mx/info.md");
    }

    #[test]
    fn execute_strips_frontmatter_before_copy() {
        let (catalog, _dir, _) =
            build_catalog_with_snippet("---\ntitle: My Snippet\n---\nbody only\n");
        let clipboard = InMemoryClipboard::default();

        execute("wc", &catalog, &clipboard, None).expect("copy should succeed");
        assert_eq!(clipboard.contents(), "body only\n");
    }

    #[test]
    fn execute_strips_frontmatter_then_expands_placeholders() {
        let (catalog, _dir, _) =
            build_catalog_with_snippet("---\ntitle: T\n---\nheader {{.mx/info.md}}");
        let clipboard = InMemoryClipboard::default();
        let workspace_files = InMemoryWorkspaceFileReader::default();
        workspace_files.set_file(".mx/info.md", "injected");

        execute("wc", &catalog, &clipboard, Some(&workspace_files)).expect("copy should succeed");
        assert_eq!(clipboard.contents(), "header injected");
    }

    #[test]
    fn execute_unaffected_when_no_frontmatter() {
        let (catalog, _dir, _) = build_catalog_with_snippet("plain body\n");
        let clipboard = InMemoryClipboard::default();

        execute("wc", &catalog, &clipboard, None).expect("copy should succeed");
        assert_eq!(clipboard.contents(), "plain body\n");
    }
}
