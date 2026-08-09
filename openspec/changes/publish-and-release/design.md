## Context

The crate is `randanarana` 0.1.0 (edition 2024, `description`/`license` already set), published on GitHub with CI (fmt/clippy/test). The name is available on crates.io (verified 404). The tool has no spec-level behavior change, so this change is pure tooling/CI/docs (`skip_specs: true`). See proposal.md - Why.

## Goals / Non-Goals

**Goals:**
- A crate ready for crates.io and docs.rs (complete metadata, `cargo publish --dry-run` clean).
- An automated release pipeline: version bump + CHANGELOG + crates.io publish (release-plz) and prebuilt binaries + GitHub Release (cargo-dist).
- README that explains the new install paths and shows the project is maintained (badges).

**Non-Goals:**
- The actual first publish (requires the author's crates.io account + token). This change makes it possible and documents it.
- Homebrew/AUR/Nix packaging (community, follow-up).
- Renaming the crate (name is free).

## Decisions

**D1. `release-plz` for versioning + changelog + crates.io.** `release-plz init` generates `.github/workflows/release-plz.yml`. Workflow: on every push to `main`, release-plz opens a release PR (bumps version per conventional commits, updates `CHANGELOG.md`); when that PR is merged, it tags `vX.Y.Z` and runs `cargo publish`. Requires the `CARGO_REGISTRY_TOKEN` secret. Alternatives: `cargo release` (manual, less GitHub-native), hand-rolled workflow (maintenance burden). release-plz is the community standard and keeps its generated workflow/config maintained by the tool.

**D2. `cargo-dist` for prebuilt binaries.** `cargo dist init` adds a `[package.metadata.dist]` section and generates `.github/workflows/release.yml` that, on `v*` tags (created by release-plz), builds the standard targets (Linux x64/aarch64, macOS x64/aarch64, Windows x64), attaches them to the GitHub Release, and provides `cargo binstall`-compatible artifacts plus install scripts. Alternatives: a manual `cargo build --release --target ...` + `action-gh-release` workflow (works but reinvents cargo-dist's cross-compile matrix and installer generation). cargo-dist keeps the pipeline robust and standardized.

**D3. Metadata.** `repository` and `homepage` = `https://github.com/berthosefin/randanarana`; `keywords = ["rename", "random", "files"]`; `categories = ["command-line-utilities"]`; `rust-version = "1.85"` (first stable with edition 2024; lower MSRV would require rechecking clap/rand). Versioning stays `0.1.0`; the first release PR bumps it.

**D4. README.** Add shields.io badges (crates.io version, downloads, docs.rs, CI) and rewrite the Install section: `cargo install randanarana` (crates.io), `cargo binstall randanarana` (prebuilt), plus a link to GitHub Releases for direct binaries. Keep the existing `cargo install --path .` note for contributors.

## Risks / Trade-offs

- [First real publish blocked on author's crates.io account/token] → The change completes everything up to `cargo publish --dry-run`; the final publish and the `CARGO_REGISTRY_TOKEN` secret are documented manual steps.
- [`randanarana` name could be taken between now and publish] → Verified available today; if it becomes taken, renaming is a separate change (flagged, not handled here).
- [Generated workflows from release-plz/cargo-dist may evolve with tool versions] → Committing the generated files pins them; the tools also provide upgrade commands for later maintenance.
- [cargo-dist builds on the repo's toolchain (edition 2024, nightly locally)] → Generated workflows pin a stable toolchain, which supports edition 2024 since 1.85; the existing CI already proves stable builds.

## Migration Plan

- Apply: add metadata → `cargo publish --dry-run` → init cargo-dist and release-plz, commit generated files → README update → CI green.
- No runtime migration: the binary's behavior is unchanged.
- Rollback: revert the commit; no external service is touched until the author runs the first real release.

## Open Questions

None — the remaining unknowns (exact generated workflow content, crates.io token provisioning) are tool outputs and author actions, not spec/approach decisions.
