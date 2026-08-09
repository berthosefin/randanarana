## Context

`randanarana` is a single-command binary (positional `TARGET` + flags). To support undo without breaking the existing invocation, the CLI gains a subcommand structure where `rename` stays the default. Undo needs a durable record of every performed rename. See proposal.md - Why.

## Goals / Non-Goals

**Goals:**
- Non-breaking CLI: `randanarana [OPTIONS] <DIR>` keeps working exactly as before.
- A hidden, per-directory manifest that survives across runs so undo is always possible for the most recent run.
- Undo that is safe: preview + confirmation, per-item conflict classification, exit codes matching the tool's conventions.

**Non-Goals:**
- Multi-run undo history (stack). Only the most recent run is undone.
- Global (XDG) history or cross-machine undo.
- Interactive per-item undo prompts (single global confirmation only).

## Decisions

**D1. CLI subcommands with a default via clap derive.** Use the documented clap pattern for a default subcommand (clap discussion #4134 / PR #4350):

```rust
#[derive(Parser)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[command(flatten)]
    rename: RenameArgs,
}

enum Command { Rename(RenameArgs), Undo(UndoArgs) }
```

Dispatch: `cli.command.unwrap_or(Command::Rename(cli.rename))`. With `args_conflicts_with_subcommands`, `randanarana <DIR>` and `randanarana rename <DIR>` parse the same `RenameArgs`; `randanarana undo` parses `UndoArgs` (directory optional, default `.`). `RenameArgs` carries all current flags; `UndoArgs` carries only `-n/--dry-run`. Alternatives considered: a separate `-u/--undo` flag (less discoverable), hand-rolled dispatch on a positional `"undo"` token (fragile). The `unwrap_or` pattern is the clap-endorsed approach; verified against clap 4.6 semantics, and locked by the existing integration tests (help, exit codes, all flags) which must pass unchanged.

**D2. Manifest as JSON with `serde` + `serde_json`.** New runtime deps. Shape:

```json
{
  "version": 1,
  "tool_version": "0.1.0",
  "created_at": 1730000000,
  "renamed": [{ "from": "sub/a.txt", "to": "sub/XyZ9aBc1.txt" }]
}
```

`created_at` is Unix seconds from `std::time::SystemTime` (no chrono dependency). Paths are relative to the target and stored as strings (round-trip via `PathBuf::from`). `version` guards the format for future changes; parse failure → error + exit 1. Alternatives: TOML (no manifest-specific benefit, needs the `toml` crate anyway), hand-rolled parser (error-prone, not worth it).

**D3. Manifest lifecycle — written only by real runs.** The rename flow collects entries as items are renamed (failed renames are not recorded) and persists `.randanarana-undo.json` in the target directory at the end of the loop, including on the interactive interruption path (exit 130) so partial runs are undoable. Dry runs, cancelled runs, and runs that rename nothing neither create nor modify the manifest. A run that renames ≥1 item overwrites it, so undo always restores the most recent run.

**D4. Undo flow.** `randanarana undo [DIR]` resolves the target (default `.`), reads the manifest (missing → "No renames to undo.", exit 0; unparseable → error, exit 1), prints a preview (`current -> original`, same arrow semantics as rename), asks one `y/N` confirmation, then classifies each entry in recorded order:
1. new name missing on disk → skip (note printed),
2. original name already present → failed (error printed),
3. otherwise → rename new name back to the original (restored).

Summary line `Done: X restored, Y skipped, Z failed.`; the manifest is deleted only when `failed == 0`. `-n` prints the preview and exits without touching anything.

**D5. Refactor `rename_one` to expose the destination path.** Currently `renamer::rename_one` returns `bool` and builds the new path internally. It will return `Option<PathBuf>` (the new path on success, `None` on failure) so the caller can record the `(from, to)` entry. Non-UTF-8 paths cannot be recorded faithfully and are logged as not-undoable (see Risks).

## Risks / Trade-offs

- [clap default-subcommand quirk: required args + `args_conflicts_with_subcommands` interaction] → Mitigation: the pattern is the documented one (PR #4350 makes required fields in the flattened default struct work); the existing integration test suite verifies all current invocations after the refactor.
- [Non-UTF-8 file names cannot be stored in the JSON manifest] → Such renames still happen but are logged as not undoable; document as a known limitation (rare on typical filesystems).
- [Interrupted runs (real SIGINT) in default mode kill the process without writing the manifest] → Same limitation as today (no signal crate, design D10 of the rewrite change); interactive mode writes the manifest on its 130 path, which is the primary undo use case.
- [A failed undo leaves the manifest behind, which may confuse] → It is intentional (retry support) and reported in the summary.

## Migration Plan

- Add deps (`serde`, `serde_json`) and the `manifest`/`undo` modules; restructure `cli.rs`; adjust `main.rs` dispatch. Existing integration tests must pass unchanged.
- Rollback: revert the commit. Any manifest written by an intermediate build is a hidden dot-file in the target directory and is harmless (discovery skips it).
- No data migration: manifests did not exist before this change.

## Open Questions

None — remaining unknowns (exact `undo` help wording, JSON field naming) do not change the specs, approach, or task breakdown.
