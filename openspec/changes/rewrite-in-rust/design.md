## Context

The existing `randanarana` is a bash script that renames files/directories in a target directory to random alphanumeric names while preserving extensions (see proposal.md - Why). This change ports it to a Rust binary crate at the repo root and adds tests, CI, and open-source docs. The author is learning Rust (Book chapters 1-9: ownership, structs, enums, modules, collections, error handling; rustlings 1-47), so the design favors idiomatic but accessible patterns — and the author writes as much code as possible themselves, with review and correction. `openspec/specs/` is empty — there are no existing specs.

## Goals / Non-Goals

**Goals:**
- Faithful drop-in CLI parity with the bash script (same flags, same observable behavior).
- Robust, idiomatic Rust: clear error handling, guaranteed-unique names, no collisions.
- Testable core logic (name generation, partitioning, matching) plus end-to-end CLI tests.
- CI (fmt, clippy, test) and an English README for open-source publishing.
- A learning-friendly structure: small, focused modules the author can write, understand, and get reviewed.

**Non-Goals:**
- New CLI features beyond the bash script (no new flags in this change).
- Replacing `~/.bin/randanarana` / dots deployment (handled later).
- Performance tuning or parallelism for huge trees.

## Decisions

**D1. Crate layout.** Single binary crate at the repo root named `randanarana`, edition 2024. Modules: `main.rs` (wiring, exit codes), `cli.rs` (clap definitions), `names.rs` (generation + pattern matching), `renamer.rs` (discovery, partitioning, preview, execution). Mirrors the four specs and keeps the learning curve flat. Alternative (one big `main.rs`) rejected: harder to test and read.

**D2. CLI parsing with `clap` (derive).** Standard, gives free `--help`, validation, and consistent errors; the derive API is beginner-friendly and showcases `#[derive]`. Alternatives: hand-rolled `std::env::args` (educational but error-prone), `getopts` (less ergonomic). Clap's generated help differs cosmetically from the bash `usage` text — acceptable (spec only requires help to exist and exit 0).

**D3. Randomness with the `rand` crate.** `StdRng` seeded from OS entropy. Correct and uniform vs bash `$RANDOM`; std has no RNG. Alternative: hand-written PRNG (rejected: hard to get right, no benefit).

**D4. Directory walking with std `fs::read_dir`.** Manual recursion covers the few needed behaviors (recursive, hidden-skip, bottom-up dirs) without adding deps and is instructive — the author exercises recursion and iterators. Alternative: `walkdir` (rejected: extra dep, and the custom bottom-up ordering for `--dirs` is a small amount of code).

**D5. Error handling with `anyhow`.** `anyhow::Result` + `.context()` for application-level errors (invalid dir, io failures). Idiomatic for binaries; keeps focus on flow. Alternative: custom error enum (more boilerplate than needed here). The author already studied `Result`/error handling in the Book; `anyhow` shows the real-world pattern.

**D6. Uniqueness via `HashSet`.** Reserve all original basenames, then generate against the set, inserting each new name, regenerating on collision — the same algorithm as bash. Simple and deterministic. Alternative: counting/suffix schemes (rejected: changes observable names).

**D7. Hidden-file edge case.** Bash treats a leading-dot file like `.bashrc` with its whole name as "extension"; Rust `Path::extension()` returns `None` for it. Moot because hidden items are skipped during discovery; for non-hidden files extension = everything after the last dot, if any.

**D8. Integration tests.** `assert_cmd` + `predicates` + `tempfile` as dev-dependencies; `assert_cmd` runs the built binary via `CARGO_BIN_EXE`. Unit tests cover `names.rs` and `renamer.rs` matching/partitioning. Alternative: std-only `std::process::Command` + hand-made temp dirs (less convenient).

**D9. CI.** GitHub Actions on `ubuntu-latest`, stable toolchain: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.

**D10. Interrupt handling.** Interactive mode: a read error/EOF on stdin (user pressed Ctrl+C) → print summary and exit 130, matching bash. Non-interactive runs use default SIGINT behavior (no summary, exit 130 via signal) — no signal crate. Alternative: `ctrlc` crate for graceful summary everywhere (rejected: extra dep, small benefit).

## Risks / Trade-offs

- **Behavior parity with bash** → Covered by integration tests porting each behavior (dry-run, force, dirs, interactive, recursive), reviewed side by side with the bash script during implementation.
- **`clap` help/error text differs from bash** → Acceptable; spec requires help + nonzero-on-error only, not byte-identical text.
- **Dependency count** (clap, rand, anyhow + dev deps) → Kept to 3 runtime deps; std used where reasonable.
- **Edition 2024 needs a recent toolchain** → CI pins stable; README documents `rustup update`.
- **Author-written code may need several review rounds** → Not a risk to the design; tasks are small so each can be written, reviewed, and corrected before moving on.

## Migration Plan

- Build the crate at the repo root; leave the bash script untouched in dots.
- Publish the GitHub repo and push. Later (out of scope): `cargo install --path .` and replace the `~/.bin` symlink.
- Rollback: bash script remains in dots; the symlink only changes in a later deployment step.

## Open Questions

None — remaining unknowns (version pinning, exact help wording, LICENSE choice) don't change the specs, approach, or task breakdown.
