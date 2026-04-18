use crate::app::snippets;
use crate::error::AppError;
use crate::snippets::FilesystemSnippetStore;

pub(crate) fn run(path: &str, force: bool) -> Result<(), AppError> {
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = snippets::create_command(path, force, &store)?;
    println!("Created command template: {} ({})", outcome.key, outcome.path.display());
    Ok(())
}
