use crate::app;
use crate::context_files::LocalContextFileStore;
use crate::error::AppError;
use crate::project_fs::{CurrentDirectoryLocator, WorkspaceLocator};

pub(crate) fn run(key: &str) -> Result<(), AppError> {
    let workspace_root = CurrentDirectoryLocator.find_workspace_root()?;
    let store = LocalContextFileStore::new(workspace_root);
    let content = app::cat_context(key, &store)?;
    print!("{}", content);
    Ok(())
}
