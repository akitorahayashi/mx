use crate::context_files::resolve_validated_context_path;
use crate::context_files::ContextFileStore;
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct CleanOutcome {
    pub message: String,
}

pub fn execute(
    key: Option<String>,
    store: &dyn ContextFileStore,
) -> Result<CleanOutcome, AppError> {
    match key {
        None => {
            if store.remove_context_root()? {
                Ok(CleanOutcome { message: "Cleared .mx directory contents".to_string() })
            } else {
                Ok(CleanOutcome { message: ".mx directory not found".to_string() })
            }
        }
        Some(key) => {
            let relative_path = resolve_validated_context_path(&key)?;
            let target_path = store.remove_context_file(&relative_path)?;
            Ok(CleanOutcome { message: format!("Removed {}", target_path.display()) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_files::resolve_context_path;
    use crate::context_files::test_support::InMemoryContextStore;
    use crate::context_files::ContextFileStore;

    #[test]
    fn execute_removes_root_when_no_key_is_provided() {
        let store = InMemoryContextStore::default();
        let relative_path = resolve_context_path("tk");
        let safe_path = crate::project_fs::SafePath::try_from_path(&relative_path).unwrap();
        let status = store.prepare_context_file(&safe_path, false).unwrap();
        store.write_context_contents(&status.path, "content").unwrap();

        let outcome = execute(None, &store).expect("clean command should succeed");
        assert_eq!(outcome.message, "Cleared .mx directory contents");
    }

    #[test]
    fn execute_removes_specific_context_file() {
        let store = InMemoryContextStore::default();
        let relative_path = resolve_context_path("tk");
        let safe_path = crate::project_fs::SafePath::try_from_path(&relative_path).unwrap();
        let status = store.prepare_context_file(&safe_path, false).unwrap();
        store.write_context_contents(&status.path, "content").unwrap();

        let outcome =
            execute(Some("tk".to_string()), &store).expect("targeted clean should succeed");
        assert!(outcome.message.contains(".mx/tasks.md"));
    }
}
