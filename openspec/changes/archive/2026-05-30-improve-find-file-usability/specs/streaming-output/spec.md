## ADDED Requirements

### Requirement: Streaming output during search
The system SHALL print each matching file as it is found, not after all files are scanned. Results SHALL appear in the terminal as the directory traversal progresses.

#### Scenario: Results appear immediately
- **WHEN** user runs `find-file -n "*.log"` in a directory with nested subdirectories
- **THEN** system prints each matching path as soon as it is found, before continuing traversal

#### Scenario: Interrupted search shows partial results
- **WHEN** user presses Ctrl+C during a search
- **THEN** system displays all results found before interruption
