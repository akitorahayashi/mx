use crate::adapters::context_file_store::LocalContextFileStore;
use crate::adapters::workspace_locator::CurrentDirectoryLocator;
use crate::app::api;
use crate::domain::error::AppError;
use crate::domain::ports::WorkspaceLocator;

pub(crate) fn run(key: Option<String>) -> Result<(), AppError> {
    let workspace_root = CurrentDirectoryLocator.find_workspace_root()?;
    let store = LocalContextFileStore::new(workspace_root);
    let outcome = api::clean_context(key, &store)?;
    println!("✅ {}", outcome.message);
    Ok(())
}
