use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "find-file", version, about = "Find files and folders easily in Mac/Linux terminal")]
pub struct Args {
    /// Search by filename (glob pattern)
    #[arg(short = 'n')]
    pub name: Option<String>,

    /// Search file content (text files only)
    #[arg(short = 'c')]
    pub content: Option<String>,

    /// Filter by type: "file", "dir", or "all"
    #[arg(short = 't', default_value = "all")]
    pub file_type: String,

    /// Search directory (default: home directory)
    #[arg(short = 'd', default_value = "~")]
    pub dir: PathBuf,

    /// Output format: "text" or "json"
    #[arg(short = 'o', default_value = "text")]
    pub output: String,

    /// Case-sensitive search (default: case-insensitive)
    #[arg(long)]
    pub case_sensitive: bool,
}

impl Args {
    pub fn has_search_args(&self) -> bool {
        self.name.is_some() || self.content.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_name_flag() {
        let args = Args::try_parse_from(["find-file", "-n", "*.toml"]).unwrap();
        assert_eq!(args.name, Some("*.toml".to_string()));
    }

    #[test]
    fn test_parse_content_flag() {
        let args = Args::try_parse_from(["find-file", "-c", "TODO"]).unwrap();
        assert_eq!(args.content, Some("TODO".to_string()));
    }

    #[test]
    fn test_parse_type_flag() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt", "-t", "file"]).unwrap();
        assert_eq!(args.file_type, "file");
    }

    #[test]
    fn test_parse_dir_flag() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt", "-d", "/tmp"]).unwrap();
        assert_eq!(args.dir, PathBuf::from("/tmp"));
    }

    #[test]
    fn test_default_dir_is_home() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt"]).unwrap();
        assert_eq!(args.dir, PathBuf::from("~"));
    }

    #[test]
    fn test_parse_output_flag() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt", "-o", "json"]).unwrap();
        assert_eq!(args.output, "json");
    }

    #[test]
    fn test_parse_case_sensitive_flag() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt", "--case-sensitive"]).unwrap();
        assert!(args.case_sensitive);
    }

    #[test]
    fn test_has_search_args_with_name() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt"]).unwrap();
        assert!(args.has_search_args());
    }

    #[test]
    fn test_has_search_args_with_content() {
        let args = Args::try_parse_from(["find-file", "-c", "TODO"]).unwrap();
        assert!(args.has_search_args());
    }

    #[test]
    fn test_has_search_args_with_both() {
        let args = Args::try_parse_from(["find-file", "-n", "*.txt", "-c", "TODO"]).unwrap();
        assert!(args.has_search_args());
    }

    #[test]
    fn test_no_args_shows_help() {
        let args = Args::try_parse_from(["find-file"]).unwrap();
        assert!(!args.has_search_args());
    }
}
