use anyhow::Result;
use globset::Glob;
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;

const TEXT_EXTENSIONS: &[&str] = &[
    "txt", "md", "java", "js", "ts", "jsx", "tsx", "py", "rb", "go", "rs", "c", "h", "cpp",
    "hpp", "toml", "yaml", "yml", "json", "xml", "html", "css", "scss", "less", "sh", "bash",
    "zsh", "fish", "sql", "csv", "log", "ini", "cfg", "conf", "env", "dockerfile", "makefile",
    "gitignore", "gitattributes", "editorconfig", "prettierrc", "eslintrc", "babelrc",
    "tsconfig", "package", "cargo", "gemfile", "pipfile", "requirements",
];

pub fn search_filename(config: &Config, pattern: &str, dir: &Path, file_type: &str, callback: &mut dyn FnMut(PathBuf)) -> Result<()> {
    let glob = Glob::new(pattern)?;
    let glob_matcher = glob.compile_matcher();

    let max_depth = if config.performance.max_depth > 0 {
        config.performance.max_depth
    } else {
        usize::MAX
    };

    for entry in WalkDir::new(dir)
        .max_depth(max_depth)
        .follow_links(false)
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if glob_matcher.is_match(name.as_ref()) {
            let matches_type = match file_type {
                "file" => entry.file_type().is_file(),
                "dir" => entry.file_type().is_dir(),
                _ => true,
            };

            if matches_type {
                if let Ok(abs_path) = fs::canonicalize(path) {
                    callback(abs_path);
                }
            }
        }
    }

    Ok(())
}

pub fn search_content(
    config: &Config,
    pattern: &str,
    dir: &Path,
    case_sensitive: bool,
    max_size_mb: u64,
    callback: &mut dyn FnMut(PathBuf),
) -> Result<()> {
    let regex_pattern = if case_sensitive {
        pattern.to_string()
    } else {
        format!("(?i){}", pattern)
    };

    let re = Regex::new(&regex_pattern)?;

    let max_depth = if config.performance.max_depth > 0 {
        config.performance.max_depth
    } else {
        usize::MAX
    };

    let max_bytes = max_size_mb * 1024 * 1024;

    for entry in WalkDir::new(dir)
        .max_depth(max_depth)
        .follow_links(false)
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > max_bytes {
                continue;
            }
        } else {
            continue;
        }

        if !is_text_file(path) {
            continue;
        }

        if let Ok(content) = fs::read_to_string(path) {
            if re.is_match(&content) {
                if let Ok(abs_path) = fs::canonicalize(path) {
                    callback(abs_path);
                }
            }
        }
    }

    Ok(())
}

fn is_text_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        if TEXT_EXTENSIONS.iter().any(|&e| e == ext_lower) {
            return true;
        }
    }

    if let Some(name) = path.file_name() {
        let name_lower = name.to_string_lossy().to_lowercase();
        if TEXT_EXTENSIONS.iter().any(|&e| name_lower.starts_with(e)) {
            return true;
        }
    }

    if let Ok(mut file) = fs::File::open(path) {
        let mut buffer = [0u8; 512];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            return !buffer[..bytes_read].contains(&0);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        fs::write(dir.join("test.txt"), "Hello World").unwrap();
        fs::write(dir.join("readme.md"), "TODO: fix this").unwrap();
        fs::write(dir.join("data.json"), "{\"key\": \"value\"}").unwrap();

        fs::create_dir(dir.join("src")).unwrap();
        fs::write(dir.join("src").join("app.js"), "console.log('hello');").unwrap();
        fs::write(dir.join("src").join("utils.js"), "TODO: implement").unwrap();

        fs::create_dir(dir.join("docs")).unwrap();
        fs::write(dir.join("docs").join("guide.md"), "This is a guide").unwrap();

        temp_dir
    }

    #[test]
    fn test_search_filename_simple() {
        let temp_dir = create_test_dir();
        let config = Config::default();
        let mut results = Vec::new();
        search_filename(&config, "*.txt", temp_dir.path(), "all", &mut |p| results.push(p)).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].to_string_lossy().ends_with("test.txt"));
    }

    #[test]
    fn test_search_filename_nested() {
        let temp_dir = create_test_dir();
        let config = Config::default();
        let mut results = Vec::new();
        search_filename(&config, "*.js", temp_dir.path(), "all", &mut |p| results.push(p)).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_filename_type_filter() {
        let temp_dir = create_test_dir();
        let config = Config::default();

        let mut files = Vec::new();
        search_filename(&config, "src", temp_dir.path(), "file", &mut |p| files.push(p)).unwrap();
        assert_eq!(files.len(), 0);

        let mut dirs = Vec::new();
        search_filename(&config, "src", temp_dir.path(), "dir", &mut |p| dirs.push(p)).unwrap();
        assert_eq!(dirs.len(), 1);
    }

    #[test]
    fn test_search_content() {
        let temp_dir = create_test_dir();
        let config = Config::default();
        let mut results = Vec::new();
        search_content(&config, "TODO", temp_dir.path(), false, 10, &mut |p| results.push(p)).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_content_case_insensitive() {
        let temp_dir = create_test_dir();
        let config = Config::default();
        let mut results = Vec::new();
        search_content(&config, "hello", temp_dir.path(), false, 10, &mut |p| results.push(p)).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_content_case_sensitive() {
        let temp_dir = create_test_dir();
        let config = Config::default();
        let mut results = Vec::new();
        search_content(&config, "Hello", temp_dir.path(), true, 10, &mut |p| results.push(p)).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_is_text_file() {
        assert!(is_text_file(Path::new("test.txt")));
        assert!(is_text_file(Path::new("readme.md")));
        assert!(is_text_file(Path::new("app.js")));
    }
}
