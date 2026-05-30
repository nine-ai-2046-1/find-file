## ADDED Requirements

### Requirement: CLI argument parsing
The system SHALL parse command-line arguments using clap with derive macros. The following flags SHALL be supported:
- `-n <NAME>`: filename glob pattern
- `-c <CONTENT>`: content search keyword
- `-t <TYPE>`: file type filter ("file", "dir", "all")
- `-d <DIR>`: search root directory
- `-o <FORMAT>`: output format ("text", "json")
- `-h, --help`: show help message

#### Scenario: Valid arguments parsed
- **WHEN** user runs `find-file -n "*.toml" -d ./src`
- **THEN** system parses `-n` as filename pattern "*.toml" and `-d` as search directory "./src"

#### Scenario: Invalid argument rejected
- **WHEN** user runs `find-file -x "unknown"`
- **THEN** system shows error message and usage guide

### Requirement: Usage guide display
The system SHALL display a usage guide when:
- No arguments are provided
- Arguments are invalid or incomplete

The usage guide SHALL include:
- Program description
- All available flags with descriptions
- Default values
- Example commands

#### Scenario: No arguments provided
- **WHEN** user runs `find-file` with no arguments
- **THEN** system displays usage guide and exits

#### Scenario: Invalid flag provided
- **WHEN** user runs `find-file --invalid-flag`
- **THEN** system displays error message followed by usage guide
