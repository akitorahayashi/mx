use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("{0}")]
    ConfigError(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    ClipboardError(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("{0}")]
    PathTraversal(String),

    #[error("Aborted: {0}")]
    Aborted(String),
}

impl AppError {
    pub(crate) fn config_error<S: Into<String>>(message: S) -> Self {
        Self::ConfigError(message.into())
    }

    pub(crate) fn not_found<S: Into<String>>(message: S) -> Self {
        Self::NotFound(message.into())
    }

    pub(crate) fn clipboard_error<S: Into<String>>(message: S) -> Self {
        Self::ClipboardError(message.into())
    }

    pub fn invalid_key<S: Into<String>>(message: S) -> Self {
        Self::InvalidKey(message.into())
    }

    pub fn path_traversal<S: Into<String>>(message: S) -> Self {
        Self::PathTraversal(message.into())
    }

    pub fn aborted<S: Into<String>>(message: S) -> Self {
        Self::Aborted(message.into())
    }

    pub fn kind(&self) -> io::ErrorKind {
        match self {
            Self::Io(err) => err.kind(),
            Self::ConfigError(_) => io::ErrorKind::InvalidInput,
            Self::NotFound(_) => io::ErrorKind::NotFound,
            Self::ClipboardError(_) => io::ErrorKind::Other,
            Self::InvalidKey(_) => io::ErrorKind::InvalidInput,
            Self::PathTraversal(_) => io::ErrorKind::InvalidInput,
            Self::Aborted(_) => io::ErrorKind::Other,
        }
    }
}
