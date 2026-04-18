use crate::app::snippets;
use crate::error::AppError;
use crate::snippets::{FilesystemSnippetCatalog, FilesystemSnippetStore};

pub(crate) fn run(snippet: &str) -> Result<(), AppError> {
    let catalog = FilesystemSnippetCatalog::from_env()?;
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = snippets::remove_snippet(snippet, &catalog, &store)?;
    println!("✅ Removed snippet '{}' from {}", outcome.key, outcome.path.display());
    Ok(())
}
