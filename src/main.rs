mod cli;
mod config;
mod output;
mod search;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use config::Config;

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.has_search_args() {
        Args::parse_from(["find-file", "--help"]);
        return Ok(());
    }

    let mut config = Config::load()?;

    if args.name.is_some() {
        config.name.case_sensitive = args.case_sensitive;
    }

    if args.content.is_some() {
        config.content.case_sensitive = args.case_sensitive;
    }

    let search_dir = if args.dir.to_string_lossy() == "~" {
        dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."))
    } else {
        args.dir.clone()
    };
    let output_format = args.output.clone();

    let mut callback = |path: std::path::PathBuf| {
        output::print_line(&path, &output_format);
    };

    if let Some(ref name) = args.name {
        search::search_filename(
            &config,
            name,
            &search_dir,
            &config.file_type,
            &mut callback,
        )?;
    }

    if let Some(ref content) = args.content {
        search::search_content(
            &config,
            content,
            &search_dir,
            config.content.case_sensitive,
            config.content.max_file_size_mb,
            &mut callback,
        )?;
    }

    Ok(())
}
