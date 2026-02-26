use crate::domain::context_file::{resolve_context_path, validate_path};
use crate::domain::error::AppError;
use crate::ports::ContextFileStore;

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
            let relative_path = resolve_context_path(&key);
            validate_path(&key, &relative_path)?;
            let target_path = store.remove_context_file(&relative_path)?;
            Ok(CleanOutcome { message: format!("Removed {}", target_path.display()) })
        }
    }
}
