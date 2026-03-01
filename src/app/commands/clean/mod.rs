use crate::domain::context_file::resolve_validated_context_path;
use crate::domain::error::AppError;
use crate::domain::ports::ContextFileStore;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct CleanOutcome {
    pub message: String,
}

pub fn execute(
    key: Option<String>,
    force: bool,
    store: &dyn ContextFileStore,
) -> Result<CleanOutcome, AppError> {
    if !force {
        let target = match &key {
            Some(ref k) => format!("context file '{}'", k),
            None => ".mx directory".to_string(),
        };
        print!("Are you sure you want to remove {}? [y/N] ", target);
        io::stdout().flush().map_err(AppError::Io)?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(AppError::Io)?;
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            return Err(AppError::aborted("Operation cancelled by user"));
        }
    }

    match key {
        None => {
            if store.remove_context_root()? {
                Ok(CleanOutcome { message: "Removed .mx directory".to_string() })
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
    use crate::domain::context_file::resolve_context_path;
    use crate::domain::ports::ContextFileStore;
    use crate::testing::InMemoryContextStore;

    #[test]
    fn execute_removes_root_when_no_key_is_provided() {
        let store = InMemoryContextStore::default();
        let relative_path = resolve_context_path("tk");
        let status = store.prepare_context_file(&relative_path, false).unwrap();
        store.write_context_contents(&status.path, "content").unwrap();

        let outcome = execute(None, true, &store).expect("clean command should succeed");
        assert_eq!(outcome.message, "Removed .mx directory");
    }

    #[test]
    fn execute_removes_specific_context_file() {
        let store = InMemoryContextStore::default();
        let relative_path = resolve_context_path("tk");
        let status = store.prepare_context_file(&relative_path, false).unwrap();
        store.write_context_contents(&status.path, "content").unwrap();

        let outcome =
            execute(Some("tk".to_string()), true, &store).expect("targeted clean should succeed");
        assert!(outcome.message.contains(".mx/tasks.md"));
    }
}
