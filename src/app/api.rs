use crate::adapters::clipboard::clipboard_from_env;
use crate::adapters::context_file_store::LocalContextFileStore;
use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::adapters::workspace_locator::CurrentDirectoryLocator;
use crate::app::commands;
use crate::domain::error::AppError;
use crate::ports::{Clipboard, ContextFileStore, WorkspaceLocator};

pub use crate::app::commands::clean::CleanOutcome;
pub use crate::app::commands::copy::CopyOutcome;
pub use crate::app::commands::list::ListEntry;
pub use crate::app::commands::touch::TouchOutcome;

fn find_workspace_root() -> Result<std::path::PathBuf, AppError> {
    CurrentDirectoryLocator.find_workspace_root()
}

pub fn cat_context(key: &str) -> Result<String, AppError> {
    let store = LocalContextFileStore::new(find_workspace_root()?);
    cat_context_with_store(key, &store)
}

pub fn clean_context(key: Option<String>) -> Result<CleanOutcome, AppError> {
    let store = LocalContextFileStore::new(find_workspace_root()?);
    clean_context_with_store(key, &store)
}

pub fn copy_snippet(
    snippet: &str,
    storage: &FilesystemSnippetCatalog,
) -> Result<CopyOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    let workspace_store = find_workspace_root().ok().map(LocalContextFileStore::new);

    commands::copy::execute(
        snippet,
        storage,
        clipboard.as_ref(),
        workspace_store.as_ref().map(|store| store as &dyn ContextFileStore),
    )
}

pub fn list_snippets(storage: &FilesystemSnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    commands::list::execute(storage)
}

pub fn touch_context(key: &str, force: bool) -> Result<TouchOutcome, AppError> {
    let store = LocalContextFileStore::new(find_workspace_root()?);
    let clipboard = clipboard_from_env()?;
    touch_context_with_deps(key, force, &store, clipboard.as_ref())
}

pub(crate) fn cat_context_with_store(
    key: &str,
    store: &dyn ContextFileStore,
) -> Result<String, AppError> {
    commands::cat::execute(key, store)
}

pub(crate) fn clean_context_with_store(
    key: Option<String>,
    store: &dyn ContextFileStore,
) -> Result<CleanOutcome, AppError> {
    commands::clean::execute(key, store)
}

pub(crate) fn touch_context_with_deps(
    key: &str,
    force: bool,
    store: &dyn ContextFileStore,
    clipboard: &dyn Clipboard,
) -> Result<TouchOutcome, AppError> {
    commands::touch::execute(key, force, store, clipboard)
}
