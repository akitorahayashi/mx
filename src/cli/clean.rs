use crate::app::context_files;
use crate::context_files::LocalContextFileStore;
use crate::error::AppError;
use crate::project_fs::{CurrentDirectoryLocator, WorkspaceLocator};

pub(crate) fn run(key: Option<String>) -> Result<(), AppError> {
    let workspace_root = CurrentDirectoryLocator.find_workspace_root()?;
    let store = LocalContextFileStore::new(workspace_root);
    let outcome = context_files::clean_context(key, &store)?;
    println!("✅ {}", outcome.message);
    Ok(())
}
