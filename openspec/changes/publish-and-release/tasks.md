## 1. Crate metadata and dry-run

- [ ] 1.1 Add to `Cargo.toml`: `repository`, `homepage`, `keywords = ["rename", "random", "files"]`, `categories = ["command-line-utilities"]`, `rust-version = "1.85"`
- [ ] 1.2 Run `cargo publish --dry-run` and fix anything it reports (missing metadata, packaging issues)

## 2. cargo-dist (prebuilt binaries)

- [ ] 2.1 Install `cargo-dist` CLI: `cargo install cargo-dist`
- [ ] 2.2 Run `cargo dist init` and commit the generated `.github/workflows/release.yml` and the `[package.metadata.dist]` section (default plan, all standard targets + installers)
- [ ] 2.3 Verify `cargo dist plan` succeeds and the generated release workflow is valid (e.g. via the repo's CI or the tool's own check)

## 3. release-plz (versioning + changelog + crates.io)

- [ ] 3.1 Install `release-plz` CLI: `cargo install release-plz`
- [ ] 3.2 Run `release-plz init` and commit the generated `.github/workflows/release-plz.yml` (and any config file it creates)
- [ ] 3.3 Document (in the PR/README) the author steps: `cargo login`, then add the crates.io token as the `CARGO_REGISTRY_TOKEN` secret on GitHub

## 4. Docs

- [ ] 4.1 Add shields.io badges to `README.md`: crates.io version, crates.io downloads, docs.rs, CI
- [ ] 4.2 Rewrite the Install section: `cargo install randanarana`, `cargo binstall randanarana`, GitHub Releases binaries; keep the contributor `cargo install --path .` note

## 5. Verification

- [ ] 5.1 `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` all green
- [ ] 5.2 `cargo publish --dry-run` still clean and `cargo dist plan` succeeds
- [ ] 5.3 CI workflows all pass on a pushed branch/PR
