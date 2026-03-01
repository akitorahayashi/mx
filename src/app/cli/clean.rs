use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(key: Option<String>, force: bool) -> Result<(), AppError> {
    let outcome = api::clean_context(key, force)?;
    println!("✅ {}", outcome.message);
    Ok(())
}
