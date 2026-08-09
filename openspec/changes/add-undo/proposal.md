## Why

Renaming files is risky: one wrong confirm and a batch of files gets random names with no way back. An `undo` command removes that fear and makes the tool safe enough to use on real data, a key differentiator for a file renamer.

## What Changes

- **Manifest of renames**: when a rename run actually renames items, the tool writes a hidden JSON manifest `.randanarana-undo.json` in the target directory recording every performed rename (relative original path and relative new path). Dry runs, cancelled runs, and runs that rename nothing do not write a manifest. A new run overwrites the previous manifest, so undo always restores the most recent run.
- **`undo` command**: `randanarana undo [DIR]` reads the manifest and renames every recorded item back to its original name, printing a preview and asking for confirmation before restoring (`y/N`). It supports `--dry-run` (`-n`) to preview without touching anything.
- **Conflict handling**: an entry is skipped with a note when its target no longer exists; it counts as failed when the original name is already taken. The manifest is deleted only when the whole run restored without failures; otherwise it is kept so the user can retry.
- **CLI restructured as subcommands** (backward compatible): `randanarana [rename] [DIR]` stays the default behavior, `randanarana undo [DIR]` is new.

## Capabilities

### New Capabilities
- `manifest`: the `.randanarana-undo.json` file — its location, JSON format, when it is written, and its overwrite/delete lifecycle.
- `undo`: the `randanarana undo` command — reading the manifest, restoring items, conflict handling, summary, and exit codes.

### Modified Capabilities
- `cli`: the CLI gains a subcommand structure (`rename` default + `undo`) while keeping the current flags and behaviors.
- `renaming`: a successful rename run records every performed rename in the manifest file.

## Impact

- **Code**: `src/cli.rs` (subcommand parsing), `src/main.rs` (dispatch + manifest writing on run/interrupt), new `src/manifest.rs` (serialize/deserialize), new `src/undo.rs` (restore logic).
- **Dependencies**: `serde` + `serde_json` (only if we do not hand-roll JSON; decide in design).
- **Tests**: unit tests for manifest round-trip and conflict classification; integration tests for the full undo flow (rename, undo, conflicts, dry-run, exit codes).
- **Docs**: README gains an `undo` section and example.
- **Non-breaking**: existing invocation `randanarana [OPTIONS] DIR` keeps working; existing integration tests must keep passing unchanged.
