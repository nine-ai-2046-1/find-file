## MODIFIED Requirements

### Requirement: Search root configuration
The system SHALL search starting from the directory specified by `-d`. Default search root SHALL be `.` (current working directory).

#### Scenario: Default search root
- **WHEN** user runs `find-file -n "*.conf"` without `-d` flag
- **THEN** system searches from current working directory

#### Scenario: Custom search root
- **WHEN** user runs `find-file -n "*.log" -d /var/log`
- **THEN** system searches only within `/var/log` and its subdirectories
