# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Layout

- One folder per version. Now: `v0.1-Sentinel/v0.1_squeeze-sentinel/`. Planned: `v0.2_squeeze-interceptor/` (browser extension), `v0.3_squeeze-conduit/` (MCP server).
- The Rust workspace manifest is `CARGO/Cargo.toml`; run cargo with `--manifest-path CARGO/Cargo.toml` (build output goes to `CARGO/target/`). The `squeeze` crate is `v0.1-Sentinel/v0.1_squeeze-sentinel/`.
- Layout: `README/` (project README in 6 languages, `README.md` = English), `CHANGELOG/` (English `CHANGELOG.md` is read by the release workflow), `INSTALL/` (installers), `CARGO/` (Rust workspace `Cargo.toml` and `Cargo.lock`). `.github/README.md` is a copy of `README/README.md` with links adjusted, so GitHub shows it on the front page: regenerate it whenever `README/README.md` changes. `v0.1-Sentinel/v0.1_squeeze-sentinel/` has its English `README.md` at the top of the folder (so GitHub shows it) and translations in `README/`. When you change one README, update all six, keeping the same sections and numbers.

## Rules

- **No dependencies.** Standard library only (`json.rs` is our own parser). Do not add crates.
- Hooks must never approve a tool call and must fail open (no output, exit 0) on any error.
- Keep `cargo fmt`, `cargo clippy --manifest-path CARGO/Cargo.toml -- -D warnings` and `cargo test` clean.
- Benchmark numbers in READMEs come from `cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures --markdown`. Re-run it and update every README if the compressor changes. Never type numbers by hand.
- Fixtures must not contain secrets or personal paths.

## Commands

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures --markdown
```
