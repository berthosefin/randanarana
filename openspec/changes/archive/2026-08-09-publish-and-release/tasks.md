## 1. Crate metadata and dry-run

- [x] 1.1 Add to `Cargo.toml`: `repository`, `homepage`, `keywords = ["rename", "random", "files"]`, `categories = ["command-line-utilities"]`, `rust-version = "1.85"`
- [x] 1.2 Run `cargo publish --dry-run` and fix anything it reports (missing metadata, packaging issues)

## 2. cargo-dist (prebuilt binaries)

- [x] 2.1 Install `cargo-dist` CLI (prebuilt 0.32.0, compiled install was too slow)
- [x] 2.2 Run `cargo dist init` and commit the generated `.github/workflows/release.yml` and the dist config (cargo-dist 0.32 writes `dist-workspace.toml` instead of `[package.metadata.dist]`; installers set to `shell` + `powershell`, 5 standard targets)
- [x] 2.3 Verify `cargo dist plan` succeeds and the generated release workflow is valid

## 3. release-plz (versioning + changelog + crates.io)

- [x] 3.1 Install `release-plz` CLI (prebuilt 0.3.160)
- [x] 3.2 Run `release-plz init` and commit the generated `.github/workflows/release-plz.yml` (no config file created; set `persist-credentials: false` to match the tool's recommendation)
- [x] 3.3 Document (in the README "Releasing" section) the author steps: `cargo login`, then add the crates.io token as the `CARGO_REGISTRY_TOKEN` secret on GitHub (plus the `RELEASE_PLZ_TOKEN` PAT)

## 4. Docs

- [x] 4.1 Add shields.io badges to `README.md`: crates.io version, crates.io downloads, docs.rs, CI
- [x] 4.2 Rewrite the Install section: `cargo install randanarana`, `cargo binstall randanarana`, GitHub Releases binaries; keep the contributor `cargo install --path .` note

## 5. Verification

- [x] 5.1 `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` all green
- [x] 5.2 `cargo publish --dry-run` still clean and `cargo dist plan` succeeds
- [x] 5.3 CI workflows all pass on a pushed branch/PR (fixed the release-plz workflow skip: job-level `if` cannot use `secrets`/`env`, moved to step-level guards on `env.RELEASE_PLZ_TOKEN != ''`)
