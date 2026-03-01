use crate::domain::context_file::resolve_validated_context_path;
use crate::domain::error::AppError;
use crate::domain::ports::ContextFileStore;

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

        let outcome = execute(None, &store).expect("clean command should succeed");
        assert_eq!(outcome.message, "Removed .mx directory");
    }

    #[test]
    fn execute_removes_specific_context_file() {
        let store = InMemoryContextStore::default();
        let relative_path = resolve_context_path("tk");
        let status = store.prepare_context_file(&relative_path, false).unwrap();
        store.write_context_contents(&status.path, "content").unwrap();

        let outcome =
            execute(Some("tk".to_string()), &store).expect("targeted clean should succeed");
        assert!(outcome.message.contains(".mx/tasks.md"));
    }
}

use crate::app::api;

#[derive(clap::Args)]
pub struct Cli {
    pub key: Option<String>,
}

pub fn run(args: Cli) -> Result<(), crate::domain::error::AppError> {
    let outcome = api::clean_context(args.key)?;
    println!("✅ {}", outcome.message);
    Ok(())
}
