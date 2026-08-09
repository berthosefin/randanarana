## Purpose

Discovers the files and directories to rename, decides which items to skip, shows a preview, and performs the renames while reporting the outcome.

## MODIFIED Requirements

### Requirement: Rename execution and summary
The system SHALL perform the renames using a standard filesystem rename and SHALL print a summary with the counts of renamed, skipped, and failed items. For every item actually renamed, the system SHALL record the rename in the manifest file so the run can be undone later.

#### Scenario: Successful renames
- **WHEN** renames complete
- **THEN** a summary reports how many items were renamed, skipped, and failed

#### Scenario: Rename failure
- **WHEN** an item cannot be renamed
- **THEN** an error is printed for that item, it is counted as failed, and the remaining items are still processed

#### Scenario: Renames are recorded
- **WHEN** a rename run renames at least one item
- **THEN** every performed rename is recorded in the manifest file
