use std::io;

/// Library-wide error type capturing domain-neutral and underlying I/O failures.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Configuration or environment issue that prevents command execution.
    #[error("{0}")]
    ConfigError(String),

    /// Raised when a requested resource cannot be located.
    #[error("{0}")]
    NotFound(String),

    /// Clipboard interaction failure surfaced to the user.
    #[error("{0}")]
    ClipboardError(String),

    /// Invalid key provided for touch command
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Path traversal attempt detected (security violation)
    #[error("{0}")]
    PathTraversal(String),
}

impl AppError {
    pub(crate) fn config_error<S: Into<String>>(message: S) -> Self {
        AppError::ConfigError(message.into())
    }

    /// Provide an `io::ErrorKind`-like view for callers expecting legacy behavior.
    pub fn kind(&self) -> io::ErrorKind {
        match self {
            AppError::Io(err) => err.kind(),
            AppError::ConfigError(_) => io::ErrorKind::InvalidInput,
            AppError::NotFound(_) => io::ErrorKind::NotFound,
            AppError::ClipboardError(_) => io::ErrorKind::Other,
            AppError::InvalidKey(_) => io::ErrorKind::InvalidInput,
            AppError::PathTraversal(_) => io::ErrorKind::InvalidInput,
        }
    }

    pub(crate) fn not_found<S: Into<String>>(message: S) -> Self {
        AppError::NotFound(message.into())
    }

    pub(crate) fn clipboard_error<S: Into<String>>(message: S) -> Self {
        AppError::ClipboardError(message.into())
    }

    pub fn invalid_key<S: Into<String>>(message: S) -> Self {
        AppError::InvalidKey(message.into())
    }

    pub fn path_traversal<S: Into<String>>(message: S) -> Self {
        AppError::PathTraversal(message.into())
    }
}
