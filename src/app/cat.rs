use crate::context_files::resolve_validated_context_path;
use crate::context_files::ContextFileStore;
use crate::error::AppError;

pub fn execute(key: &str, store: &dyn ContextFileStore) -> Result<String, AppError> {
    let relative_path = resolve_validated_context_path(key)?;
    store.read_context_contents(&relative_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_files::resolve_context_path;
    use crate::context_files::ContextFileStore;
    use crate::context_files::InMemoryContextStore;

    #[test]
    fn execute_reads_existing_context_file() {
        let store = InMemoryContextStore::default();
        let relative_path = resolve_context_path("tk");
        let safe_path = crate::project_fs::SafePath::try_from_path(&relative_path).unwrap();
        let status =
            store.prepare_context_file(&safe_path, false).expect("context file should be prepared");
        store
            .write_context_contents(&status.path, "task body")
            .expect("context file should be written");

        let content = execute("tk", &store).expect("cat command should succeed");
        assert_eq!(content, "task body");
    }

    #[test]
    fn execute_rejects_path_traversal() {
        let store = InMemoryContextStore::default();
        let result = execute("../secret", &store);
        assert!(matches!(
            result,
            Err(AppError::PathTraversal(crate::error::PathTraversalError::Detected(_)))
        ));
    }
}
