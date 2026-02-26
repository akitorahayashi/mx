use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(key: Option<String>) -> Result<(), AppError> {
    let outcome = api::clean_context(key)?;
    println!("✅ {}", outcome.message);
    Ok(())
}
