## ADDED Requirements

### Requirement: Filename glob search
The system SHALL search for files and directories matching a glob pattern provided via `-n`. The search SHALL traverse the directory tree starting from the search root.

#### Scenario: Simple glob match
- **WHEN** user runs `find-file -n "*.toml"` in a directory containing `config.toml` and `data.txt`
- **THEN** system outputs the absolute path to `config.toml`

#### Scenario: Nested directory match
- **WHEN** user runs `find-file -n "*.js"` in a project with `src/app.js` and `lib/utils.js`
- **THEN** system outputs absolute paths to both `src/app.js` and `lib/utils.js`

### Requirement: Type filtering
The system SHALL support `-t` flag to filter results by type:
- `"file"`: only files
- `"dir"`: only directories
- `"all"`: both files and directories (default)

#### Scenario: Search only files
- **WHEN** user runs `find-file -n "test" -t file` in a directory containing `test/` folder and `test.txt` file
- **THEN** system outputs only the path to `test.txt`

#### Scenario: Search only directories
- **WHEN** user runs `find-file -n "src" -t dir` in a directory containing `src/` folder and `src.txt` file
- **THEN** system outputs only the path to `src/`

### Requirement: Search root configuration
The system SHALL search starting from the directory specified by `-d`. Default search root SHALL be `/` (machine root).

#### Scenario: Custom search root
- **WHEN** user runs `find-file -n "*.log" -d /var/log`
- **THEN** system searches only within `/var/log` and its subdirectories

#### Scenario: Default search root
- **WHEN** user runs `find-file -n "*.conf"` without `-d` flag
- **THEN** system searches from `/` (machine root)
