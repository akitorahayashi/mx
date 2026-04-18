use crate::app::snippets;
use crate::error::AppError;
use crate::snippets::FilesystemSnippetStore;

pub(crate) fn run(
    path: &str,
    title: Option<&str>,
    description: Option<&str>,
    force: bool,
) -> Result<(), AppError> {
    let store = FilesystemSnippetStore::from_env()?;
    let outcome = snippets::add_snippet(path, title, description, force, &store)?;
    println!("✅ Added snippet '{}' at {}", outcome.key, outcome.path.display());
    Ok(())
}
