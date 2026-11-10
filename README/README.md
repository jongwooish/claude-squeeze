# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#install) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

**English** | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **Feedback welcome.** Improvements, the v1.0 release, version updates, install errors or any other trouble: please [open an issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 languages OK).

**Spend your Claude tokens on thinking, not on log spam.**

squeeze keeps long command output, large and repeated file reads, oversized sessions and idle MCP servers out of Claude's context. Claude still sees every error and warning. **squeeze-Auditor**, the installed app, shows where your tokens went and lets you choose how hard squeeze works.

- [Effect](#effect) · [Versions](#versions) · [Where it works](#where-it-works) · [v0.2 squeeze-Auditor](#v02-squeeze-auditor) · [v0.1 Sentinel](#v01-sentinel) · [Install](#install) · [Usage](#usage) · [How it works](#how-it-works) · [Roadmap](#roadmap) · [FAQ](#faq)

## Why

Claude Code re-sends the whole conversation with every request, and command output is part of the conversation. One `npm install`, test run or `journalctl` can add thousands of tokens that ride along on every later turn, even though Claude only needed "it passed" or three error lines.

squeeze stops that output **before** it enters the context.

## Effect

Seven real-world logs replayed through squeeze ([method](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/docs/bench.md)):

| Log | Lines before → after | Tokens before → after (est.) | Saved | Distinct errors/warnings kept |
|---|---:|---:|---:|---:|
| `pytest` run, 2 failures | 217 → 39 | 4,040 → 516 | **87.2%** | 11 / 11 |
| `go test -v`, 1 failure | 488 → 16 | 4,247 → 202 | **95.2%** | 4 / 4 |
| `make` with gcc warnings + 1 error | 370 → 44 | 7,046 → 850 | **87.9%** | 7 / 7 |
| `tsc` with 2 type errors | 124 → 9 | 1,310 → 172 | **86.9%** | 3 / 3 |
| `dpkg -l` package list | 1049 → 63 | 35,237 → 2,106 | **94.0%** | 8 / 8 |
| Server journal, OOM kill (synthetic) | 1400 → 37 | 31,161 → 997 | **96.8%** | 5 / 5 |
| Short `npm` output | 16 → 16 | 287 → 287 | 0.0% (left alone) | 12 / 12 |
| **Total** | | **83,325 → 5,128** | **93.8%** | **50 / 50** |

**Why this is useful**

- **Nothing important was lost in these tests.** Every distinct error and warning survived. Repeats are shown once with a count (`… [same message 29 more times] …`).
- **Short output is never touched.** Under 80 lines and 8 KB, you get the original.
- **Savings repeat every turn.** A 31,000-token log that stays in context for the next 20 turns is about 620,000 input tokens. Squeezed to about 1,000 tokens, the same 20 turns cost about 20,000.
- **Longer sessions before limits.** Less noise means the context fills more slowly, so you hit compaction and plan limits later.
- **Claude focuses better.** The answer to "why did the build fail?" is in front of it, not buried in 1,000 lines.

**Honest limits:** token counts are estimates (bytes ÷ 4), and these numbers come from replaying saved logs, not from a live A/B run of Claude Code (planned). Run `squeeze report` in your project to see your real numbers.

**v0.2 adds four blockers** that stop the same tokens from being sent again and again: a repeated read of an unchanged file, a session that has grown past your budget, a long session that should start over from a short handoff note, and fixed costs such as idle MCP servers. Their effect depends on how you work, so the [v0.2 README](../v0.2-Auditor/v0.2.0_squeeze-auditor/README.md#effect) shows the arithmetic, and the squeeze-Auditor dashboard shows your own numbers.

## Versions

Each version has its own folder and README. Releases are tags on the whole repository.

| Version | Name | What it adds | Status |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/) | **Sentinel** | Claude Code hooks that catch noisy commands and whole-file reads, `squeeze run`, `compress`, `clip`, `report` | ✅ v0.1.0 |
| [v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/) | **squeeze-Auditor** | Four session blockers (repeated read, budget pause, handoff, fixed cost), the installed squeeze-Auditor app with a usage dashboard and saving modes, Claude desktop app support | ✅ v0.2.0 |
| v0.3 | *(name to come)* | claude.ai in the browser and the mobile apps, with a new approach to token complaints | 🛠 Planned |

### How the versions fit together

| Version | What you install | Needs the earlier version? |
|---|---|---|
| v0.1 Sentinel | The `squeeze` program | — |
| v0.2 squeeze-Auditor | The same `squeeze` program, now with an installer, a Start menu app (`squeeze-auditor`) and new hooks | **No, it includes v0.1.** Install it over v0.1, or on its own. |
| v0.3 | Planned | To be announced |

- **The program is cumulative.** A newer `squeeze` includes every earlier feature. You never install v0.1 and v0.2 side by side.
- **Folders describe what each version added.** You do not install a folder.
- **Your settings carry over.** `.squeeze/config.json` and hooks from v0.1's `squeeze init` keep working after an update. v0.2 adds a global `~/.squeeze/config.json` and connects to Claude Code once for all projects.

| You use… | Install |
|---|---|
| Claude Code | squeeze-Auditor (v0.2), then **Connect Claude Code** |
| Claude desktop app | squeeze-Auditor (v0.2), then **Connect desktop app** |
| claude.ai in a browser, or the mobile apps | `squeeze clip` for long pastes now; v0.3 when it ships |

## Where it works

| Where | Effect | How |
|---|---|---|
| **Claude Code** | Automatic, all projects | squeeze-Auditor (or `squeeze setup`) adds hooks to `~/.claude/settings.json`. All four v0.2 blockers and both v0.1 guards run. |
| **Claude desktop app** (Windows, macOS) | When Claude reads files or saves a handoff | Connecting adds the `squeeze mcp` tools: Claude reads your logs already squeezed, only from folders you allow, and can save and load handoff notes. squeeze cannot see or change the chats themselves. |
| **claude.ai (web, mobile)** | Manual, when you paste long text | Copy a log, run `squeeze clip`, paste. Planned for v0.3. |
| **Any other terminal tool** | Manual | `<command> 2>&1 \| squeeze compress` |

On the Pro plan, Claude Code and claude.ai share one usage limit, so tokens saved in Claude Code are left for your chats too.

## v0.2 squeeze-Auditor

v0.1 looked at one output at a time. **v0.2 looks at the whole session**: what Claude already read, how big the conversation has grown, and what rides along with every message. It blocks four kinds of waste:

| Blocker | What it catches | How it blocks |
|---|---|---|
| **Repeated read** | A full re-read of a file that has not changed | Denies the `Read`: "you already have this". Allowed again when the file changes or after compaction. |
| **Budget pause** | Sending more messages into a huge session (default 150K tokens) | Holds your message once and suggests `/compact` or `/clear`. Send it again and it goes through. |
| **Handoff** | A long session that re-reads its whole history (default 120K) | Claude writes a short handoff note; after `/clear` the fresh session starts from the note instead of the whole conversation. |
| **Fixed cost** | `CLAUDE.md` files, skills, agents and MCP tool lists sent with every message | Measures each one; turn idle MCP servers off with one click (Claude Code's own switch), or automatically after 14 unused days. |

**squeeze-Auditor** is the app you open from the Start menu or Applications: a dashboard of real Claude Code usage next to what squeeze kept out, saving modes (Strong, Balanced, Light), every setting, the fixed-cost diagnosis, your handoff notes, and the connections to Claude Code and the desktop app.

![squeeze-Auditor dashboard](../v0.2-Auditor/v0.2.0_squeeze-auditor/docs/images/auditor-en-dashboard.png)

Full v0.2 documentation: [v0.2-Auditor/v0.2.0_squeeze-auditor/README.md](../v0.2-Auditor/v0.2.0_squeeze-auditor/README.md).

## v0.1 Sentinel

Sentinel stands guard between Claude Code and your shell. Single binary, Rust, **zero dependencies**, macOS / Linux / Windows.

- **Bash guard**: noisy commands (`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log` and more) are redirected to `squeeze run -- <command>`. Commands that are already bounded (`| tail -20`, `-n 50`, `> file`, `--quiet`) are left alone.
- **Read guard**: reading a file over 400 lines without `offset`/`limit` is redirected. Claude gets an outline with line numbers and picks a range.
- **Smart compression**: first 8 and last 20 lines, every error and warning with 2 lines of context, stack traces; repeats collapsed with a count; errors chosen before warnings.
- **Exit codes preserved**: `squeeze run` exits with the command's own code.
- **Full logs on disk**: `.squeeze/raw/`, newest 50 kept, readable only by you. `.squeeze/` is added to `.gitignore`.
- **`squeeze clip`** for claude.ai and **`squeeze report`** for your real savings.
- **Safe by design**: hooks never approve a tool call, fail open on any error, and `squeeze uninstall` removes everything squeeze added. No network code.

What Claude sees after `squeeze run -- pytest -v` (real output, 217 lines → 39):

```
============================= test session starts ==============================
platform linux -- Python 3.13.18, pytest-9.1.1, pluggy-1.6.0 -- /usr/local/bin/python3
cachedir: .pytest_cache
rootdir: ~/project/
plugins: platformdirs-4.13.3, anyio-4.15.1
collecting ... collected 183 items

test_inventory.py::test_restock[0] PASSED                                [  0%]
… [178 lines omitted] …
test_inventory.py::test_restock[179] PASSED                              [ 98%]
test_inventory.py::test_negative PASSED                                  [ 98%]
test_inventory.py::test_bulk_order FAILED                                [ 99%]
test_inventory.py::test_missing_sku FAILED                               [100%]

=================================== FAILURES ===================================
… (both tracebacks kept in full) …
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

Full v0.1 documentation: [v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.md](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.md).

## Install

**Installer (recommended).** From the [latest release](https://github.com/jongwooish/claude-squeeze/releases/latest):

| System | File |
|---|---|
| Windows 10/11 | `squeeze-Auditor-0.2.0-setup-x64.exe` (ARM: `-setup-arm64.exe`) |
| macOS | `squeeze-Auditor-0.2.0-macos-arm64.pkg` (Intel: `-macos-x86_64.pkg`) |
| Linux (Debian, Ubuntu) | `squeeze-auditor_0.2.0_amd64.deb` (ARM: `_arm64.deb`) |

The installers are not code-signed yet, so Windows and macOS may block them. SmartScreen, Smart App Control, Gatekeeper and Linux permissions: [If your system blocks it](../v0.2-Auditor/v0.2.0_squeeze-auditor/README.md#if-your-system-blocks-it).

**Terminal:**

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# From source (any OS with Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

Every release file is listed in `checksums.txt` (SHA-256), and the install scripts check it. `SQUEEZE_VERSION=v0.1.0` pins a version; `SQUEEZE_INSTALL_DIR` changes the folder.

## Quick start

1. Open **squeeze-Auditor** and click **Connect Claude Code** (all projects). Using the desktop app? Connect it in **Connections** and restart it.
2. Use Claude Code as usual.
3. Come back to the dashboard to see what was kept out. Change how hard squeeze works in **Saving modes** or **Settings**.

From a terminal:

```bash
squeeze setup      # connect Claude Code (and the desktop app, if installed)
claude             # use Claude Code as usual
squeeze auditor    # open the window
squeeze setup --remove
```

Connecting changes only `~/.claude/settings.json` (and `claude_desktop_config.json` for the desktop app). Other settings and hooks are kept, and the original is backed up in `~/.squeeze/backups/` first. v0.1's per-project `squeeze init` still works.

## Usage

| Command | What it does |
|---|---|
| `squeeze auditor` | Open the squeeze-Auditor window |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Connect or disconnect Claude Code (all projects) and the desktop app |
| `squeeze doctor [--dir D] [--desktop] [--json]` | What is sent with every message (fixed cost) |
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Install hooks into one project only (v0.1 style) |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Remove a project's hooks and note; `--purge` also deletes `.squeeze/` |
| `squeeze run -- <command>` | Run a command; short summary when output is long |
| `<command> 2>&1 \| squeeze compress` | Compress piped output |
| `squeeze clip [--print]` | Compress the clipboard (for claude.ai) |
| `squeeze report [--since 7d] [--global] [--json]` | Show savings |
| `squeeze bench <dir> [--markdown]` | Replay `*.log` files and measure |
| `squeeze mcp` | The MCP server the desktop app runs |
| `squeeze version` | Print the version |

Prefix a command with `SQUEEZE_SKIP=1` to get its raw output once, or put `SQUEEZE_SKIP` in a message to pass the budget pause. Settings: [v0.2 configuration](../v0.2-Auditor/v0.2.0_squeeze-auditor/README.md#configuration).

## How it works

```
Claude wants to run:  npm test
        │
        ▼
PreToolUse hook ── squeeze hook pre-bash
        │  noisy? ── no ──▶ silent; normal permission flow
        │  yes
        ▼
deny + hint: "run it as  squeeze run -- 'npm test'"
        │
        ▼
Claude runs: squeeze run -- 'npm test'   (your permission rules apply as usual)
        │
        ├─▶ full output ── .squeeze/raw/<time>.log
        └─▶ short output ─ Claude  (errors, warnings, head, tail, log path)
```

Why "deny with a hint" instead of rewriting the command silently? A hook that rewrites a command must also approve it, which would skip your permission prompts. squeeze never approves anything for you.

### How squeeze differs from wrappers

Tools like [chaude](https://github.com/int-ttt/chaude) wrap the Claude Code CLI in their own screen and save tokens by rotating sessions and handing work to cheaper models. squeeze works on the other side of the problem:

- **No wrapper.** You keep using `claude` as it is, through Claude Code's own hooks.
- **It stops waste at the door** instead of cleaning up a full context.
- **The two can be used together.**

## Roadmap

- [x] **v0.1 Sentinel**: Claude Code hooks, `run`, `compress`, `clip`, `report`, `bench`
- [x] **v0.2 squeeze-Auditor**: repeated-read, budget, handoff and fixed-cost blockers; installed app with dashboard and saving modes; Claude desktop app tools
- [ ] **v0.3**: claude.ai in the browser and the mobile apps, with a new approach to the token problems people report most
- [ ] Code-signed installers
- [ ] Live Claude Code A/B benchmark (same tasks, squeeze on and off)
- [ ] v1.0: stable commands and config

## FAQ

**Q. Can Claude miss something important?**
A. In every fixture, all distinct error and warning messages were kept. If a summary is not enough, the footer gives the path of the full log. Blocked re-reads tell Claude how to read the file again if it really needs to.

**Q. Does squeeze send my data anywhere?**
A. No. It has no network code; the squeeze-Auditor window is a page on `127.0.0.1` that only your computer can open.

**Q. Can it break my Claude Code session?**
A. The hooks fail open: bad input, a missing file or a crash means no output, and Claude Code continues normally. squeeze never approves a tool call for you.

**Q. Does it save tokens in regular claude.ai chats?**
A. Only when you paste long text after running `squeeze clip`. The browser and the mobile apps are the focus of v0.3.

**Q. Is the dashboard my plan's usage limit?**
A. No. It reads Claude Code's own logs on this computer. There is no official way for other programs to read plan usage.

**Q. Why version 0.x?**
A. While the major version is 0, command names and config may still change. v1.0 will mean they are stable.

## Repository layout

```
claude-squeeze/
├── README/                      ← overview in 6 languages (README.md = English)
├── CHANGELOG/                   ← CHANGELOG in 6 languages
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   version overview
│   └── v0.1.0_squeeze-sentinel/   ← v0.1.0 as released (frozen)
├── v0.2-Auditor/                ← v0.2 squeeze-Auditor
│   ├── README.md                   version overview
│   └── v0.2.0_squeeze-auditor/    ← v0.2: the current squeeze program (Rust, no dependencies)
│       ├── README.md               full v0.2 docs (English)
│       ├── README/                 translations (5 languages)
│       ├── src/                    engine, hooks, blockers, squeeze-Auditor server
│       ├── ui/                     the squeeze-Auditor window (one HTML file)
│       ├── packaging/              Windows, macOS and Linux installers, icon
│       ├── docs/                   benchmark, user guide (Korean), screenshots
│       └── testdata/fixtures/      logs used by tests and the benchmark
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock (Rust workspace)
├── .github/                     ← issue forms, CI, release, front-page README
├── CLAUDE.md
└── LICENSE
```

## Contributing

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
```

New fixtures are welcome: add a real `*.log` (secrets and personal paths removed) to `v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures/`. Tests fail if any distinct error or warning is lost. Bugs and ideas: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).

## Community

**Found a problem or have an idea? Please open an [issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose).** Improvements, the v1.0 release, version updates, installer errors and any other software trouble: every report helps shape the next version.


| I want to… | Go to |
|---|---|
| Report a bug or a broken session | [Issues → Bug report](https://github.com/jongwooish/claude-squeeze/issues/new?template=bug_report.yml) |
| Report a dropped line, too little saving, or a command that was (not) caught | [Issues → Compression problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=compression.yml) |
| Report an install, update or version problem | [Issues → Install, update or version problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=version_problem.yml) |
| Suggest a feature | [Issues → Feature idea](https://github.com/jongwooish/claude-squeeze/issues/new?template=feature_request.yml) |
| Ask a question or get help troubleshooting | [Discussions → Q&A](https://github.com/jongwooish/claude-squeeze/discussions/categories/q-a) |
| Talk about the roadmap and what v1.0 should include | [Discussions → Ideas](https://github.com/jongwooish/claude-squeeze/discussions/categories/ideas) |
| Hear about new versions | [Discussions → Announcements](https://github.com/jongwooish/claude-squeeze/discussions/categories/announcements) and [Releases](https://github.com/jongwooish/claude-squeeze/releases) (click **Watch → Custom → Releases** to get notified) |

Any of the six languages is fine. Security problems: see [SECURITY.md](../.github/SECURITY.md). How to contribute: [CONTRIBUTING.md](../.github/CONTRIBUTING.md).

## License

MIT. See [LICENSE](../LICENSE). squeeze is an unofficial community tool and is not affiliated with Anthropic.
