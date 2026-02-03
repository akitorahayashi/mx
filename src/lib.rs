//! Library entry point exposing the mx CLI command handlers.

pub mod error;

mod commands;
mod storage;

use commands::cat;
use commands::clean;
use commands::clipboard::clipboard_from_env;
use commands::copy::Copy;
use commands::list;
use commands::touch;
use commands::touch::find_project_root;
use error::AppError;

pub use commands::clean::CleanOutcome;
pub use commands::copy::CopyOutcome;
pub use commands::list::ListEntry;
pub use commands::touch::TouchOutcome;
pub use storage::SnippetStorage;

/// Displays the contents of a context file from the `.mx/` directory.
///
/// This command reuses the same path resolution logic as `touch`, supporting:
/// - Predefined aliases (tk, rq, pdt, etc.)
/// - Dynamic numbered aliases (tk1, tk2, etc.)
/// - Pending prefix (pd-tk, pd-rq, etc.)
/// - Custom relative paths with automatic .md extension
///
/// # Arguments
///
/// * `key` - The key to resolve to a file path (e.g., "tk", "rq", "docs/spec")
///
/// # Returns
///
/// The file contents as a String, or an error if:
/// - The file does not exist
/// - Path traversal is attempted
/// - The project root cannot be found
///
/// # Examples
///
/// ```no_run
/// use mx::cat_context;
///
/// // Read the tasks file
/// let content = cat_context("tk").expect("Failed to read tasks");
/// println!("{}", content);
/// ```
pub fn cat_context(key: &str) -> Result<String, AppError> {
    let root = find_project_root()?;
    cat::cat(&root, key)
}

pub fn clean_context(key: Option<String>) -> Result<CleanOutcome, AppError> {
    let root = find_project_root()?;
    clean::clean(&root, key)
}

pub fn copy_snippet(snippet: &str, storage: &SnippetStorage) -> Result<CopyOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    let root = find_project_root().ok();
    Copy { snippet }.execute(storage, clipboard.as_ref(), root.as_deref())
}

pub fn list_snippets(storage: &SnippetStorage) -> Result<Vec<ListEntry>, AppError> {
    list::list(storage)
}

pub fn touch_context(key: &str, force: bool) -> Result<TouchOutcome, AppError> {
    let root = find_project_root()?;
    let clipboard = clipboard_from_env()?;
    touch::touch(&root, key, force, clipboard.as_ref())
}
