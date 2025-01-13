use crate::arg_utils::raw_args::RawArgs;

/// Represents validated command-line arguments.
pub struct Args {
    pub paths: Vec<String>,         // Paths to process.
    pub brotli: bool,               // Enable Brotli compression.
    pub clean: bool,                // Clean existing compressed files.
    pub compress: bool,             // Enable LZW compression.
    pub compress_min_code_size: u8, // Minimum code size for LZW compression.
    pub deflate: bool,              // Enable Deflate compression.
    pub dry_run: bool,              // Enable dry run mode.
    pub gzip: bool,                 // Enable Gzip compression.
    pub help: bool,                 // Display help information.
    pub max_threads: usize,         // Maximum threads for parallel processing.
    pub version: bool,              // Display version information.
    pub zstd: bool,                 // Enable Zstandard compression.
    pub zstd_level: u8,             // Compression level for Zstandard.
}

impl TryFrom<RawArgs> for Args {
    type Error = anyhow::Error;

    // Validate compress_min_code_size.
    fn try_from(value: RawArgs) -> anyhow::Result<Self> {
        // Validate compress_min_code_size.
        if value.compress_min_code_size < 8 {
            return Err(anyhow::anyhow!(
                "`--compress-min-code-size` must be 8 or greater."
            ));
        }

        // Validate zstd_level.
        if value.zstd_level < 1 || value.zstd_level > 22 {
            return Err(anyhow::anyhow!("`--zstd-level` must be between 1 and 22."));
        }

        // Canonicalize paths and check existence.
        let mut paths: Vec<String> = Vec::new();
        for path in value.path {
            if std::path::PathBuf::from(&path).exists() {
                paths.push(path);
            } else {
                return Err(anyhow::anyhow!("Path does not exist: {}", path));
            }
        }

        Ok(Self {
            paths,
            brotli: value.brotli > 0,
            clean: value.clean > 0,
            compress: value.compress > 0,
            compress_min_code_size: value.compress_min_code_size,
            deflate: value.deflate > 0,
            dry_run: value.dry_run > 0,
            gzip: value.gzip > 0,
            help: value.help > 0,
            max_threads: value.max_threads,
            version: value.version > 0,
            zstd: value.zstd > 0,
            zstd_level: value.zstd_level,
        })
    }
}
