# Contributing to claude-squeeze

Thanks for helping. Issues and pull requests are welcome in English, 한국어, 日本語, 中文, Español or Français.

## Reporting

[Open an issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) and pick a form:

- **Bug report**: squeeze crashed, printed something wrong, or broke a session
- **Compression problem**: an important line was dropped, too much was kept, or a command was (not) redirected
- **Install, update or version problem**: the installer failed, a release file is missing, or an update broke something
- **Feature idea**: something squeeze should do

Questions, troubleshooting and roadmap talk (including what v1.0 should include) go to [Discussions](https://github.com/jongwooish/claude-squeeze/discussions). New versions are announced there and in [Releases](https://github.com/jongwooish/claude-squeeze/releases).

Remove secrets and personal paths before pasting logs. Security problems go through [SECURITY.md](SECURITY.md), not public issues.

## Code

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo fmt --manifest-path CARGO/Cargo.toml --check
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench V0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures
```

- No new dependencies: squeeze uses the standard library only.
- Hooks must never approve a tool call and must stay silent on any error.
- If the compressor changes, re-run the bench and update the numbers in every README (see [CLAUDE.md](../CLAUDE.md)).

## Test fixtures

A real log that squeeze handles badly is the best contribution. Add it as `V0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures/<name>.log` (secrets and personal paths removed). The tests fail if any distinct error or warning in it is lost, so a fixture keeps the bug fixed for good.
