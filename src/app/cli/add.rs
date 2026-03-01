use crate::adapters::snippet_store::FilesystemSnippetStore;
use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(
    path: &str,
    title: Option<&str>,
    description: Option<&str>,
    force: bool,
) -> Result<(), AppError> {
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = api::add_snippet(path, title, description, force, &store)?;
    println!("✅ Added snippet '{}' at {}", outcome.key, outcome.path.display());
    Ok(())
}
