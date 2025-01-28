use std::path::PathBuf;

/// Represents the error during file process operation.
#[derive(Debug, Clone, Default)]
pub struct FileProcessError {
    pub message: String, // Error message.
    pub path: PathBuf,   // Path to the compressed file.
}
