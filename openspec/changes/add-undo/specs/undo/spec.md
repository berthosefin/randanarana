## Purpose

Defines the `randanarana undo` command that restores files to their original names using the manifest written by a previous rename run.

## ADDED Requirements

### Requirement: Undo invocation
The system SHALL support `randanarana undo [DIR]` where `DIR` defaults to the current directory. When no manifest exists in the target directory, the system SHALL print a message and exit with status 0.

#### Scenario: Undo restores renamed items
- **WHEN** the user runs `randanarana undo` in a directory whose manifest records renames
- **THEN** the recorded items are renamed back to their original names

#### Scenario: No manifest
- **WHEN** the user runs `randanarana undo` in a directory without a manifest
- **THEN** the tool prints that there is nothing to undo and exits with status 0

### Requirement: Confirmation before restoring
The system SHALL print a preview of the restorations and ask for a single confirmation (`y`/`N`) before renaming anything. With `--dry-run`, the system SHALL print the preview and exit without renaming.

#### Scenario: Confirmed undo
- **WHEN** the user confirms the undo preview
- **THEN** all listed items are restored

#### Scenario: Declined undo
- **WHEN** the user declines the undo preview
- **THEN** nothing is renamed and the tool exits

#### Scenario: Dry-run undo
- **WHEN** the user runs `randanarana undo -n`
- **THEN** the preview is printed and nothing is renamed

### Requirement: Restore conflicts and summary
For each entry the system SHALL restore the item when its new name exists and its original name is free. When the new name no longer exists, the system SHALL skip the entry with a note. When the original name is already taken, the system SHALL report an error for that entry and count it as failed. The system SHALL print a summary with the counts of restored, skipped, and failed items, and SHALL delete the manifest when nothing failed.

#### Scenario: Entry already gone
- **WHEN** a recorded new name no longer exists on disk
- **THEN** the entry is skipped with a note and does not count as restored or failed

#### Scenario: Original name taken
- **WHEN** a recorded original name already exists on disk
- **THEN** the entry is reported as failed and the remaining entries are still processed

#### Scenario: Manifest kept on failure
- **WHEN** at least one entry fails to restore
- **THEN** the manifest is kept so the user can retry

#### Scenario: Manifest deleted on success
- **WHEN** all entries restore without failures
- **THEN** the manifest is deleted

### Requirement: Undo exit codes
The system SHALL exit with status 0 when undo succeeds or finds nothing to do, and with a nonzero status on errors such as a corrupt or unreadable manifest.

#### Scenario: Successful undo
- **WHEN** undo completes
- **THEN** the tool prints a summary and exits with status 0

#### Scenario: Corrupt manifest
- **WHEN** the manifest cannot be parsed
- **THEN** the tool prints an error and exits with a nonzero status
