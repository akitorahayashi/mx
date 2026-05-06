use crate::app;
use crate::error::AppError;
use crate::snippets::FilesystemSnippetCatalog;

pub(crate) fn run(snippet: Option<&str>) -> Result<(), AppError> {
    let catalog = FilesystemSnippetCatalog::from_env()?;
    let outcome = app::which_path(snippet, &catalog, catalog.commands_root())?;
    println!("{}", outcome.path.display());
    Ok(())
}
