use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::adapters::snippet_store::FilesystemSnippetStore;
use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(snippet: &str, force: bool) -> Result<(), AppError> {
    let catalog = FilesystemSnippetCatalog::from_env()?;
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = api::remove_snippet(snippet, force, &catalog, &store)?;
    println!("✅ Removed snippet '{}' from {}", outcome.key, outcome.path.display());
    Ok(())
}
