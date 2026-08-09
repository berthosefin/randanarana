# cli Specification

## Purpose

Defines the command-line interface of the randanarana tool: how users invoke it, which options are accepted, what help is shown, and the exit codes and run modes it supports.

## Requirements

### Requirement: Invocation and required directory
The system SHALL accept exactly one directory argument and a set of options. It SHALL exit with a nonzero status and print an error when the directory argument is missing, when more than one directory is given, or when the given path is not a directory.

#### Scenario: Directory provided
- **WHEN** the user runs the tool with a valid directory path
- **THEN** the tool proceeds to process that directory

#### Scenario: Missing directory
- **WHEN** the user runs the tool without a directory argument
- **THEN** the tool prints an error and exits with a nonzero status

#### Scenario: More than one directory
- **WHEN** the user runs the tool with two directory arguments
- **THEN** the tool prints an error and exits with a nonzero status

#### Scenario: Path is not a directory
- **WHEN** the user runs the tool with a path that is not a directory
- **THEN** the tool prints an error and exits with a nonzero status

### Requirement: Options
The system SHALL support these options:
- `-l, --length N`: length of the random part (default 8)
- `-p, --prefix P`: prefix prepended to generated names
- `-s, --suffix S`: suffix appended to generated names
- `-r, --recursive`: process files in subdirectories, keeping subdirectory names
- `-D, --dirs`: also rename subdirectories (implies `--recursive`)
- `-i, --interactive`: confirm each item individually
- `-f, --force`: rename items that already match the random pattern
- `-n, --dry-run`: show the preview without renaming
- `-h, --help`: show help and exit

#### Scenario: Length validation
- **WHEN** the user passes `--length` with a non-positive or non-numeric value
- **THEN** the tool prints an error and exits with a nonzero status

#### Scenario: Unknown option
- **WHEN** the user passes an option that is not recognized
- **THEN** the tool prints an error and exits with a nonzero status

### Requirement: Help output
The system SHALL print usage information describing the tool and all options when `-h` or `--help` is passed, and SHALL exit with status 0.

#### Scenario: Help requested
- **WHEN** the user passes `-h` or `--help`
- **THEN** the tool prints usage information and exits with status 0

### Requirement: Run modes
The system SHALL support three run modes selected by options:
- Dry-run (`-n`): print the preview and exit without renaming
- Interactive (`-i`): present each rename and wait for per-item confirmation
- Default: print the preview, then ask for a single confirmation before renaming

#### Scenario: Dry-run preview
- **WHEN** the user runs with `-n`
- **THEN** the tool prints the preview and does not rename anything

#### Scenario: Interactive mode
- **WHEN** the user runs with `-i`
- **THEN** the tool presents each item and only renames it when the user answers `y` or `a`

#### Scenario: Confirmation declined
- **WHEN** the user answers `n` (or anything but `y`) to the global confirmation
- **THEN** the tool cancels without renaming and exits

### Requirement: Exit codes
The system SHALL exit with status 0 on success and help, and with a nonzero status on errors. When the user interrupts an interactive run, the system SHALL print a summary and exit with status 130.

#### Scenario: Successful run
- **WHEN** the renames complete successfully
- **THEN** the tool prints a summary and exits with status 0

#### Scenario: Interrupted run
- **WHEN** the user interrupts the tool during an interactive run
- **THEN** the tool prints a summary and exits with status 130
