use std::path::PathBuf;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

pub fn search_files(
    locations: &[String],
    pattern: lazy_regex::regex::Regex,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut builder = GlobSetBuilder::new();
    for location in locations {
        builder.add(Glob::new(location)?);
    }
    let globset = builder.build()?;
    let mut files = Vec::new();

    // Walk through all provided locations
    for location in locations {
        let path = PathBuf::from(location);

        // If it's a directory, walk through it recursively
        if path.is_dir() {
            for entry in WalkDir::new(&path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                if !entry_path.is_file() {
                    continue;
                }
                if let Some(path_str) = entry_path.to_str() {
                    if pattern.is_match(path_str) {
                        files.push(entry_path.to_path_buf());
                    }
                }
            }
        }
        // If it's a file or a glob pattern
        else {
            let base_dir = if path.is_file() {
                path.parent().unwrap_or(&path).to_path_buf()
            } else {
                PathBuf::from(".")
            };

            for entry in WalkDir::new(&base_dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                if !entry_path.is_file() {
                    continue;
                }

                // Check if the path matches any glob pattern
                if globset.is_match(entry_path) {
                    if let Some(path_str) = entry_path.to_str() {
                        if pattern.is_match(path_str) {
                            files.push(entry_path.to_path_buf());
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}
