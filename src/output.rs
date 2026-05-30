use serde_json;
use std::path::Path;

pub fn print_line(path: &Path, format: &str) {
    match format {
        "json" => {
            let obj = serde_json::json!({"path": path.display().to_string()});
            println!("{}", obj);
        }
        _ => {
            println!("{}", path.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_print_line_text() {
        let path = PathBuf::from("/path/to/file1.txt");
        print_line(&path, "text");
    }

    #[test]
    fn test_print_line_json() {
        let path = PathBuf::from("/path/to/file1.txt");
        print_line(&path, "json");
    }
}
