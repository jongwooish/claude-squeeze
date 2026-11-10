# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Layout

- One folder per version: `v0.1-Sentinel/v0.1.0_squeeze-sentinel/` (frozen v0.1.0 snapshot, standalone crate `squeeze-sentinel`), `v0.2-Auditor/v0.2.0_squeeze-auditor/` (the current `squeeze` crate). Planned: v0.3 (browser and mobile).
- The Rust workspace manifest is `CARGO/Cargo.toml`; run cargo with `--manifest-path CARGO/Cargo.toml` (build output goes to `CARGO/target/`). Its only member is the current crate. The v0.1 snapshot has its own `[workspace]` and is tested in CI with `--manifest-path v0.1-Sentinel/v0.1.0_squeeze-sentinel/Cargo.toml`. Do not change the snapshot except for docs that point to newer versions.
- The current crate builds two programs: `squeeze` (CLI and hook entry points) and `squeeze-auditor` (the window; a GUI-subsystem program on Windows). The window's page is `ui/index.html`, embedded at build time. Installers are in `packaging/` (Inno Setup, macOS `.pkg`, Linux `.deb`) and are built by `.github/workflows/release.yml` (and by CI on every push).
- Layout: `README/` (project README in 6 languages, `README.md` = English), `CHANGELOG/` (English `CHANGELOG.md` is read by the release workflow; its `## [x.y.z] - date — Name` heading gives the release title), `INSTALL/` (install scripts), `CARGO/` (workspace `Cargo.toml` and `Cargo.lock`). `.github/README.md` is a copy of `README/README.md` with links adjusted, so GitHub shows it on the front page: regenerate it whenever `README/README.md` changes. Each version crate has its English `README.md` at the top of the folder and translations in `README/`. Each version folder's `README.md` (e.g. `v0.2-Auditor/README.md`) is a copy of the crate README with links adjusted: regenerate it whenever that file changes. When you change one README, update all six languages, keeping the same sections and numbers.

## Rules

- **No dependencies.** Standard library only (`json.rs` is our own parser). Do not add crates.
- Hooks must never approve a tool call and must fail open (no output, exit 0) on any error.
- squeeze-Auditor listens on `127.0.0.1` only and every API call needs the per-launch token. Never read Claude credentials.
- Back up any Claude config file before changing it (`paths::backup`).
- Keep `cargo fmt`, `cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings` and `cargo test` clean.
- Benchmark numbers in READMEs come from `cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures --markdown`. Re-run it and update every README if the compressor changes. Never type numbers by hand.
- Fixtures and screenshots must not contain secrets or personal paths.

## Commands

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures --markdown
cargo run --manifest-path CARGO/Cargo.toml -- auditor
```
