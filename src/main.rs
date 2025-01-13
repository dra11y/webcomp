use crate::arg_utils::args::Args;
use app::app_clean::app_clean;
use app::app_compress::app_compress;
use arg_utils::raw_args::RawArgs;
use clap::{crate_name, crate_version, CommandFactory, Parser};
use clap_help::Printer;

mod app;
mod arg_utils;
mod compressor;
mod search_files;
mod style;

fn main() -> anyhow::Result<()> {
    match get_args() {
        Ok(args) => {
            if args.help {
                display_help();
            } else if args.version {
                display_version();
            } else if args.paths.is_empty() {
                anyhow::bail!("No path specified. Use -h or --help for usage information.");
            } else if args.clean {
                app_clean(&args)?;
            } else if args.brotli || args.compress || args.deflate || args.gzip || args.zstd {
                app_compress(&args)?;
            } else {
                anyhow::bail!(
            "Specify at least one compression type (brotli, compress, deflate, gzip, or zstd). Use -h or --help for usage information."
            );
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("{}", &err);
            display_help();
            Err(anyhow::anyhow!(err))
        }
    }
}

/// Parse and validate command-line arguments.
fn get_args() -> anyhow::Result<Args> {
    let args: Args = RawArgs::try_parse()?.try_into()?;
    Ok(args)
}

/// Display usage instructions.
fn display_help() {
    static INTRO: &str = "
Compress files into multiple formats (brotli, compress, deflate, gzip and zstd) at once.
nSupported file types: CSS, CSV, HTML, JS, JSON, MAP, MATHML, MD, SGML, SVG, TSV, TXT, WASM, XHTML, XML, XSLT and YAML.
";

    Printer::new(RawArgs::command())
        .with("introduction", INTRO)
        .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
        .without("author")
        .print_help();
}

/// Display version information.
fn display_version() {
    println!("{} {}", crate_name!(), crate_version!());
}
