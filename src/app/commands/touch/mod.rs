use crate::domain::context_file::{resolve_context_path, validate_path};
use crate::domain::error::AppError;
use crate::ports::{Clipboard, ContextFileStore};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TouchOutcome {
    pub key: String,
    pub path: PathBuf,
    pub existed: bool,
    pub overwritten: bool,
}

pub fn execute(
    key: &str,
    force: bool,
    store: &dyn ContextFileStore,
    clipboard: &dyn Clipboard,
) -> Result<TouchOutcome, AppError> {
    let relative_path = resolve_context_path(key);
    validate_path(key, &relative_path)?;

    let status = store.prepare_context_file(&relative_path, force)?;
    if status.should_write() {
        let content = clipboard.paste()?;
        store.write_context_contents(&status.path, &content)?;
    }

    Ok(TouchOutcome {
        key: key.to_string(),
        path: status.path,
        existed: status.existed,
        overwritten: status.overwritten,
    })
}
