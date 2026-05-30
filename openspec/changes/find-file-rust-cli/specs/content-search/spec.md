## ADDED Requirements

### Requirement: Content search in text files
The system SHALL search for a keyword inside file contents when `-c` is provided. Only non-binary files SHALL be searched. Binary detection SHALL use:
1. Check file extension against known text extensions (txt, md, java, js, ts, toml, yaml, json, xml, html, css, py, rb, go, rs, c, h, cpp, sh, sql, csv, log, ini, cfg, conf, env)
2. If no extension match, read first 512 bytes and check for null byte

#### Scenario: Search finds keyword in text file
- **WHEN** user runs `find-file -c "TODO"` in a directory containing `readme.md` with "TODO" text
- **THEN** system outputs the absolute path to `readme.md`

#### Scenario: Binary file skipped
- **WHEN** user runs `find-file -c "error"` in a directory containing `image.png` and `app.log` with "error" text
- **THEN** system outputs only the path to `app.log`, skipping `image.png`

### Requirement: Case-insensitive content search
The system SHALL perform case-insensitive content search by default. A flag SHALL be available to enable case-sensitive search.

#### Scenario: Default case-insensitive
- **WHEN** user runs `find-file -c "error"` and a file contains "ERROR"
- **THEN** system matches and outputs that file

#### Scenario: Case-sensitive search
- **WHEN** user runs `find-file -c "error" --case-sensitive` and a file contains "ERROR"
- **THEN** system does NOT match that file

### Requirement: Max file size for content search
The system SHALL skip files larger than `max_file_size_mb` (default: 10MB) during content search.

#### Scenario: Large file skipped
- **WHEN** user runs `find-file -c "data"` and a file is 50MB
- **THEN** system skips that file and continues searching
