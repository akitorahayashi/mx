use crate::app::snippets;
use crate::error::AppError;
use crate::project_fs::{CurrentDirectoryLocator, LocalWorkspaceFileReader, WorkspaceLocator};
use crate::snippets::FilesystemSnippetCatalog;

pub(crate) fn run(snippet: &str) -> Result<(), AppError> {
    let storage = FilesystemSnippetCatalog::from_env()?;
    let workspace_store =
        CurrentDirectoryLocator.find_workspace_root().ok().map(LocalWorkspaceFileReader::new);
    let outcome = snippets::copy_snippet(snippet, &storage, workspace_store.as_ref())?;

    println!(
        "✅ Copied '{}' from {} -> {}",
        outcome.snippet,
        outcome.relative_path,
        outcome.absolute_path.display()
    );
    Ok(())
}
