## ADDED Requirements

### Requirement: Config file path resolution
The system SHALL resolve the config file path in this order:
1. If `FIND_FILE_CONFIG` env var is set, use that path
2. Otherwise, use `$HOME/.config/find-file/config.toml`

#### Scenario: Env var overrides default path
- **WHEN** `FIND_FILE_CONFIG=/custom/path/config.toml` is set
- **THEN** system loads config from `/custom/path/config.toml`

#### Scenario: Default path used
- **WHEN** no env var is set
- **THEN** system loads config from `$HOME/.config/find-file/config.toml`

### Requirement: Auto-create config on first run
The system SHALL create the config directory and default config file if they do not exist. The default config SHALL contain all available settings with sensible defaults.

#### Scenario: First run creates config
- **WHEN** user runs `find-file` for the first time and `$HOME/.config/find-file/config.toml` does not exist
- **THEN** system creates the directory and writes default config file

#### Scenario: Config already exists
- **WHEN** config file already exists
- **THEN** system loads existing config without overwriting

### Requirement: Config file format
The system SHALL use TOML format for the config file. The config SHALL support these sections:
- `search_root` (string): default search directory
- `type` (string): default file type filter
- `output` (string): default output format
- `[content]` section: content search settings
- `[name]` section: filename search settings
- `[performance]` section: performance settings

#### Scenario: Valid config parsed
- **WHEN** config file contains valid TOML with `search_root = "~"`
- **THEN** system uses home directory as default search root

#### Scenario: Invalid config handled
- **WHEN** config file contains invalid TOML syntax
- **THEN** system shows warning and falls back to built-in defaults

### Requirement: Config precedence
The system SHALL apply configuration in this order: CLI flags > config file > built-in defaults. CLI flags SHALL only override config when explicitly provided by the user.

#### Scenario: CLI flag overrides config
- **WHEN** config says `search_root = "~"` and user runs `find-file -d /tmp -n "*.log"`
- **THEN** system searches `/tmp`, not home directory

#### Scenario: Config overrides default
- **WHEN** config says `output = "json"` and user runs `find-file -n "*.txt"` without `-o` flag
- **THEN** system outputs JSON format
