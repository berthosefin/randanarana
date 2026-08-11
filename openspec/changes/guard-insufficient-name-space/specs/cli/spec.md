## MODIFIED Requirements

### Requirement: Exit codes
The system SHALL exit with status 0 on success and help, and with a nonzero status on errors. When the user interrupts an interactive run, the system SHALL print a summary and exit with status 130.

#### Scenario: Successful run
- **WHEN** the renames complete successfully
- **THEN** the tool prints a summary and exits with status 0

#### Scenario: Interrupted run
- **WHEN** the user interrupts the tool during an interactive run
- **THEN** the tool prints a summary and exits with status 130

#### Scenario: Insufficient name space
- **WHEN** the requested `--length` cannot yield enough unique names for the items to rename in the target directory
- **THEN** the tool prints an error explaining the length and the number of names needed and available, renames nothing, and exits with a nonzero status
