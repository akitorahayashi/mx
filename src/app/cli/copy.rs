use crate::adapters::context_file_store::LocalContextFileStore;
use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::adapters::workspace_locator::CurrentDirectoryLocator;
use crate::app::api;
use crate::domain::error::AppError;
use crate::domain::ports::WorkspaceLocator;

pub(crate) fn run(snippet: &str) -> Result<(), AppError> {
    let storage = FilesystemSnippetCatalog::from_env()?;
    let workspace_store = CurrentDirectoryLocator
        .find_workspace_root()
        .ok()
        .map(LocalContextFileStore::new);
    let api::CopyOutcome { snippet: snippet_key, relative_path, absolute_path } =
        api::copy_snippet(snippet, &storage, workspace_store.as_ref())?;

    println!("✅ Copied '{snippet_key}' from {relative_path} -> {}", absolute_path.display());
    Ok(())
}
