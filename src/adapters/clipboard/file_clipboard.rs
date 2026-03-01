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
    use tempfile::tempdir;

    #[test]
    fn file_clipboard_roundtrip() {
        let dir = tempdir().unwrap();
        let clip = FileClipboard::new(dir.path().join("clipboard.txt")).unwrap();
        clip.copy("abc").unwrap();
        assert_eq!(clip.paste().unwrap(), "abc");
    }

    #[test]
    fn file_clipboard_paste_non_existent() {
        let dir = tempdir().unwrap();
        let clip = FileClipboard::new(dir.path().join("non_existent.txt")).unwrap();
        assert_eq!(clip.paste().unwrap(), "");
    }

    #[test]
    fn file_clipboard_paste_empty() {
        let dir = tempdir().unwrap();
        let clip = FileClipboard::new(dir.path().join("empty.txt")).unwrap();
        clip.copy("").unwrap();
        assert_eq!(clip.paste().unwrap(), "");
    }

    #[test]
    fn file_clipboard_copy_invalid_path() {
        let dir = tempdir().unwrap();
        // Use a path that is actually a directory so that write fails
        let clip = FileClipboard::new(dir.path().to_path_buf()).unwrap();
        let result = clip.copy("data");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
    }

    #[test]
    fn file_clipboard_paste_error() {
        let dir = tempdir().unwrap();
        // Use a path that is actually a directory so that read fails
        let clip = FileClipboard::new(dir.path().to_path_buf()).unwrap();
        let result = clip.paste();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
    }
}
