use crate::adapters::context_file_store::LocalContextFileStore;
use crate::adapters::workspace_locator::CurrentDirectoryLocator;
use crate::app::api;
use crate::domain::error::AppError;
use crate::domain::ports::WorkspaceLocator;

pub(crate) fn run(key: &str, force: bool) -> Result<(), AppError> {
    let workspace_root = CurrentDirectoryLocator.find_workspace_root()?;
    let store = LocalContextFileStore::new(workspace_root);
    let outcome = api::touch_context(key, force, &store)?;

    if outcome.overwritten {
        println!("✅ Context file overwritten: {}", outcome.path.display());
    } else if outcome.existed {
        println!("⚠️ Context file already exists: {}", outcome.path.display());
    } else {
        println!("✅ Context file created: {}", outcome.path.display());
    }

    Ok(())
}
