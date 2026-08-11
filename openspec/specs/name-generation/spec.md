# name-generation Specification

## Purpose

Generates unique random names of the form prefix + alphanumeric body + suffix + original extension, guaranteed not to collide with existing items in the target directory.

## Requirements

### Requirement: Name format
Generated names SHALL have the form `{prefix}{random body}{suffix}{original extension}`, where the random body is a sequence of `LENGTH` alphanumeric characters (a-z, A-Z, 0-9). When no prefix/suffix is given they are omitted. Directories SHALL be renamed without an extension.

#### Scenario: Default name
- **WHEN** a file `photo.jpg` is renamed with default settings
- **THEN** the new name is 8 alphanumeric characters followed by `.jpg`

#### Scenario: Prefix and suffix
- **WHEN** renaming with `--prefix img_` and `--suffix _2026`
- **THEN** the new name starts with `img_`, ends with `_2026`, and keeps the original extension

#### Scenario: Directories have no extension
- **WHEN** a directory is renamed
- **THEN** the new name is exactly `prefix + body + suffix` with no extension

### Requirement: Character set
The random body SHALL be drawn from `abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`.

#### Scenario: Allowed characters
- **WHEN** a name body is generated
- **THEN** every character is a lowercase or uppercase ASCII letter or ASCII digit

### Requirement: Uniqueness
The system SHALL generate names that do not collide with the original names in the target set or with each other. When a generated name collides, the system SHALL regenerate it until a unique name is found.

#### Scenario: Collision avoidance
- **WHEN** a generated name equals an existing item's name
- **THEN** the system generates a different name that matches no existing item

#### Scenario: No duplicate generated names
- **WHEN** multiple names are generated in one run
- **THEN** no two items receive the same new name

### Requirement: Feasibility guarantee
The system SHALL refuse to start renaming when the number of unique names required for a run cannot be generated with the requested LENGTH. The check SHALL be done per extension pool: items sharing an extension draw from one pool of `62^LENGTH` possible bodies, and directories and extensionless files form a separate shared pool. Each original item accounts for one required name, and each item to be generated accounts for another. When a pool that must generate at least one item requires more distinct names than `62^LENGTH` allows, the system SHALL print an error stating the length and the number of names needed and available, SHALL rename nothing, and SHALL exit with a nonzero status.

#### Scenario: Insufficient length
- **WHEN** the user runs with `--length 1` on a directory whose `.jpg` pool needs 64 distinct names (32 original + 32 to generate) while only 62 exist
- **THEN** the tool prints an error mentioning the length, renames nothing, and exits with a nonzero status

#### Scenario: Sufficient length
- **WHEN** the user runs with `--length 2` on a directory of 100 `.jpg` files (200 names needed, 62^2 = 3844 available)
- **THEN** the run proceeds normally and every file is renamed to a unique name

#### Scenario: Per-extension pools
- **WHEN** the user runs with `--length 1` on a directory containing 20 `.jpg` and 20 `.png` files
- **THEN** the run proceeds because each pool requires only 40 of its 62 available names

#### Scenario: One pool overflows
- **WHEN** the user runs with `--length 1` on a directory containing 60 `.jpg` and 5 `.png` files
- **THEN** the run fails because the `.jpg` pool requires 120 distinct names but only 62 exist, and nothing is renamed
