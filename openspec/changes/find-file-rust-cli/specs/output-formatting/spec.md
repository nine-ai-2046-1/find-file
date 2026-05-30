## ADDED Requirements

### Requirement: Plain text output
The system SHALL output results as a plain text list by default. Each result SHALL be one absolute path per line.

#### Scenario: Multiple results
- **WHEN** search finds 3 files matching the pattern
- **THEN** system outputs 3 lines, each with an absolute file path

#### Scenario: No results found
- **WHEN** search finds no matching files
- **THEN** system outputs nothing (empty output) and exits with non-zero status

### Requirement: JSON output
The system SHALL output results in JSON format when `-o json` is specified. The JSON SHALL be an array of strings, each being an absolute file path.

#### Scenario: JSON format output
- **WHEN** user runs `find-file -n "*.toml" -o json` and finds 2 files
- **THEN** system outputs valid JSON array: `["/path/to/file1.toml", "/path/to/file2.toml"]`

#### Scenario: Empty JSON array
- **WHEN** search finds no matching files with `-o json`
- **THEN** system outputs empty JSON array: `[]`

### Requirement: Absolute paths in output
The system SHALL always output absolute paths, regardless of the search root or current working directory.

#### Scenario: Relative search root
- **WHEN** user runs `find-file -n "*.txt" -d ./src` and finds `./src/readme.txt`
- **THEN** system outputs `/absolute/path/to/src/readme.txt`
