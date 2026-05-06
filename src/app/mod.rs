pub mod cat;
pub mod clean;
pub mod copy;
pub mod list;
pub mod touch;
pub mod which;

use crate::clipboard::clipboard_from_env;
use crate::context_files::ContextFileStore;
use crate::error::AppError;
use crate::project_fs::WorkspaceFileReader;
use crate::snippets::SnippetCatalog;
use std::path::Path;

pub use clean::CleanOutcome;
pub use copy::CopyOutcome;
pub use list::ListEntry;
pub use touch::TouchOutcome;
pub use which::WhichOutcome;

pub fn cat_context(key: &str, store: &impl ContextFileStore) -> Result<String, AppError> {
    cat::execute(key, store)
}

pub fn clean_context(
    key: Option<String>,
    store: &impl ContextFileStore,
) -> Result<CleanOutcome, AppError> {
    clean::execute(key, store)
}

pub fn copy_snippet(
    snippet: &str,
    catalog: &impl SnippetCatalog,
    workspace_files: Option<&impl WorkspaceFileReader>,
) -> Result<CopyOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    copy::execute(
        snippet,
        catalog,
        clipboard.as_ref(),
        workspace_files.map(|store| store as &dyn WorkspaceFileReader),
    )
}

pub fn list_snippets(catalog: &impl SnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    list::execute(catalog)
}

pub fn touch_context(
    key: &str,
    force: bool,
    store: &impl ContextFileStore,
) -> Result<TouchOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    touch::execute(key, force, store, clipboard.as_ref())
}

pub fn which_path(
    query: Option<&str>,
    catalog: &impl SnippetCatalog,
    commands_root: &Path,
) -> Result<WhichOutcome, AppError> {
    which::execute(query, catalog, commands_root)
}
