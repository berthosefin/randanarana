## Context

`randanarana` generates random names from a 62-character charset. Today `NameGenerator::generate` loops until it draws a body not in `used` (`src/names.rs:76`). When the requested `--length` is too small for the number of items, no unique body can ever be found and the loop never terminates — the tool hangs with no message. There is no feasibility check anywhere: `plan` reserves every original name, then draws one body per item to rename (`src/renamer.rs:90`). See proposal.md - Why.

## Goals / Non-Goals

**Goals:**
- Refuse to run, with a clear error and nonzero exit, whenever a rename cannot be completed (this is a preventive guard, not a fix for partial failure).
- The check must be exact for every realistic case: accept every run that can terminate, reject every run that cannot.
- Renaming nothing and writing no manifest on the error path.

**Non-Goals:**
- Retrying with a shorter/longer length automatically.
- Reporting the size of the search space at runtime for huge lengths (only relevant when the space is limited).
- Interactive fallback (pick length again, proceed anyway).

## Decisions

**D1. Capacity helper in `src/names.rs`.** `pub fn max_names(length: usize) -> Option<u64>` = `62u64.checked_pow(length as u32)`. Returns `None` for length ≥ 11 (62^11 > u64::MAX), meaning "no practical limit" — the guard is skipped, since no realistic file count can exhaust the space and the collision loop is bounded by 62^length draws. Single source of truth for the 62-char charset (kept next to `CHARSET`); unit-tested for exact values (`62^1 = 62`, `62^2 = 3844`, overflow at 11).

**D2. Per-pool feasibility check in `plan`, which becomes `Result<Plan>`.** A pool is the set of items sharing an extension, plus one pool for directories and extensionless files together — exactly the partition `generate` already uses via `extension_with_dot` (`src/names.rs:70-75`). This partition is exact for termination, not merely safe: candidates always carry their pool's extension (or none), so the shared `used` set never consumes a body across pools.

`plan` computes, per pool: `orig` = number of items (files + dirs) in the pool (all reserve a body), `gen` = number of items in the pool to rename (non-skipped; with `--force`, `gen` = `orig`). If any pool has `gen > 0` and `max_names(length)` is `Some(limit)` with `orig + gen > limit`, the run is rejected. The check runs before any generation or renaming, so nothing is modified and no manifest is written. `skipped` items reserve but never generate, so they only count toward `orig`.

```rust
// in plan, before generating:
struct Pool { orig: u64, gen: u64 }
// key = extension_with_dot(name) for files, None for dirs
```

**D3. Error message.** Printed to stderr, one line, actionable, stating the numbers:

```
Error: cannot rename with --length 1: ".jpg" needs 64 unique names (32 existing + 32 to rename) but only 62^1 = 62 are possible. Use a longer --length.
```

`main.rs` propagates it via `?` (already `anyhow::Result`, so the exit code is already nonzero); nothing else changes in `main.rs`. `anyhow` is already a dependency — no new deps.

**D4. Test strategy.** Unit tests in `src/names.rs` for `max_names`; unit tests in `src/renamer.rs` for the per-pool counting (two pools both fitting under `--length 1`; one pool overflowing; `--force` turning an acceptable run into a rejected one; directories + extensionless files sharing one pool). Integration tests in `tests/cli.rs`:
- `-l 1` with 32 `.jpg` files → exit nonzero, stderr mentions the length and the pool, nothing renamed, no `.randanarana-undo.json` written.
- `-l 2` with 100 `.jpg` files → exit 0, all renamed.
- mixed extensions where each pool fits → success.

## Risks / Trade-offs

- [`max_names` overflow (length ≥ 11) disables the guard] → Acceptable: 62^11 ≈ 5.2e19 bodies per pool, unreachable with any real filesystem; the guard only exists where the space is actually exhaustible.
- [Existing run that relied on the silent hang] → None: a run that could terminate always does, and the guard accepts it. Only runs that were doomed to hang are now rejected.
- [Counting uses `basename` which returns `""` for non-UTF-8 names] → Same limitation as the rest of the tool (`src/renamer.rs:170`); non-UTF-8 names are edge cases and `generate` receives the same basename today.
- [Rejected run reserves originals into the generator before failing] → Harmless: the generator is dropped when the error propagates and nothing is renamed.

## Migration Plan

- Add the guard inside `plan` and make it return `anyhow::Result<Plan>`; update the two existing `plan` unit tests to `?`/`unwrap`; keep `main.rs` behavior for successful runs identical.
- Rollback: revert the commit. No data migration — nothing new is persisted.
