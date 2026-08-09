# manifest Specification

## Purpose

Defines the `.randanarana-undo.json` manifest file that records every performed rename, so a later run can restore the previous state of the directory.

## Requirements

### Requirement: Manifest file location and lifecycle
A rename run that renames at least one item SHALL write a JSON manifest named `.randanarana-undo.json` in the target directory. The file SHALL start with a dot so discovery never treats it as an item to rename. Dry runs, cancelled runs, and runs that rename nothing SHALL NOT write a manifest. Each new rename run SHALL overwrite the previous manifest, so undo always restores the most recent run.

#### Scenario: Manifest written after a successful run
- **WHEN** a rename run renames at least one item
- **THEN** a file `.randanarana-undo.json` exists in the target directory describing that run

#### Scenario: No manifest on dry-run or cancellation
- **WHEN** the user runs with `--dry-run` or declines the confirmation
- **THEN** no manifest file is created or modified

#### Scenario: Overwrite on next run
- **WHEN** a new rename run renames at least one item
- **THEN** the manifest reflects only the new run

### Requirement: Manifest content
The manifest SHALL record the tool version, a creation timestamp, and, for every item actually renamed, the item's path relative to the target directory before and after the rename.

#### Scenario: Records every performed rename
- **WHEN** a run renames several items, including items in subdirectories
- **THEN** the manifest lists each item with its relative original path and relative new path
