# Changelog

**English** | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

All notable changes are listed here. Versions follow [Semantic Versioning](https://semver.org/). While the major version is 0, commands and config may still change.

## [0.2.0] - 2026-11-04 — squeeze-Auditor

v0.2 looks at the whole session, not one output at a time. Code in [`v0.2-Auditor/v0.2.0_squeeze-auditor/`](../v0.2-Auditor/v0.2.0_squeeze-auditor/). Includes everything in 0.1.0.

### Added

- Four session blockers (Claude Code):
  - **Repeated read**: a full `Read` of a file that was already read in this session and has not changed is denied. Allowed again when the file changes or the context is compacted.
  - **Budget pause** (`UserPromptSubmit`): when the session passes the budget (default 150K tokens), the message is held once with a suggestion to `/compact` or `/clear`. Sending it again goes through.
  - **Handoff** (`Stop` + `SessionStart`): when the session passes the handoff size (default 120K), Claude writes a handoff note to `.squeeze/handoff/`; after `/clear` the new session starts from the note. `remind` mode only suggests `/compact`.
  - **Fixed cost**: measures `CLAUDE.md` files and imports, skills, agents and MCP tool lists sent with every message. Turn MCP servers off per project with Claude Code's own switch, or automatically after N unused days (off by default).
- **squeeze-Auditor** app (`squeeze-auditor`, `squeeze auditor`): dashboard of real Claude Code usage from its session logs and what squeeze kept out, the current 5-hour window, saving modes (Strong, Balanced, Light, Custom), all settings, fixed-cost diagnosis with Measure and On/Off, handoff notes, connections. Local only (`127.0.0.1`, one-time key). English and Korean.
- **Claude desktop app** support: `squeeze mcp`, an MCP server with `read_log`, `tail_log`, `search_log`, `list_logs`, `save_handoff`, `load_handoff`, limited to folders you allow.
- `squeeze setup [--claude-code] [--desktop] [--remove]`: connect once for all projects (`~/.claude/settings.json`) and the desktop app, with backups in `~/.squeeze/backups/`.
- `squeeze doctor`: fixed cost per message in the terminal.
- Installers: Windows Setup (x64, ARM64, per-user, Start menu icon, uninstaller that disconnects), macOS `.pkg` with `squeeze-Auditor.app`, Linux `.deb` with a menu entry. Not code-signed yet.
- Global config `~/.squeeze/config.json` (a project's `.squeeze/config.json` still overrides it) and a global ledger `~/.squeeze/ledger.jsonl`.

### Changed

- The install scripts also install `squeeze-auditor`.
- The release title now comes from the CHANGELOG heading. CI also builds the installers and tests the frozen v0.1.0 snapshot.
- v0.1.0's frozen crate is named `squeeze-sentinel` (the program is still `squeeze`), so `cargo install --git … squeeze` builds v0.2.

### Planned

- v0.3: claude.ai in the browser and the mobile apps, with a new approach.

## [0.1.0] - 2026-11-03 — Sentinel

First release. Code in [`v0.1-Sentinel/v0.1.0_squeeze-sentinel/`](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/).

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
