use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    ClipboardError(#[from] ClipboardError),

    #[error(transparent)]
    InvalidKey(#[from] InvalidKeyError),

    #[error(transparent)]
    PathTraversal(#[from] PathTraversalError),
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Duplicate snippet: {0}")]
    DuplicateSnippet(String),
    #[error("Empty snippet name")]
    EmptySnippetName,
    #[error("Invalid UTF-8 in snippet path")]
    InvalidUtf8,
    #[error("Unable to derive relative path for: {0}")]
    RelativePathDerivation(String),
    #[error("File system error: {0}")]
    Io(String),
    #[error("Configuration error: {0}")]
    Other(String),
}

#[derive(thiserror::Error, Debug)]
pub enum NotFoundError {
    #[error("Snippet not found: {0}")]
    Snippet(String),
    #[error("Context file not found: {0}")]
    ContextFile(String),
    #[error("File not found: {0}")]
    File(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ClipboardError {
    #[error("Command not found or empty: {0}")]
    CommandMissing(String),
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Command exited with non-zero status ({0}). Stderr: {1}")]
    NonZeroExit(i32, String),
    #[error("Invalid UTF-8 in clipboard content: {0}")]
    InvalidUtf8(String),
    #[error("Clipboard Error: {0}")]
    Other(String),
}

#[derive(thiserror::Error, Debug)]
pub enum InvalidKeyError {
    #[error("Path must be under {expected} (got '{actual}')")]
    NotInCommands { expected: String, actual: String },
    #[error("Path cannot be empty after {0}")]
    EmptyAfterPrefix(String),
    #[error("Path must end with a valid filename: '{0}'")]
    NoFilename(String),
    #[error("Directory already exists for key: {0}")]
    DirectoryExists(String),
    #[error("Invalid key: {0}")]
    Other(String),
}

#[derive(thiserror::Error, Debug)]
pub enum PathTraversalError {
    #[error("Path traversal detected: {0}")]
    Detected(String),
}

impl AppError {
    pub fn kind(&self) -> io::ErrorKind {
        match self {
            Self::Io(err) => err.kind(),
            Self::ConfigError(_) => io::ErrorKind::InvalidInput,
            Self::NotFound(_) => io::ErrorKind::NotFound,
            Self::ClipboardError(_) => io::ErrorKind::Other,
            Self::InvalidKey(_) => io::ErrorKind::InvalidInput,
            Self::PathTraversal(_) => io::ErrorKind::InvalidInput,
        }
    }
}
