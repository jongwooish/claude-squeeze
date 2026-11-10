# v0.1.0 Sentinel
<!-- Copy of v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.md with links adjusted. Edit that file instead. -->

**English** | [한국어](v0.1.0_squeeze-sentinel/README/README.ko.md) | [日本語](v0.1.0_squeeze-sentinel/README/README.ja.md) | [简体中文](v0.1.0_squeeze-sentinel/README/README.zh-CN.md) | [Español](v0.1.0_squeeze-sentinel/README/README.es.md) | [Français](v0.1.0_squeeze-sentinel/README/README.fr.md)

> 💬 **Feedback welcome.** Improvements, the v1.0 release, version updates, install errors or any other trouble: please [open an issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 languages OK).

← [Back to squeeze](../README/README.md)

Sentinel stands guard between Claude Code and your shell. When Claude is about to run a command that floods the context (installs, builds, tests, logs) or read a huge file in one go, Sentinel redirects it to a short version. Claude still gets every error and warning.

Single binary, written in Rust, **zero dependencies**. Works on macOS, Linux and Windows.

**Requires:** nothing. This is the base `squeeze` program. Later versions are the same program with more features: [v0.2 squeeze-Auditor](../v0.2-Auditor/v0.2.0_squeeze-auditor/README.md) adds four session blockers and an installed control panel for Claude Code and the Claude desktop app; v0.3 (planned) covers the browser and the mobile apps. See [how the versions fit together](../README/README.md#how-the-versions-fit-together).

## Effect

Fixture replay, 7 logs, 83,325 → 5,128 estimated tokens (**−93.8%**), every distinct error and warning kept. Details and method: [docs/bench.md](v0.1.0_squeeze-sentinel/docs/bench.md).

What Claude sees after `squeeze run -- pytest -v` (217 lines in, 39 out). This is the real output for the `pytest-fail.log` fixture:

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
_______________________________ test_bulk_order ________________________________

    def test_bulk_order():
        orders = {"apple": 3, "pear": 5}
>       assert sum(orders.values()) == 9, "bulk order total mismatch"
E       AssertionError: bulk order total mismatch
E       assert 8 == 9
E        +  where 8 = sum(dict_values([3, 5]))
E        +    where dict_values([3, 5]) = <built-in method values of dict object at 0x7f4c5d1f9900>()
E        +      where <built-in method values of dict object at 0x7f4c5d1f9900> = {'apple': 3, 'pear': 5}.values

test_inventory.py:12: AssertionError
_______________________________ test_missing_sku _______________________________

    def test_missing_sku():
        stock = {"A1": 3}
>       assert stock["B7"] == 0
               ^^^^^^^^^^^
E       KeyError: 'B7'

test_inventory.py:15: KeyError
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

## Features

- **Bash guard**: noisy commands (`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log` and more) are redirected to `squeeze run -- <command>`.
- **Read guard**: reading a file over 400 lines without `offset`/`limit` is redirected. Claude gets an outline with line numbers and picks a range.
- **Smart compression**: keeps the first 8 and last 20 lines, every error and warning with context, and stack traces. Repeated messages are collapsed with a count. Errors are kept before warnings.
- **Exit codes preserved**: `squeeze run` exits with the command's own exit code.
- **Full logs on disk**: `.squeeze/raw/`, newest 50 kept, readable only by you (0600). `.squeeze/` is added to `.gitignore`.
- **`squeeze clip`** for claude.ai: compress the clipboard before pasting a log into a chat.
- **`squeeze report`**: what would have entered the context, what did, and the biggest savings.
- **Safe by design**: hooks never approve a tool call; any error makes them stay silent (fail open); `squeeze uninstall` removes everything it added.

## Install

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# From source
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

The installers check the release `checksums.txt` before installing. `SQUEEZE_VERSION=v0.1.0` pins a version; `SQUEEZE_INSTALL_DIR` changes the folder.

## Quick start

```bash
cd ~/my-project
squeeze init
claude
```

`squeeze init` changes four things, and `squeeze uninstall` undoes them:

| File | Change |
|---|---|
| `.claude/settings.json` | Adds two `PreToolUse` hooks (`Bash`, `Read`). Your other settings and hooks are kept; a `.squeeze.bak` backup is written first. |
| `.squeeze/config.json` | Default config (see below). |
| `.gitignore` | Adds `.squeeze/`, because raw logs can contain secrets. |
| `CLAUDE.md` | Adds a 4-line note between `<!-- squeeze:start -->` markers. |

## Usage

| Command | What it does |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Install into a project. Use `--bin` with an absolute path if `squeeze` is not on `PATH`. |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Remove hooks and the CLAUDE.md note. `--purge` also deletes `.squeeze/`. |
| `squeeze run -- <command>` | Run a command; print a short summary when the output is long. |
| `<command> 2>&1 \| squeeze compress` | Compress piped output. |
| `squeeze clip [--print]` | Compress the clipboard in place (`--print` shows it instead). |
| `squeeze report [--since 7d] [--global] [--json] [--top N]` | Show savings for this project. `--global` shows `squeeze clip` savings. |
| `squeeze bench <dir> [--markdown]` | Replay `*.log` files and measure savings. |
| `squeeze hook pre-bash` / `pre-read` | Hook entry points called by Claude Code. |
| `squeeze version` | Print the version. |

To get raw output for one command anyway, Claude (or you) can prefix it with `SQUEEZE_SKIP=1`.

## Configuration

`.squeeze/config.json` (all keys optional):

| Key | Default | Meaning |
|---|---|---|
| `compress.max_lines` / `max_bytes` | `80` / `8000` | Output at or below both limits is left alone. |
| `compress.head_lines` / `tail_lines` | `8` / `20` | Lines always kept from the start / end. |
| `compress.context` | `2` | Lines kept around each error or warning. |
| `compress.budget` | `120` | Rough cap on kept lines. |
| `guard_bash` / `guard_read` | `true` / `true` | Turn each guard on or off. |
| `read_max_lines` | `400` | Files above this are redirected to a ranged read. |
| `extra_noisy` | `[]` | Command prefixes to treat as noisy, e.g. `"./deploy.sh"`. |
| `quiet` | `[]` | Command prefixes the guard never touches, e.g. `"make docs"`. |
| `keep_raw` | `50` | Raw logs kept in `.squeeze/raw/`. |

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
        ▼
full output ──▶ .squeeze/raw/<time>.log
short output ──▶ Claude  (errors, warnings, head, tail, footer with log path)
```

Why "deny with a hint" instead of rewriting the command silently? A hook that rewrites a command also has to approve it, which would skip your permission prompts. squeeze never does that.

More: [docs/guide/how-it-works.md](v0.1.0_squeeze-sentinel/docs/guide/how-it-works.md) (Korean).

## Where it applies

- **Claude Code**: automatic, through hooks.
- **claude.ai**: only when you paste long text, with `squeeze clip`. Normal chat is not affected; squeeze cannot see or change your chats.
- **Other terminal tools**: pipe into `squeeze compress`.

## FAQ

**Q. Can Claude miss something important?**
A. In every fixture, all distinct error and warning messages were kept. If a summary is not enough, the footer gives the path of the full log, and Claude can read it with a ranged read.

**Q. Does squeeze send my data anywhere?**
A. No. It has no network code. Logs stay in `.squeeze/` on your machine.

**Q. Can it break my session?**
A. The hooks fail open: bad input, a missing file or any panic means no output, and Claude Code continues normally.

**Q. Claude keeps reading a big file and I want the whole thing.**
A. The hint tells Claude to call Read again with `limit=<lines>`, which passes. Or set `"guard_read": false`.

**Q. Does it save tokens in regular claude.ai chats?**
A. Only when you paste long text after running `squeeze clip`.

## Feedback

Please leave improvements and problems in [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose): ideas for v1.0, version update or installer errors, compression that dropped something, or anything else that went wrong. Pick a form (bug report, compression problem, install/update/version problem, feature idea). Questions and roadmap talk go to [Discussions](https://github.com/jongwooish/claude-squeeze/discussions). Any of the six languages is fine.

## Contributing

```bash
cargo test --manifest-path CARGO/Cargo.toml                       # unit tests, including fixture checks
cargo clippy --manifest-path CARGO/Cargo.toml -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml -- bench v0.1-Sentinel/v0.1.0_squeeze-sentinel/testdata/fixtures
```

New fixtures are welcome: add a real `*.log` (remove secrets and personal paths first) to `testdata/fixtures/`. The tests fail if any distinct error or warning is lost.

Issues and ideas: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).
