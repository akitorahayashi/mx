use crate::domain::error::AppError;
use crate::ports::Clipboard;
use std::fs;
use std::path::PathBuf;

pub struct FileClipboard {
    path: PathBuf,
}

impl FileClipboard {
    pub fn new(path: PathBuf) -> Result<Self, AppError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(Self { path })
    }
}

impl Clipboard for FileClipboard {
    fn copy(&self, text: &str) -> Result<(), AppError> {
        fs::write(&self.path, text).map_err(|err| AppError::clipboard_error(err.to_string()))
    }

    fn paste(&self) -> Result<String, AppError> {
        match fs::read_to_string(&self.path) {
            Ok(content) => Ok(content),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
            Err(err) => Err(AppError::clipboard_error(err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn file_clipboard_roundtrip() {
        let dir = tempdir().unwrap();
        let clip = FileClipboard::new(dir.path().join("clipboard.txt")).unwrap();
        clip.copy("abc").unwrap();
        assert_eq!(clip.paste().unwrap(), "abc");
    }
}
