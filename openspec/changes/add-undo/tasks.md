## 1. CLI restructure (capability: cli)

- [x] 1.1 Add `serde` (derive) and `serde_json` to `Cargo.toml`
- [x] 1.2 Restructure `src/cli.rs`: `Cli { dry_run (global -n), command: Option<Command>, #[command(flatten)] rename: Option<RenameArgs> }` with `args_conflicts_with_subcommands = true` (flatten must be `Option` — required positional + subcommand hits clap #3857; `-n` is shared, so it becomes global); `Command` enum with `Rename(RenameArgs)` and `Undo(UndoArgs)`; dispatch via `match command { Some(..), None => rename or missing_target() }`
- [x] 1.3 Verify all existing integration tests still pass unchanged (help, missing dir exit 2, flags, modes)

## 2. Manifest module (capability: manifest)

- [x] 2.1 Create `src/manifest.rs`: `Manifest { version, tool_version, created_at, renamed }` and `RenameEntry { from, to }` with serde derive; `version: 1`
- [x] 2.2 Implement `Manifest::write(target) -> io::Result<()>` writing `.randanarana-undo.json` (indented JSON, atomic via temp file + rename)
- [x] 2.3 Implement `Manifest::read(target)` returning `Option<Manifest>` (None when absent) and a parse error (propagated) when corrupt
- [x] 2.4 Record renames in `main.rs`/`renamer.rs`: `rename_one` returns `Option<PathBuf>` (new path on success); both rename modes collect entries; write the manifest at the end of the loop, including on the interactive interruption path (exit 130)
- [x] 2.5 Do not write/modify the manifest on dry-run, on cancellation, or when nothing was renamed; a run that renames ≥1 item overwrites it

## 3. Undo module (capability: undo)

- [x] 3.1 Create `src/undo.rs`: resolve target (arg or `.`); read manifest; "No renames to undo." exit 0 when absent; error exit 1 when corrupt (`-n` is the global `dry_run` on `Cli`)
- [x] 3.2 Implement preview (`current -> original`, truncated after 20 like rename) and a single `y/N` confirmation; `-n` prints preview and exits
- [x] 3.3 Implement classification per entry: new name missing → skip with note; original name taken → error + failed; else restore
- [x] 3.4 Print summary `Done: X restored, Y skipped, Z failed.`; delete the manifest only when `failed == 0`; exit 0

## 4. Tests

- [x] 4.1 Unit tests: manifest serde round-trip (relative paths with subdirs); classify() for restore/skip/failed cases
- [x] 4.2 Integration: rename → manifest written; undo restores files and extensions; manifest deleted after successful undo
- [x] 4.3 Integration: undo with no manifest (exit 0), corrupt manifest (exit 1), `undo -n` (nothing changed), declined undo (nothing changed)
- [x] 4.4 Integration: conflict cases — original name taken → failed + manifest kept; recursive renames undone; second rename run overwrites manifest
- [x] 4.5 Update `README.md` with an `undo` section and example

## 5. Verification

- [x] 5.1 `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` all green
- [x] 5.2 Manual end-to-end: rename a scratch dir, undo it, verify contents restored byte-for-byte
