use crate::app::context_files;
use crate::context_files::LocalContextFileStore;
use crate::error::AppError;
use crate::project_fs::{CurrentDirectoryLocator, WorkspaceLocator};

pub(crate) fn run(key: &str, force: bool) -> Result<(), AppError> {
    let workspace_root = CurrentDirectoryLocator.find_workspace_root()?;
    let store = LocalContextFileStore::new(workspace_root);
    let outcome = context_files::touch_context(key, force, &store)?;

    if outcome.overwritten {
        println!("✅ Context file overwritten: {}", outcome.path.display());
    } else if outcome.existed {
        println!("⚠️ Context file already exists: {}", outcome.path.display());
    } else {
        println!("✅ Context file created: {}", outcome.path.display());
    }

    Ok(())
}
