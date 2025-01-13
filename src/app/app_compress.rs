use crate::app::file_process_error::FileProcessError;
use crate::arg_utils::args::Args;
use crate::compressor::compress_type::CompressType;
use crate::compressor::compressed_result::CompressedResult;
use crate::search_files::search_files;
use crate::style::{STYLE_BAR, STYLE_SPINNER};

/// Compresses files in specified paths using selected compression methods.
///
/// # Arguments
/// * `args` - Program arguments containing paths and compression options.
pub fn app_compress(args: &Args) -> anyhow::Result<()> {
    // Supported file extensions for compression.
    // see: https://svn.apache.org/viewvc/httpd/httpd/trunk/docs/conf/mime.types?view=markup
    let now = std::time::Instant::now();

    let files = search_files(
        &args.paths,
        lazy_regex::Regex::new(
            r#"(?i)\.(css|csv|tsv|es|html?|sgml?|xht(ml)?|m?js|json(ml)?|map|mathml|md|markdown|svg|te?xt|conf|def|list|log|in|wasm|xml|xslt?|ya?ml)$"#,
        )?,
    )?;
    if files.is_empty() {
        anyhow::bail!("No matching files found. Ensure the directory is accessible and not ignored by .gitignore.");
    }

    let mut compress_types: Vec<CompressType> = vec![];
    if args.brotli {
        compress_types.push(CompressType::Brotli);
    }
    if args.compress {
        compress_types.push(CompressType::Lzw(args.compress_min_code_size));
    }
    if args.deflate {
        compress_types.push(CompressType::Deflate);
    }
    if args.gzip {
        compress_types.push(CompressType::Gzip);
    }
    if args.zstd {
        compress_types.push(CompressType::Zstd(args.zstd_level as i32));
    }
    let dry_run = args.dry_run;

    let total_tasks = (files.len() * compress_types.len()) as u64;
    let progress = indicatif::MultiProgress::new();
    let overall_bar = progress.add(indicatif::ProgressBar::new(total_tasks));
    overall_bar.set_style(STYLE_BAR.clone());

    let thread_pool = threadpool::ThreadPool::new(args.max_threads);
    let compressed_files = std::sync::Arc::new(std::sync::RwLock::new(Vec::new()));
    let skipped_files = std::sync::Arc::new(std::sync::RwLock::new(Vec::new()));
    let errored_files = std::sync::Arc::new(std::sync::RwLock::new(Vec::new()));

    for file in files {
        let source = match std::fs::read(&file) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let source_len = source.len();
        let source_arc = std::sync::Arc::new(source);

        for compress_type in compress_types.clone() {
            let filename = compress_type.get_filename(&file);
            let source_cloned = std::sync::Arc::clone(&source_arc);
            let processed_files_cloned = compressed_files.clone();
            let skipped_files_cloned = skipped_files.clone();
            let errored_files_cloned = errored_files.clone();

            let progress_spinner = progress.add(indicatif::ProgressBar::new_spinner());
            progress_spinner.set_style(STYLE_SPINNER.clone());
            progress_spinner.enable_steady_tick(std::time::Duration::from_millis(120));
            progress_spinner.set_message(filename.clone());

            let overall_bar_cloned = overall_bar.clone();

            thread_pool.execute(move || {
                match compress_type.compress(&source_cloned) {
                    Ok(compressed_data) => {
                        let compressed_data_len = compressed_data.len();
                        let result = CompressedResult {
                            ratio: (compressed_data_len as f64) / (source_len as f64) * 100.0,
                            path: filename.to_string(),
                        };

                        if source_len > compressed_data_len {
                            if dry_run {
                                processed_files_cloned.write().unwrap().push(result);
                            } else {
                                match std::fs::write(&filename, &compressed_data) {
                                    Ok(_) => {
                                        processed_files_cloned.write().unwrap().push(result);
                                    }
                                    Err(err) => {
                                        progress_spinner.abandon_with_message(format!(
                                            "Error saving {}: {}",
                                            filename, err
                                        ));
                                        errored_files_cloned.write().unwrap().push(
                                            FileProcessError {
                                                message: err.to_string(),
                                                path: filename,
                                            },
                                        );
                                    }
                                }
                            }
                        } else {
                            skipped_files_cloned.write().unwrap().push(filename);
                        }
                    }
                    Err(err) => {
                        progress_spinner.abandon_with_message(format!(
                            "Compression failed for {}: {}",
                            filename, err
                        ));
                        errored_files_cloned
                            .write()
                            .unwrap()
                            .push(FileProcessError {
                                message: err.to_string(),
                                path: filename,
                            });
                    }
                }

                progress_spinner.finish_and_clear();
                overall_bar_cloned.inc(1);
            });
        }
    }
    thread_pool.join();
    progress.clear()?;

    {
        let mut final_compressed_files = compressed_files.read().unwrap().clone();
        final_compressed_files.sort_by(|a, b| a.path.cmp(&b.path));

        let mut final_skipped_files = skipped_files.read().unwrap().clone();
        final_skipped_files.sort();

        let mut final_errored_files = errored_files.read().unwrap().clone();
        final_errored_files.sort_by(|a, b| a.path.cmp(&b.path));

        println!(
            "Compressed files:{} Skipped files:{} Errored files:{} Elapsed time:{:.2?}\n",
            final_compressed_files.len(),
            final_skipped_files.len(),
            final_errored_files.len(),
            now.elapsed(),
        );

        if !final_compressed_files.is_empty() {
            if args.dry_run {
                println!(
                    "Files to be compressed (Dry run): {}",
                    final_compressed_files.len()
                );
            } else {
                println!("Compressed files: {}", final_compressed_files.len());
            }

            println!("Ratio\tFile Path",);
            for processed_file in final_compressed_files {
                println!("{:.1}%\t{}", processed_file.ratio, processed_file.path);
            }
        }

        if !final_skipped_files.is_empty() {
            println!("\nSkipped files: {}", final_skipped_files.len());
            for skipped_file in final_skipped_files {
                println!("{}", skipped_file);
            }
        }

        if !final_errored_files.is_empty() {
            println!(
                "\nErrored files: {}\nFile Path\tError Message",
                final_errored_files.len()
            );
            for errored_file in final_errored_files {
                println!("{}\t{}", errored_file.path, errored_file.message);
            }
        }
    }
    Ok(())
}
