use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub search_root: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub output: String,
    pub content: ContentConfig,
    pub name: NameConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentConfig {
    pub text_only: bool,
    pub case_sensitive: bool,
    pub max_file_size_mb: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NameConfig {
    pub pattern_type: String,
    pub case_sensitive: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub threads: usize,
    pub max_depth: usize,
    pub respect_gitignore: bool,
    pub skip_hidden: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search_root: "~".to_string(),
            file_type: "all".to_string(),
            output: "text".to_string(),
            content: ContentConfig {
                text_only: true,
                case_sensitive: false,
                max_file_size_mb: 10,
            },
            name: NameConfig {
                pattern_type: "glob".to_string(),
                case_sensitive: false,
            },
            performance: PerformanceConfig {
                threads: 0,
                max_depth: 0,
                respect_gitignore: true,
                skip_hidden: true,
            },
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        if let Ok(env_path) = std::env::var("FIND_FILE_CONFIG") {
            return Ok(PathBuf::from(env_path));
        }

        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(".config").join("find-file").join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            let config = Self::default();
            config.save(&path)?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)
            .context(format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .context(format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create config directory: {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(path, content)
            .context(format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.search_root, "~");
        assert_eq!(config.file_type, "all");
        assert_eq!(config.output, "text");
        assert!(config.content.text_only);
        assert!(!config.content.case_sensitive);
        assert_eq!(config.content.max_file_size_mb, 10);
    }

    #[test]
    fn test_config_path_with_env_var() {
        std::env::set_var("FIND_FILE_CONFIG", "/custom/path/config.toml");
        let path = Config::config_path().unwrap();
        assert_eq!(path, PathBuf::from("/custom/path/config.toml"));
        std::env::remove_var("FIND_FILE_CONFIG");
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let config = Config::default();
        config.save(&config_path).unwrap();

        let loaded = Config::load().unwrap();
        assert_eq!(loaded.search_root, config.search_root);
    }
}
