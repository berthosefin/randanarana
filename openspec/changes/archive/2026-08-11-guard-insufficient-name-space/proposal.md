## Why

When `--length` is too small for the number of items being renamed, the name generator loops forever: it draws random names until one is unique, but when fewer than `62^LENGTH` distinct names can exist (each item needs an original *and* a new name in the same pool), no unique name can ever be found and the tool hangs silently with no message (e.g. `-l 1` with 32 `.jpg` files needs 64 names but only 62 exist).

## What Changes

- **Feasibility check before generating**: before planning names, the tool counts, per extension pool (files sharing an extension; directories and extensionless files together), how many distinct names a run would need — original names reserved plus names to generate. When a pool needs more names than `62^LENGTH` allows and at least one name must be generated there, the tool prints a clear error (stating the length and the numbers involved) and exits with a nonzero status, renaming nothing and writing no manifest.
- **No hang**: the collision-retry loop becomes guaranteed to terminate, since every pool is known to have enough room before generation starts.

## Capabilities

### New Capabilities
- *(none)*

### Modified Capabilities
- `name-generation`: a new *Feasibility guarantee* requirement — refuse to run when an extension pool requires more distinct names than `62^LENGTH`, with scenarios for insufficient/sufficient length and per-extension pooling.
- `cli`: a new scenario under *Exit codes* — an insufficient name space prints an error and exits nonzero without renaming anything.

## Impact

- **Code**: `src/names.rs` (capacity helper), `src/renamer.rs` (`plan` gains the per-pool check and returns `anyhow::Result<Plan>`), `src/main.rs` (propagates the error via `?`; no other change).
- **Dependencies**: none (uses `anyhow`, already a dependency).
- **Tests**: unit tests for the capacity math and per-pool counting; integration tests for the error path (`-l 1` with too many files → exit 1, nothing renamed, no manifest) and the working path (`-l 2` with 100 files → success).
- **Docs**: none required (the error message is self-explanatory).
- **Non-breaking**: existing invocations keep working; existing tests must pass unchanged.
