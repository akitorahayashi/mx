mod file_clipboard;
mod system_clipboard;

use crate::domain::error::AppError;
use crate::domain::ports::Clipboard;
use std::env;
use std::path::PathBuf;

pub use file_clipboard::FileClipboard;
pub use system_clipboard::SystemClipboard;

pub fn clipboard_from_env() -> Result<Box<dyn Clipboard>, AppError> {
    if let Ok(path) = env::var("MX_CLIPBOARD_FILE") {
        return Ok(Box::new(FileClipboard::new(PathBuf::from(path))?));
    }

    Ok(Box::new(SystemClipboard::detect()?))
}
