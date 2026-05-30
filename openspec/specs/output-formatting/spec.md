## MODIFIED Requirements

### Requirement: JSON output
The system SHALL output results in NDJSON format when `-o json` is specified. Each result SHALL be a separate JSON object on its own line with a "path" key.

#### Scenario: NDJSON format output
- **WHEN** user runs `find-file -n "*.toml" -o json` and finds 2 files
- **THEN** system outputs two lines, each a JSON object with "path" key

#### Scenario: Empty output
- **WHEN** search finds no matching files with `-o json`
- **THEN** system outputs nothing (empty output)
