## ADDED Requirements

### Requirement: NDJSON output format
The system SHALL output results in Newline Delimited JSON format when `-o json` is specified. Each result SHALL be a separate JSON object on its own line.

#### Scenario: NDJSON output
- **WHEN** user runs `find-file -n "*.toml" -o json` and finds 2 files
- **THEN** system outputs two lines:
  ```
  {"path":"/path/to/file1.toml"}
  {"path":"/path/to/file2.toml"}
  ```

#### Scenario: Empty NDJSON output
- **WHEN** search finds no matching files with `-o json`
- **THEN** system outputs nothing (empty output)
