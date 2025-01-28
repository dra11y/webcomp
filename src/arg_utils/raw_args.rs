use clap::Parser;

/// Compress files into multiple formats (brotli, deflate, gzip and zstd) at once.
#[derive(Parser, Debug)]
#[command(
    version,
    author,
    about,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct RawArgs {
    /// Paths to process. Multiple paths are accepted.
    #[arg(short, long)]
    pub path: Vec<String>,

    /// Enable Brotli compression.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub brotli: u8,

    /// Clean existing compressed files.
    #[arg(long, action = clap::ArgAction::Count)]
    pub clean: u8,

    /// Enable LZW compression.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub compress: u8,

    /// Minimum code size for LZW compression. Value must be 8 or greater.
    #[arg(long, default_value_t = 8)]
    pub compress_min_code_size: u8,

    /// Enable Deflate compression.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub deflate: u8,

    /// Perform a dry run without making changes (for compression or cleaning).
    #[arg(long, action = clap::ArgAction::Count)]
    pub dry_run: u8,

    /// Enable Gzip compression.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub gzip: u8,

    /// Set the maximum number of threads for parallel processing.
    #[arg(long, default_value_t = 10)]
    pub max_threads: usize,

    /// Display version information.
    #[clap(short = 'V', long, action = clap::ArgAction::Count)]
    pub version: u8,

    /// Enable Zstandard compression.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub zstd: u8,

    /// Compression level for Zstandard (range: 1-22).
    #[arg(long, default_value_t = 10)]
    pub zstd_level: u8,

    /// Skip generating MD5 checksums and using them to skip unchanged files.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_md5: bool,

    /// Show a progress bar.
    #[arg(long, default_value_t = false)]
    pub progress_bar: bool,

    /// Display help information.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub help: u8,
}
