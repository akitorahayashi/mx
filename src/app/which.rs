use crate::error::AppError;
use crate::snippets::SnippetCatalog;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WhichOutcome {
    pub path: PathBuf,
}

pub fn execute(
    query: Option<&str>,
    catalog: &dyn SnippetCatalog,
    commands_root: &Path,
) -> Result<WhichOutcome, AppError> {
    match query {
        Some(snippet) => {
            let entry = catalog.resolve_snippet(snippet)?;
            let path = absolute_path(&entry.absolute_path)?;
            Ok(WhichOutcome { path })
        }
        None => {
            let path = absolute_path(commands_root)?;
            Ok(WhichOutcome { path })
        }
    }
}

fn absolute_path(path: &Path) -> Result<PathBuf, AppError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    Ok(std::env::current_dir()?.join(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snippets::{InMemoryCatalog, SnippetEntry};
    use std::path::PathBuf;

    #[test]
    fn execute_returns_commands_root_when_query_absent() {
        let catalog = InMemoryCatalog::new(Vec::new());
        let commands_root = PathBuf::from("/tmp/commands");
        let outcome = execute(None, &catalog, &commands_root).expect("which should succeed");
        assert_eq!(outcome.path, commands_root);
    }

    #[test]
    fn execute_resolves_snippet_when_query_present() {
        let snippet_path = PathBuf::from("/tmp/commands/w/wc.md");
        let catalog = InMemoryCatalog::new(vec![SnippetEntry {
            key: "wc".to_string(),
            relative_path: "w/wc".to_string(),
            absolute_path: snippet_path.clone(),
        }]);

        let outcome =
            execute(Some("wc"), &catalog, Path::new("/tmp/commands")).expect("should succeed");
        assert_eq!(outcome.path, snippet_path);
    }
}
