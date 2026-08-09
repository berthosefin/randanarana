## Why

The current `randanarana` tool is a bash script (`~/Projets/dots/home/.bin/randanarana`, symlinked into `~/.bin`) that renames files to random alphanumeric names while keeping their extensions. It works but is hard to maintain: `$RANDOM`-based name generation, no automated tests, no packaging, and it only runs where bash is available. Rewriting it in Rust makes it faster, safer, installable via `cargo install`, testable, and — most importantly — a practical Rust learning project for the author (finished *The Rust Book* up to chapter 9 and rustlings through exercise 47). Publishing it on GitHub as a documented open-source project follows the pattern of the author's other projects (e.g. the `dots` repo).

Beyond the functional goal, the author's personal goal is first-class: learning Rust and mastering it deeply. The author writes as much of the code as possible themselves and receives mentorship, code review, and corrections; the workflow is as much about the process as the result.

## What Changes

- Create a new Rust crate (binary) at the repo root of `/home/thos/Projets/randanarana` with a `src/` layout.
- Port the existing CLI (`-l/--length`, `-p/--prefix`, `-s/--suffix`, `-r/--recursive`, `-D/--dirs`, `-i/--interactive`, `-f/--force`, `-n/--dry-run`, `-h/--help`) as a drop-in replacement.
- Port existing behavior with robustness improvements:
  - Guaranteed-unique random name generation (reserving existing names, regenerating on collision).
  - Skips already-random items unless `--force`; keeps extensions; supports prefixes/suffixes.
  - Preview (truncated after 20 lines), global confirmation, and interactive per-item mode (`y/N/a/q`).
  - Summary line (`Done: X renamed, Y skipped, Z failed`) and interrupted-run summary.
  - Robust error handling instead of fragile string handling (clear errors, nonzero exit codes).
- Add unit + integration tests and a GitHub Actions CI workflow (fmt, clippy, test).
- Add project documentation: English `README.md` (what it does, how to install/use) and a `LICENSE`.
- The existing bash script is left untouched; replacing `~/.bin/randanarana` with the compiled binary is out of scope for this change (handled separately during deployment).

## Capabilities

### New Capabilities
- `cli`: Command-line interface — flag parsing, help text, exit codes, and running modes (preview/dry-run, interactive, confirm-then-rename).
- `name-generation`: Random name generation — charset/length, prefix/suffix, extension preservation, guaranteed uniqueness against existing names.
- `renaming`: File discovery and renaming — recursive/directory collection, skip-vs-rename partitioning, dry-run preview, executing renames, per-item and global confirmation, summary reporting.

### Modified Capabilities
(No existing specs — brand-new project.)

## Impact

- New project files: `Cargo.toml`, `src/*.rs`, `tests/`, `.github/workflows/ci.yml`, `README.md`, `LICENSE`, `.gitignore`.
- External dependency: `rand` (or std-only approach) — decision recorded in design.md.
- Replaces the behavior of the bash script when deployed; the bash script in the `dots` repo remains for now.
- First commit(s) to the `randanarana` GitHub repository.
