use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(key: &str) -> Result<(), AppError> {
    let content = api::cat_context(key)?;
    print!("{}", content);
    Ok(())
}
