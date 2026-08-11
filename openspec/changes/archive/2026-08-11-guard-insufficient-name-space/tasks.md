## 1. Capacity helper (capability: name-generation)

- [x] 1.1 Add `max_names(length: usize) -> Option<u64>` = `62u64.checked_pow(length as u32)` in `src/names.rs` next to `CHARSET`, with `None` for overflow (length ≥ 11)
- [x] 1.2 Unit tests in `src/names.rs`: `Some(62)` for length 1, `Some(3844)` for length 2, `None` for length 11, monotonic growth

## 2. Per-pool feasibility guard (capability: name-generation)

- [x] 2.1 Change `plan` to return `anyhow::Result<Plan>` (`src/renamer.rs`)
- [x] 2.2 Count per pool (key = `extension_with_dot(basename)` for files, `None` for directories and extensionless files): `orig` = items in pool, `gen` = items to rename in pool (non-skipped; `--force` ⇒ `gen == orig`)
- [x] 2.3 Reject with `anyhow::bail!` (error message per design D3, mentioning `--length`, the pool, and the needed vs `62^LENGTH` available numbers) when a pool has `gen > 0` and `max_names(length).is_some_and(|limit| orig + gen > limit)`, before any generation or renaming
- [x] 2.4 Update the existing `plan` unit tests (`plan_skips_already_random_unless_force`, `plan_respects_prefix_and_suffix`, `plan_skips_already_random_directories`) for the `Result` return
- [x] 2.5 Add `plan` unit tests: two pools each fitting under `--length 1` succeed; one pool overflowing fails; `--force` flips an acceptable run into a rejected one; directories + extensionless files share one pool

## 3. Error propagation in main (capability: cli)

- [x] 3.1 Propagate the `plan` error from `run_rename` via `?` (already `anyhow::Result`, exit already nonzero; no other change in `src/main.rs`)

## 4. Tests

- [x] 4.1 Integration (`tests/cli.rs`): `-l 1` with 32 `.jpg` files → nonzero exit, stderr mentions the length and pool, nothing renamed, no `.randanarana-undo.json` created
- [x] 4.2 Integration: `-l 2` with 100 `.jpg` files → exit 0, all 100 renamed
- [x] 4.3 Integration: mixed extensions (`--length 1` with 20 `.jpg` + 20 `.png`) → exit 0
- [x] 4.4 Verify existing integration tests pass unchanged (25 current tests, incl. `invalid_length_is_rejected`)

## 5. Verification

- [x] 5.1 `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` all green
- [x] 5.2 Manual end-to-end: `-l 1` on 32 `.jpg` scratch files → clear error, exit 1, nothing renamed; `-l 2` on 100 files → success
