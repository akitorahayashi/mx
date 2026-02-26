use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(key: &str, force: bool) -> Result<(), AppError> {
    let outcome = api::touch_context(key, force)?;

    if outcome.overwritten {
        println!("✅ Context file overwritten: {}", outcome.path.display());
    } else if outcome.existed {
        println!("⚠️ Context file already exists: {}", outcome.path.display());
    } else {
        println!("✅ Context file created: {}", outcome.path.display());
    }

    Ok(())
}
