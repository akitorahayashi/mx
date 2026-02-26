use crate::domain::context_file::{resolve_context_path, validate_path};
use crate::domain::error::AppError;
use crate::ports::ContextFileStore;

pub fn execute(key: &str, store: &dyn ContextFileStore) -> Result<String, AppError> {
    let relative_path = resolve_context_path(key);
    validate_path(key, &relative_path)?;
    store.read_context_contents(&relative_path)
}
