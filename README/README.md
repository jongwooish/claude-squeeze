# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#install) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

**English** | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **Feedback welcome.** Improvements, the v1.0 release, version updates, install errors or any other trouble: please [open an issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 languages OK).

**Spend your Claude tokens on thinking, not on log spam.**

squeeze keeps long command output and large file reads out of Claude's context. Claude still sees every error and warning, plus the first and last lines. The full log is saved on your disk in case it is needed.

- [Effect](#effect) · [Versions](#versions) · [Where it works](#where-it-works) · [v0.1 Sentinel](#v01-sentinel) · [Install](#install) · [Usage](#usage) · [How it works](#how-it-works) · [Roadmap](#roadmap) · [FAQ](#faq)

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

## Versions

Each version has its own folder and README. Releases are tags on the whole repository.

| Version | Name | What it adds | Status |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/) | **Sentinel** | Claude Code hooks that catch noisy commands and whole-file reads, `squeeze run`, `compress`, `clip`, `report` | ✅ v0.1.0 |
| v0.2 | **Interceptor** | Browser extension that offers to squeeze long text when you paste it into claude.ai | 🛠 Planned |
| v0.3 | **Conduit** | MCP server for the Claude desktop app: Claude reads your logs already squeezed | 🛠 Planned |

### How the versions fit together

There are two things you can install: the **`squeeze` program** (CLI) and, from v0.2, a **browser extension**.

| Version | What you install | Needs v0.1? |
|---|---|---|
| v0.1 Sentinel | The `squeeze` program | — |
| v0.2 Interceptor | A browser extension | **No.** It carries its own copy of the compressor and works alone. Install v0.1 too if you use Claude Code. |
| v0.3 Conduit | Nothing new: a command inside the `squeeze` program (`squeeze mcp`) | **Yes.** Update `squeeze` to v0.3 or later; there is no separate download. |

- **The program is cumulative.** A newer `squeeze` includes every earlier feature. You never install v0.1 and v0.3 side by side.
- **Folders describe what each version added.** You do not install a folder.
- **Your settings carry over.** `.squeeze/config.json` and the hooks from `squeeze init` keep working after an update. If a version changes them, its CHANGELOG entry says so and `squeeze init` updates them.

| You use… | Install |
|---|---|
| Claude Code only | `squeeze` (v0.1 now) |
| claude.ai only | `squeeze clip` now, the v0.2 extension when it ships |
| Claude desktop app, and you want Claude to read logs itself | `squeeze` v0.3 when it ships |
| All of the above | `squeeze` (latest) + the v0.2 extension |

## Where it works

| Where | Effect | How |
|---|---|---|
| **Claude Code** | Automatic | `squeeze init` installs hooks. Noisy commands and large file reads are redirected. |
| **claude.ai (web, desktop, mobile)** | Manual, when you paste long text | Copy a log, run `squeeze clip`, paste. Normal chat messages are not changed; squeeze cannot see your chats. |
| **Any other terminal tool** | Manual | `<command> 2>&1 \| squeeze compress` |

On the Pro plan, Claude Code and claude.ai share one usage limit, so tokens saved in Claude Code are left for your chats too.

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

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# From source (any OS with Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

The installers verify the release `checksums.txt` (SHA-256). `SQUEEZE_VERSION=v0.1.0` pins a version; `SQUEEZE_INSTALL_DIR` changes the folder.

## Quick start

```bash
cd ~/my-project
squeeze init      # hooks, config, .gitignore entry, CLAUDE.md note
claude            # use Claude Code as usual
squeeze report    # see what was kept out of context
```

`squeeze init` changes four files, and `squeeze uninstall` undoes it:

| File | Change |
|---|---|
| `.claude/settings.json` | Adds two `PreToolUse` hooks (`Bash`, `Read`). Other settings and hooks are kept; a backup is written first. |
| `.squeeze/config.json` | Default config. |
| `.gitignore` | Adds `.squeeze/` (raw logs can contain secrets). |
| `CLAUDE.md` | Adds a 4-line note between `<!-- squeeze:start -->` markers. |

## Usage

| Command | What it does |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Install into a project |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Remove hooks and note; `--purge` also deletes `.squeeze/` |
| `squeeze run -- <command>` | Run a command; short summary when output is long |
| `<command> 2>&1 \| squeeze compress` | Compress piped output |
| `squeeze clip [--print]` | Compress the clipboard (for claude.ai) |
| `squeeze report [--since 7d] [--global] [--json]` | Show savings |
| `squeeze bench <dir> [--markdown]` | Replay `*.log` files and measure |
| `squeeze version` | Print the version |

Prefix a command with `SQUEEZE_SKIP=1` to get its raw output once. Settings live in `.squeeze/config.json` ([all keys](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.md#configuration)).

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
- [ ] **v0.2 Interceptor**: browser extension for claude.ai paste, same engine compiled to WebAssembly
- [ ] **v0.3 Conduit**: `squeeze mcp`, read-only MCP server for the Claude desktop app
- [ ] Live Claude Code A/B benchmark (same tasks, squeeze on and off)
- [ ] v1.0: stable commands and config

## FAQ

**Q. Can Claude miss something important?**
A. In every fixture, all distinct error and warning messages were kept. If a summary is not enough, the footer gives the path of the full log and Claude can read it with a ranged read.

**Q. Does squeeze send my data anywhere?**
A. No. It has no network code. Logs stay in `.squeeze/` on your machine.

**Q. Can it break my Claude Code session?**
A. The hooks fail open: bad input, a missing file or a crash means no output, and Claude Code continues normally.

**Q. Does it save tokens in regular claude.ai chats?**
A. Only when you paste long text after running `squeeze clip`. v0.2 will make that automatic in the browser.

**Q. Why version 0.x?**
A. While the major version is 0, command names and config may still change. v1.0 will mean they are stable.

## Repository layout

```
claude-squeeze/
├── README/                      ← overview in 6 languages (README.md = English)
├── CHANGELOG/                   ← CHANGELOG in 6 languages
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   version overview
│   └── v0.1.0_squeeze-sentinel/   ← v0.1: CLI + Claude Code hooks (Rust, no dependencies)
│       ├── README.md               full v0.1 docs (English)
│       ├── README/                 translations (5 languages)
│       ├── src/
│       ├── docs/                   benchmark method, user guide (Korean)
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
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.1-Sentinel/v0.1.0_squeeze-sentinel/testdata/fixtures
```

New fixtures are welcome: add a real `*.log` (secrets and personal paths removed) to `v0.1-Sentinel/v0.1.0_squeeze-sentinel/testdata/fixtures/`. Tests fail if any distinct error or warning is lost. Bugs and ideas: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).

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
