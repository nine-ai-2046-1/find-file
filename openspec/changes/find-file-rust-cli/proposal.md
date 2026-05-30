## Why

Finding files and folders in the terminal is cumbersome with the native `find` command — it has verbose syntax, poor defaults, and inconsistent behavior across Mac and Linux. A simple, focused CLI tool with sensible defaults makes file discovery fast and intuitive.

## What Changes

- New Rust CLI binary `find-file` with these capabilities:
  - **Filename search** (`-n`): glob pattern matching for file/folder names
  - **Content search** (`-c`): text search inside non-binary files (txt, md, java, js, ts, toml, yaml, etc.)
  - **Type filter** (`-t`): restrict results to files, directories, or both (default: both)
  - **Search root** (`-d`): specify starting directory (default: machine root `/`)
  - **Output format** (`-o`): plain text list (default) or JSON
  - **Usage guide**: auto-shows when no args or invalid args provided
- Configuration file at `$HOME/.config/find-file/config.toml` with sensible defaults
- Config auto-creation on first run if not exists
- Config path overridable via `FIND_FILE_CONFIG` env var

## Capabilities

### New Capabilities

- `cli-parsing`: Command-line argument parsing, validation, and usage guide display
- `filename-search`: Glob-based filename/folder name matching with type filtering
- `content-search`: Text content search in non-binary files with size limits and case sensitivity options
- `config-management`: TOML config loading, path resolution, defaults, and auto-creation
- `output-formatting`: Plain text and JSON output formatters

### Modified Capabilities

<!-- None — this is a new project -->

## Impact

- New Rust project with dependencies: `clap`, `walkdir`, `globset`, `regex`, `serde`, `serde_json`, `toml`, `anyhow`
- No existing code affected — greenfield project
- Platform support: macOS (primary), Linux (secondary)
