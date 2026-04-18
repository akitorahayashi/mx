use crate::error::AppError;

pub trait Clipboard {
    fn copy(&self, text: &str) -> Result<(), AppError>;
    fn paste(&self) -> Result<String, AppError>;
}
