pub mod cat;
pub mod clean;
pub mod touch;

use crate::clipboard::clipboard_from_env;
use crate::context_files::ContextFileStore;
use crate::error::AppError;

pub use clean::CleanOutcome;
pub use touch::TouchOutcome;

pub fn cat_context(key: &str, store: &impl ContextFileStore) -> Result<String, AppError> {
    cat::execute(key, store)
}

pub fn clean_context(
    key: Option<String>,
    store: &impl ContextFileStore,
) -> Result<CleanOutcome, AppError> {
    clean::execute(key, store)
}

pub fn touch_context(
    key: &str,
    force: bool,
    store: &impl ContextFileStore,
) -> Result<TouchOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    touch::execute(key, force, store, clipboard.as_ref())
}
