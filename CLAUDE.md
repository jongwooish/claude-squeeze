# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Layout

- One folder per version. Now: `v0.1_squeeze-sentinel/`. Planned: `v0.2_squeeze-interceptor/` (browser extension), `v0.3_squeeze-conduit/` (MCP server).
- Root `Cargo.toml` is a workspace. The `squeeze` binary crate is `v0.1_squeeze-sentinel/`.
- Root README: `README.md` (English, the default) plus translations in `readme/README.{ko,ja,zh-CN,es,fr}.md`. CHANGELOG: `changelog/CHANGELOG.md` (English, read by the release workflow) plus translations in the same folder. `v0.1_squeeze-sentinel/README/` holds that version's README in all six languages (English is `README.md`). When you change one, update all six, keeping the same sections and numbers.

## Rules

- **No dependencies.** Standard library only (`json.rs` is our own parser). Do not add crates.
- Hooks must never approve a tool call and must fail open (no output, exit 0) on any error.
- Keep `cargo fmt`, `cargo clippy -- -D warnings` and `cargo test` clean.
- Benchmark numbers in READMEs come from `cargo run --release -- bench v0.1_squeeze-sentinel/testdata/fixtures --markdown`. Re-run it and update every README if the compressor changes. Never type numbers by hand.
- Fixtures must not contain secrets or personal paths.

## Commands

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release -- bench v0.1_squeeze-sentinel/testdata/fixtures --markdown
```
