use crate::app::file_process_error::FileProcessError;
use crate::arg_utils::args::Args;
use crate::search_files::search_files;

/// Removes existing compressed files matching specified patterns.
///
/// # Arguments
/// * `args` - Program arguments containing paths to search.
pub fn app_clean(args: &Args) -> anyhow::Result<()> {
    // Search for files matching compression extensions.
    let files = search_files(
        &args.paths,
        lazy_regex::Regex::new(r#"(?i)\.(br|zz|gz|Z|zst)$"#)?,
    )?;

    let mut removed_files: Vec<String> = Vec::new();
    let mut errored_files: Vec<FileProcessError> = Vec::new();

    for file in files {
        if args.dry_run {
            removed_files.push(file);
        } else {
            match std::fs::remove_file(&file) {
                Ok(_) => {
                    removed_files.push(file);
                }
                Err(err) => errored_files.push(FileProcessError {
                    message: err.to_string(),
                    path: file,
                }),
            }
        }
    }

    // Display results to the user.
    if removed_files.is_empty() {
        println!("No matching files found to remove.");
    } else {
        if args.dry_run {
            println!("Files to be removed (Dry run): {}", removed_files.len());
        } else {
            println!("Removed files: {}", removed_files.len());
        }
        removed_files.sort();
        for removed_file in removed_files {
            println!("{}", removed_file);
        }
    }

    if !errored_files.is_empty() {
        println!(
            "Errored files: {}\nFile Path\tError Message",
            errored_files.len()
        );

        errored_files.sort_by(|a, b| a.path.cmp(&b.path));
        for errored_file in errored_files {
            println!("{}\t{}", errored_file.path, errored_file.message);
        }
    }

    Ok(())
}
