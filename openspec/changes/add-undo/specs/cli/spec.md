## Purpose

Defines the command-line interface of the randanarana tool: how users invoke it, which options are accepted, what help is shown, and the exit codes and run modes it supports.

## MODIFIED Requirements

### Requirement: Invocation and required directory
The system SHALL accept exactly one directory argument and a set of options, invoked either directly (`randanarana [OPTIONS] <DIRECTORY>`) or through the `rename` subcommand (`randanarana rename [OPTIONS] <DIRECTORY>`), which SHALL behave identically. The system SHALL also accept the `undo` subcommand (`randanarana undo [OPTIONS] [DIRECTORY]`) where the directory defaults to the current directory. It SHALL exit with a nonzero status and print an error when the directory argument is missing, when more than one directory is given, or when the given path is not a directory.

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

#### Scenario: Rename subcommand is the default
- **WHEN** the user runs `randanarana rename <DIRECTORY>` or `randanarana <DIRECTORY>`
- **THEN** both invocations rename files with the same behavior

#### Scenario: Undo subcommand
- **WHEN** the user runs `randanarana undo` without a directory
- **THEN** the tool operates on the current directory
