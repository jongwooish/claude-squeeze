# Changelog

**English** | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

All notable changes are listed here. Versions follow [Semantic Versioning](https://semver.org/). While the major version is 0, commands and config may still change.

## [0.1.0] - 2026-11-03 — Sentinel

First release. Code in [`V0.1-Sentinel/v0.1_squeeze-sentinel/`](../V0.1-Sentinel/v0.1_squeeze-sentinel/).

### Added

- Claude Code `PreToolUse` hooks:
  - **Bash guard**: redirects noisy commands (installs, builds, tests, logs) to `squeeze run`. Leaves bounded commands alone (`| tail`, `-n 50`, redirects, `--quiet`).
  - **Read guard**: redirects whole-file reads of files over 400 lines to a ranged read, with an outline.
  - Hooks never approve a tool call and fail open on any error.
- `squeeze run -- <command>`: run, compress long output, save the full log, keep the exit code.
- `squeeze compress`: compress piped output.
- `squeeze clip`: compress the clipboard for pasting into claude.ai (macOS, Windows, Linux).
- `squeeze report`: savings per project (`--global` for clip, `--json`, `--since`).
- `squeeze bench`: replay logs and measure savings and kept messages.
- `squeeze init` / `squeeze uninstall`: install and cleanly remove hooks, config, `.gitignore` entry and `CLAUDE.md` note.
- Compressor: keeps head and tail, every distinct error and warning with context, stack traces; collapses repeats; picks errors before warnings.
- Seven benchmark fixtures: 93.8% fewer estimated tokens, 50/50 distinct messages kept.
- Installers for macOS/Linux (`install.sh`) and Windows (`install.ps1`) with SHA-256 checks.
- READMEs in English, Korean, Japanese, Simplified Chinese, Spanish and French.

### Planned

- v0.2 Interceptor: browser extension for claude.ai paste.
- v0.3 Conduit: MCP server for the Claude desktop app.
- Live Claude Code A/B benchmark.
