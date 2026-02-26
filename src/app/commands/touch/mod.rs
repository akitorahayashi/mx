use crate::domain::context_file::resolve_validated_context_path;
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
    let relative_path = resolve_validated_context_path(key)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::context_file::resolve_context_path;
    use crate::ports::ContextFileStore;
    use crate::testing::{InMemoryClipboard, InMemoryContextStore};

    #[test]
    fn execute_creates_context_file_with_clipboard_content() {
        let store = InMemoryContextStore::default();
        let clipboard = InMemoryClipboard::default();
        clipboard.set_contents("fresh content");

        let outcome = execute("tk", false, &store, &clipboard).expect("touch should succeed");
        assert_eq!(outcome.key, "tk");
        assert!(!outcome.existed);
        assert!(!outcome.overwritten);
        assert!(outcome.path.ends_with(".mx/tasks.md"));

        let content = store.read_context_contents(&resolve_context_path("tk")).unwrap();
        assert_eq!(content, "fresh content");
    }

    #[test]
    fn execute_reports_existing_file_and_force_overwrite() {
        let store = InMemoryContextStore::default();
        let clipboard = InMemoryClipboard::default();
        clipboard.set_contents("initial");
        execute("tk", false, &store, &clipboard).unwrap();

        clipboard.set_contents("updated");
        let skipped = execute("tk", false, &store, &clipboard).unwrap();
        assert!(skipped.existed);
        assert!(!skipped.overwritten);
        assert_eq!(store.read_context_contents(&resolve_context_path("tk")).unwrap(), "initial");

        let forced = execute("tk", true, &store, &clipboard).unwrap();
        assert!(forced.existed);
        assert!(forced.overwritten);
        assert_eq!(store.read_context_contents(&resolve_context_path("tk")).unwrap(), "updated");
    }
}
