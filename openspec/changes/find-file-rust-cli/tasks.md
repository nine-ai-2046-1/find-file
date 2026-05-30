## 1. Project Setup

- [x] 1.1 Initialize Rust project with `cargo init` in `find-file/` directory
- [x] 1.2 Add dependencies to `Cargo.toml`: clap, walkdir, globset, grep-regex, grep-searcher, rayon, serde, serde_json, toml, anyhow
- [x] 1.3 Create module structure: `main.rs`, `cli.rs`, `search.rs`, `config.rs`, `output.rs`

## 2. CLI Parsing

- [x] 2.1 Define clap struct with all flags: `-n`, `-c`, `-t`, `-d`, `-o`, `--case-sensitive`
- [x] 2.2 Implement argument validation (mutually exclusive or combinable as decided)
- [x] 2.3 Implement usage guide display for no args and invalid args
- [x] 2.4 Add test cases for CLI parsing

## 3. Config Management

- [x] 3.1 Define config struct with all fields and serde derive
- [x] 3.2 Implement config path resolution (env var > default path)
- [x] 3.3 Implement auto-create config directory and default file
- [x] 3.4 Implement config loading with TOML parsing and error handling
- [x] 3.5 Implement config precedence: CLI flags > config > defaults
- [x] 3.6 Add test cases for config management

## 4. Filename Search

- [x] 4.1 Implement glob pattern compilation from `-n` flag
- [x] 4.2 Implement directory walking with walkdir
- [x] 4.3 Implement type filtering (`-t` file/dir/all)
- [x] 4.4 Implement search root configuration (`-d`)
- [x] 4.5 Combine glob matching with directory walking
- [x] 4.6 Add test cases for filename search

## 5. Content Search

- [x] 5.1 Implement binary file detection (extension list + null byte check)
- [x] 5.2 Implement text file content search with grep-regex
- [x] 5.3 Implement case-insensitive search (default) and case-sensitive flag
- [x] 5.4 Implement max file size limit for content search
- [x] 5.5 Integrate content search with directory walking
- [x] 5.6 Add test cases for content search

## 6. Output Formatting

- [x] 6.1 Implement plain text output (one path per line)
- [x] 6.2 Implement JSON output (array of path strings)
- [x] 6.3 Ensure all output uses absolute paths
- [x] 6.4 Handle empty results (empty output for text, `[]` for JSON)
- [x] 6.5 Add test cases for output formatting

## 7. Integration & Testing

- [x] 7.1 Integrate all modules in `main.rs`
- [x] 7.2 Add integration tests with sample directory structure
- [x] 7.3 Test cross-platform compatibility (macOS + Linux)
- [x] 7.4 Test permission error handling (skip denied dirs gracefully)
- [x] 7.5 Add `--help` and error message formatting
