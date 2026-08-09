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
