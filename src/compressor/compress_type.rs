use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::app::app_compress::add_extension;

/// Supported compression types.
#[derive(Ord, Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum CompressType {
    Brotli,
    Deflate,
    Gzip,
    Lzw(u8),   // LZW compression with a specified minimum code size.
    Zstd(i32), // Zstandard compression with a specified level.
}

impl CompressType {
    /// Compresses input data based on the selected compression type.
    pub fn compress(&self, input: &[u8]) -> anyhow::Result<Vec<u8>> {
        match self {
            CompressType::Brotli => {
                let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 24);
                writer.write_all(input)?;
                writer.flush()?;
                Ok(writer.into_inner())
            }
            CompressType::Deflate => {
                let mut encoder =
                    flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
                encoder.write_all(input)?;
                encoder.flush()?;
                let result = encoder.finish()?;
                Ok(result)
            }
            CompressType::Gzip => {
                let mut encoder =
                    flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
                encoder.write_all(input)?;
                encoder.flush()?;
                let result = encoder.finish()?;
                Ok(result)
            }
            CompressType::Lzw(min_code_size) => {
                let mut compressed = vec![];
                {
                    let mut enc =
                        lzw::Encoder::new(lzw::LsbWriter::new(&mut compressed), *min_code_size)?;
                    enc.encode_bytes(input)?;
                }
                Ok(compressed)
            }
            CompressType::Zstd(level) => {
                let compressed = zstd::stream::encode_all(input, *level)?;
                Ok(compressed)
            }
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            CompressType::Brotli => "br",
            CompressType::Deflate => "zz",
            CompressType::Gzip => "gz",
            CompressType::Lzw(_) => "Z",
            CompressType::Zstd(_) => "zst",
        }
    }

    /// Generates a file name for the compressed output based on the compression type.
    pub fn get_filename(&self, original: &Path) -> PathBuf {
        add_extension(original, self.extension())
    }
}
