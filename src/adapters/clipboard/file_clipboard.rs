use crate::domain::error::AppError;
use crate::domain::ports::Clipboard;
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
    use std::io;
    use tempfile::tempdir;

    #[test]
    fn file_clipboard_roundtrip() {
        let dir = tempdir().unwrap();
        let clip = FileClipboard::new(dir.path().join("clipboard.txt")).unwrap();
        clip.copy("abc").unwrap();
        assert_eq!(clip.paste().unwrap(), "abc");
    }

    #[test]
    fn file_clipboard_copy_error_when_path_is_directory() {
        let dir = tempdir().unwrap();
        // The path points to a directory, so writing to it will fail
        let clip = FileClipboard { path: dir.path().to_path_buf() };

        let result = clip.copy("abc");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other); // AppError::ClipboardError maps to Other
        let err_msg = err.to_string().to_lowercase();
        assert!(
            err_msg.contains("is a directory") ||
            err_msg.contains("access is denied") ||
            err_msg.contains("permission denied"),
            "Unexpected error message: {}", err_msg
        );
    }

    #[test]
    fn file_clipboard_paste_error_when_path_is_directory() {
        let dir = tempdir().unwrap();
        // The path points to a directory, so reading from it will fail
        let clip = FileClipboard { path: dir.path().to_path_buf() };

        let result = clip.paste();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Other); // AppError::ClipboardError maps to Other
        let err_msg = err.to_string().to_lowercase();
        assert!(
            err_msg.contains("is a directory") ||
            err_msg.contains("access is denied") ||
            err_msg.contains("permission denied"),
            "Unexpected error message: {}", err_msg
        );
    }
}
