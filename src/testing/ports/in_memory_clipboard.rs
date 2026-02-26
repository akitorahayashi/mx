use crate::domain::error::AppError;
use crate::ports::Clipboard;
use std::cell::RefCell;

#[derive(Default)]
pub struct InMemoryClipboard {
    buffer: RefCell<String>,
}

impl InMemoryClipboard {
    pub fn contents(&self) -> String {
        self.buffer.borrow().clone()
    }

    pub fn set_contents(&self, value: &str) {
        self.buffer.replace(value.to_string());
    }
}

impl Clipboard for InMemoryClipboard {
    fn copy(&self, text: &str) -> Result<(), AppError> {
        self.buffer.replace(text.to_string());
        Ok(())
    }

    fn paste(&self) -> Result<String, AppError> {
        Ok(self.buffer.borrow().clone())
    }
}
