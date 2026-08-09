## 1. Project Setup

- [x] 1.1 Initialize a binary crate `randanarana` at the repo root (`cargo init --name randanarana`), edition 2024
- [x] 1.2 Add dependencies to `Cargo.toml`: `clap` (derive), `rand`, `anyhow`; dev-deps `assert_cmd`, `predicates`, `tempfile`
- [x] 1.3 Create `.gitignore` for `/target` and editor artifacts
- [x] 1.4 Add `.github/workflows/ci.yml` (fmt, clippy, test on stable/ubuntu-latest)

## 2. CLI (capability: cli)

- [x] 2.1 Define the clap CLI in `src/cli.rs`: `-l/--length`, `-p/--prefix`, `-s/--suffix`, `-r/--recursive`, `-D/--dirs`, `-i/--interactive`, `-f/--force`, `-n/--dry-run`, `-h/--help`, plus the positional directory
- [x] 2.2 Validate `--length` (positive integer) and required directory; wire `main.rs` to return nonzero exit codes on error
- [x] 2.3 Verify `--help` output and exit code 0

## 3. Name Generation (capability: name-generation)

- [x] 3.1 Implement the random body generator (charset a-zA-Z0-9, configurable length) in `src/names.rs`
- [x] 3.2 Implement full-name assembly: prefix + body + suffix + preserved extension (no extension for directories)
- [x] 3.3 Implement uniqueness: reserve existing basenames in a `HashSet`, regenerate on collision, insert each new name
- [x] 3.4 Implement `matches_pattern` (prefix/suffix/length/alnum check) for skip detection

## 4. Renaming Core (capability: renaming)

- [ ] 4.1 Implement discovery in `src/renamer.rs`: non-recursive files; recursive files; bottom-up dirs; skip hidden items; sorted order
- [ ] 4.2 Implement partitioning: skip already-random items unless `--force`
- [ ] 4.3 Implement the preview: relative paths, `old -> new`, truncate after 20 with a hidden-count note
- [ ] 4.4 Implement default confirm mode (preview + y/N) and interactive mode (y/N/a/q per item)
- [ ] 4.5 Implement rename execution with per-item error reporting and final summary (renamed/skipped/failed)
- [ ] 4.6 Handle interruption in interactive mode: print summary and exit 130
- [ ] 4.7 Wire all modes through `main.rs` (dry-run exits after preview; no-items messages)

## 5. Tests

- [ ] 5.1 Unit tests for name generation (format, charset, extension handling, uniqueness, collisions)
- [ ] 5.2 Unit tests for matching/partitioning (already-random skip, force, prefix/suffix cases)
- [ ] 5.3 Integration tests with `assert_cmd` + `tempfile`: dry-run, real rename, extension preserved, recursive, `--dirs`, `--force`, global decline, interactive via piped stdin
- [ ] 5.4 Verify summary counts and exit codes in integration tests

## 6. Documentation & Publishing

- [ ] 6.1 Write the English `README.md`: what the tool does, install via `cargo install`, usage/examples, options reference
- [ ] 6.2 Add a `LICENSE` (MIT — chosen by the author)
- [ ] 6.3 Ensure `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` pass locally
- [ ] 6.4 Create the GitHub repository and push the initial commit
