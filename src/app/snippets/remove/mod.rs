use crate::error::{AppError, PathTraversalError};
use crate::project_fs::SafePath;
use crate::snippets::{SnippetCatalog, SnippetStore};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RemoveOutcome {
    pub key: String,
    pub path: PathBuf,
}

pub fn execute(
    snippet: &str,
    catalog: &dyn SnippetCatalog,
    store: &dyn SnippetStore,
) -> Result<RemoveOutcome, AppError> {
    let entry = catalog.resolve_snippet(snippet)?;
    let relative = std::path::Path::new(&entry.relative_path).with_extension("md");
    let safe_path = SafePath::try_from_path(&relative).map_err(|_| {
        AppError::PathTraversal(PathTraversalError::Detected(format!(
            "Path contains unsafe segments: '{}'",
            relative.display()
        )))
    })?;
    let path = store.remove_snippet(&safe_path)?;
    Ok(RemoveOutcome { key: entry.key, path })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snippets::test_support::{InMemoryCatalog, InMemorySnippetStore};
    use crate::snippets::SnippetEntry;
    use std::path::PathBuf;

    fn entry(key: &str, rel: &str) -> SnippetEntry {
        SnippetEntry {
            key: key.to_string(),
            relative_path: rel.to_string(),
            absolute_path: PathBuf::from(format!("/fake/{rel}.md")),
        }
    }

    #[test]
    fn removes_existing_snippet() {
        let catalog = InMemoryCatalog::new(vec![entry("wc", "w/wc")]);
        let store = InMemorySnippetStore::new();
        store.seed("w/wc.md", "content");

        let outcome = execute("wc", &catalog, &store).expect("remove should succeed");
        assert_eq!(outcome.key, "wc");
        assert!(!store.has("w/wc.md"));
    }

    #[test]
    fn fails_for_nonexistent_snippet() {
        let catalog = InMemoryCatalog::new(vec![]);
        let store = InMemorySnippetStore::new();

        let err = execute("missing", &catalog, &store).expect_err("should fail");
        assert!(matches!(err, AppError::NotFound(crate::error::NotFoundError::Snippet(_))));
    }
}
