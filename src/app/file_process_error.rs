/// Represents the error during file process operation.
#[derive(Debug, Clone, Default)]
pub struct FileProcessError {
    pub message: String, // Error message.
    pub path: String,    // Path to the compressed file.
}
