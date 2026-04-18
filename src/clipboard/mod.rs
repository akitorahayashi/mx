mod contract;
mod file_clipboard;
mod system_clipboard;

#[cfg(test)]
mod in_memory_clipboard;
#[cfg(test)]
pub mod test_support;

use crate::error::AppError;
use std::env;
use std::path::PathBuf;

pub use contract::Clipboard;
pub use file_clipboard::FileClipboard;
pub use system_clipboard::SystemClipboard;

pub fn clipboard_from_env() -> Result<Box<dyn Clipboard>, AppError> {
    if let Ok(path) = env::var("MX_CLIPBOARD_FILE") {
        return Ok(Box::new(FileClipboard::new(PathBuf::from(path))?));
    }

    Ok(Box::new(SystemClipboard::detect()?))
}
