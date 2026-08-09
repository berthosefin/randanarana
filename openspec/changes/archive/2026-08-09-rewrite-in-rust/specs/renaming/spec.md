## Purpose

Discovers the files and directories to rename, decides which items to skip, shows a preview, and performs the renames while reporting the outcome.

## ADDED Requirements

### Requirement: Target discovery
By default the system SHALL collect the non-hidden files directly inside the target directory in sorted order. With `--recursive` it SHALL also collect non-hidden files in subdirectories, keeping subdirectory names. With `--dirs` it SHALL additionally collect subdirectories bottom-up (deepest first). Hidden files and hidden directories SHALL be skipped.

#### Scenario: Non-recursive discovery
- **WHEN** the user runs without `--recursive`
- **THEN** only the files directly in the target directory are considered

#### Scenario: Recursive discovery
- **WHEN** the user runs with `--recursive`
- **THEN** files in subdirectories are also considered and subdirectory names are kept

#### Scenario: Dirs mode
- **WHEN** the user runs with `--dirs`
- **THEN** subdirectories are also renamed, deepest first

#### Scenario: Hidden items skipped
- **WHEN** the target directory contains hidden files or directories
- **THEN** they are not renamed

### Requirement: Skipping already-random items
The system SHALL skip items whose name already matches `prefix + alphanumeric(LENGTH) + suffix [+ extension]`, unless `--force` is given.

#### Scenario: Already-random file
- **WHEN** an item already matches the random pattern and `--force` is not given
- **THEN** the item is skipped and reported as skipped

#### Scenario: Force renames
- **WHEN** `--force` is given
- **THEN** items matching the random pattern are renamed anyway

### Requirement: Preview
The system SHALL print a preview listing the items to rename with their new names, paths shown relative to the target directory. When more than 20 items would be shown, the preview SHALL be truncated with a note about the hidden count.

#### Scenario: Preview content
- **WHEN** a preview is shown
- **THEN** each line shows the relative original path, an arrow, and the relative new name

#### Scenario: Preview truncation
- **WHEN** there are more than 20 items to rename
- **THEN** only the first 20 are shown followed by a note

### Requirement: Confirmation before renaming
In the default mode the system SHALL print the preview and ask for confirmation (`y`/`n`) before renaming. In interactive mode it SHALL ask for each item with `y`, `N`, `a` (yes to all), `q` (quit).

#### Scenario: Global confirmation
- **WHEN** the user confirms the preview
- **THEN** all listed items are renamed

#### Scenario: Global decline
- **WHEN** the user declines the preview
- **THEN** nothing is renamed and the tool exits

#### Scenario: Interactive answers
- **WHEN** the user answers `a` in interactive mode
- **THEN** the remaining items are renamed without further prompts

#### Scenario: Quit in interactive mode
- **WHEN** the user answers `q` in interactive mode
- **THEN** the tool stops renaming and prints a summary

### Requirement: Rename execution and summary
The system SHALL perform the renames using a standard filesystem rename and SHALL print a summary with the counts of renamed, skipped, and failed items.

#### Scenario: Successful renames
- **WHEN** renames complete
- **THEN** a summary reports how many items were renamed, skipped, and failed

#### Scenario: Rename failure
- **WHEN** an item cannot be renamed
- **THEN** an error is printed for that item, it is counted as failed, and the remaining items are still processed
