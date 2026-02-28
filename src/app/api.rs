use crate::adapters::clipboard::clipboard_from_env;
use crate::adapters::context_file_store::LocalContextFileStore;
use crate::adapters::snippet_checkout::SymlinkCheckout;
use crate::adapters::workspace_locator::CurrentDirectoryLocator;
use crate::app::commands;
use crate::domain::error::AppError;
use crate::domain::ports::{
    Clipboard, ContextFileStore, SnippetCatalog, SnippetStore, WorkspaceLocator,
};

pub use crate::app::commands::add::AddOutcome;
pub use crate::app::commands::checkout::CheckoutOutcome;
pub use crate::app::commands::clean::CleanOutcome;
pub use crate::app::commands::copy::CopyOutcome;
pub use crate::app::commands::create_command::CreateCommandOutcome;
pub use crate::app::commands::list::ListEntry;
pub use crate::app::commands::remove::RemoveOutcome;
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

pub fn copy_snippet(snippet: &str, catalog: &impl SnippetCatalog) -> Result<CopyOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    let workspace_store = find_workspace_root().ok().map(LocalContextFileStore::new);

    commands::copy::execute(
        snippet,
        catalog,
        clipboard.as_ref(),
        workspace_store.as_ref().map(|store| store as &dyn ContextFileStore),
    )
}

pub fn list_snippets(catalog: &impl SnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    commands::list::execute(catalog)
}

pub fn touch_context(key: &str, force: bool) -> Result<TouchOutcome, AppError> {
    let store = LocalContextFileStore::new(find_workspace_root()?);
    let clipboard = clipboard_from_env()?;
    touch_context_with_deps(key, force, &store, clipboard.as_ref())
}

pub fn checkout_snippets(
    query: Option<&str>,
    all: bool,
    catalog: &impl SnippetCatalog,
) -> Result<CheckoutOutcome, AppError> {
    let workspace_root = find_workspace_root()?;
    let target_root = workspace_root.join(".mx").join("commands");
    let checkout = SymlinkCheckout::new();
    commands::checkout::execute(query, all, catalog, &checkout, &target_root)
}

pub fn add_snippet(
    path: &str,
    title: Option<&str>,
    description: Option<&str>,
    force: bool,
    store: &impl SnippetStore,
) -> Result<AddOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    commands::add::execute(path, title, description, force, store, clipboard.as_ref())
}

pub fn remove_snippet(
    snippet: &str,
    catalog: &impl SnippetCatalog,
    store: &impl SnippetStore,
) -> Result<RemoveOutcome, AppError> {
    commands::remove::execute(snippet, catalog, store)
}

pub fn create_command(
    path: &str,
    force: bool,
    store: &impl SnippetStore,
) -> Result<CreateCommandOutcome, AppError> {
    commands::create_command::execute(path, force, store)
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
