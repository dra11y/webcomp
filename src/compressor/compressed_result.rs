/// Represents the result of a compression operation.
#[derive(Debug, Clone, Default)]
pub struct CompressedResult {
    pub ratio: f64,   // Compression ratio as a percentage.
    pub path: String, // Path to the compressed file.
}
