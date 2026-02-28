use crate::adapters::snippet_store::FilesystemSnippetStore;
use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(path: &str, force: bool) -> Result<(), AppError> {
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = api::create_command(path, force, &store)?;
    println!("Created command template: {} ({})", outcome.key, outcome.path.display());
    Ok(())
}
