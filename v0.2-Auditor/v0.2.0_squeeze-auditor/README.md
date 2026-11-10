# v0.2.0 squeeze-Auditor

**English** | [한국어](README/README.ko.md) | [日本語](README/README.ja.md) | [简体中文](README/README.zh-CN.md) | [Español](README/README.es.md) | [Français](README/README.fr.md)

> 💬 **Feedback welcome.** Improvements, the v1.0 release, version updates, install errors or any other trouble: please [open an issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 languages OK).

← [Back to squeeze](../../README/README.md)

v0.1 Sentinel stopped one big output at a time. **v0.2 Auditor looks at the whole session.** It remembers what Claude has already read, watches how big the conversation has grown, and checks what is sent with every message before you type a word. Then it blocks four kinds of waste that a size check alone cannot see.

It comes as an installed program: run the Setup file, open **squeeze-Auditor** from the Start menu (or Applications), and see where your tokens went, what squeeze kept out, and change how hard it works.

**Works in:** Claude Code (all projects) and the Claude desktop app (Windows and macOS). claude.ai in the browser and the mobile apps come in v0.3.
**Requires:** nothing. v0.2 is the same `squeeze` program as v0.1 with everything v0.1 did, plus the features below. See [how the versions fit together](../../README/README.md#how-the-versions-fit-together).

![squeeze-Auditor dashboard](docs/images/auditor-en-dashboard.png)

## What's new: four blockers

| | Blocker | What it catches | How it blocks |
|---|---|---|---|
| 1 | **Repeated read** | Claude reads the same file in full again, though it has not changed | The `Read` is denied with "you already read this, use what you have". Allowed again as soon as the file changes or the conversation is compacted. |
| 2 | **Budget pause** | You keep sending messages into a session that has grown huge | Before your message is sent, squeeze holds it once and suggests `/compact` or `/clear`. Send it again (↑, Enter) and it goes through. It asks again only after the session has grown another 50%. |
| 3 | **Handoff** | A long session that keeps re-reading its whole history | When the session passes the handoff size, Claude writes a short handoff note before it stops. You run `/clear`; the fresh session starts from the note (about 1,000 tokens) instead of the whole conversation. |
| 4 | **Fixed cost** | Things sent with *every* message: `CLAUDE.md` files and their imports, skill and agent descriptions, MCP servers' tool lists | squeeze-Auditor measures each one. Turn idle MCP servers off with one click, using Claude Code's own on/off switch (the same one as `/mcp`). Optional: turn off servers a project has not used for 14 days automatically. |

### How this differs from v0.1

| | v0.1 Sentinel | v0.2 Auditor |
|---|---|---|
| Looks at | the size of **one** output or file | the **session**: what was read before, how big it is now, what rides along every time |
| Decides by | "is this output long?" | memory (same file, unchanged), totals (session size), history (servers never used) |
| Hooks | `PreToolUse` (Bash, Read) | also `UserPromptSubmit`, `Stop`, `SessionStart`, `PreCompact` |
| Installed with | `squeeze init` per project | Setup installer or `squeeze setup`, once for **all** projects |
| You see | `squeeze report` in a terminal | the squeeze-Auditor window |

Everything from v0.1 still runs: noisy commands go through `squeeze run`, big files get an outline.

## The squeeze-Auditor window

| Tab | What you can do |
|---|---|
| **Dashboard** | Real Claude Code usage from its own session logs (today, 7 or 30 days), next to what squeeze kept out. The current 5-hour window and burn rate. What was blocked, by kind. Recent sessions with their size against your budget. |
| **Saving modes** | Strong, Balanced (default) or Light, with the numbers each one sets. Or Custom. |
| **Settings** | Every value: when output is squeezed, how many lines are kept, the big-file size, the budget, the handoff size and style, the fixed-cost rules, commands to never touch. |
| **Fixed cost** | Per project (or the desktop app): each memory file, import, skill, agent and MCP server with its tokens per message. Measure an MCP server, turn it on or off. Tips for making it smaller. |
| **Handoffs** | Every handoff note, readable in place. |
| **Connections** | Connect or disconnect Claude Code and the Claude desktop app. Choose which folders the desktop app may read. |

The window is a small page served by squeeze on `127.0.0.1` only, with a one-time key per launch, shown in an Edge or Chrome app window (or your default browser). It closes itself shortly after you close the window. Nothing leaves your computer. English and Korean.

![Fixed cost tab](docs/images/auditor-en-fixed.png)

## Effect

**Output compression** (from v0.1, unchanged): seven real-world logs, 83,325 → 5,128 estimated tokens (**−93.8%**), every distinct error and warning kept. Method: [docs/bench.md](docs/bench.md).

**The four blockers** save in a different way: they stop the same tokens from being sent over and over. What they are worth depends on how you work, so here is the arithmetic, not a benchmark:

| Blocker | Example | Kept out |
|---|---|---|
| Repeated read | A 600-line file (about 7,000 tokens) read again, unchanged | about 7,000 tokens, plus that much again on every later message of the session |
| Budget pause / handoff | Continuing a 150,000-token session for 20 more messages, versus a fresh session started from a 1,000-token note | about 3,000,000 input tokens re-read, versus about 20,000 plus the new work |
| Fixed cost | An idle MCP server with 12 tools (1,882 tokens, as measured for a test server) over 100 messages | about 188,000 tokens |

**Honest limits.**
- Token counts are estimates (bytes ÷ 4). Most of a long session is re-read from Claude's prompt cache, which costs less than fresh input, so the cost saved is smaller than the token count suggests. It still fills the context window and counts toward usage.
- The dashboard's usage comes from Claude Code's local session logs. It does not include claude.ai or the desktop app's own chats, and it is not your plan's official limit. The 5-hour window is an estimate of how Claude Code counts it.
- squeeze cannot see the desktop app's conversations. There it saves only when Claude reads logs through the squeeze tools, or when you ask it to save a handoff.

## Install

Download from the [latest release](https://github.com/jongwooish/claude-squeeze/releases/latest):

| System | File | Then |
|---|---|---|
| Windows 10/11 (most PCs) | `squeeze-Auditor-0.2.0-setup-x64.exe` | Run it. No admin rights needed. Start menu → **squeeze-Auditor** |
| Windows on ARM | `squeeze-Auditor-0.2.0-setup-arm64.exe` | Same |
| macOS, Apple silicon | `squeeze-Auditor-0.2.0-macos-arm64.pkg` | Open it. Applications → **squeeze-Auditor** |
| macOS, Intel | `squeeze-Auditor-0.2.0-macos-x86_64.pkg` | Same |
| Linux (Debian, Ubuntu) | `squeeze-auditor_0.2.0_amd64.deb` / `_arm64.deb` | `sudo apt install ./squeeze-auditor_0.2.0_amd64.deb`, then your app menu |

**The installers are not code-signed yet**, so your system will warn you once:
- **Windows** SmartScreen: "Windows protected your PC" → **More info** → **Run anyway**.
- **macOS**: "cannot be opened because it is from an unidentified developer" → right-click the file → **Open**, or System Settings → Privacy & Security → **Open Anyway**.

Every file is listed in `checksums.txt` (SHA-256) on the release page.

Terminal install (both programs, no Start menu icon):

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# From source (Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

## Quick start

1. Install, then open **squeeze-Auditor**. (On Windows, the installer can connect Claude Code for you: leave the box ticked.)
2. Click **Connect Claude Code**. This adds squeeze's hooks to `~/.claude/settings.json` for all projects. Your other settings and hooks are kept, and a backup is written first.
3. Using the Claude desktop app? Go to **Connections**, click **Connect**, add the folders whose logs Claude may read, and restart the desktop app.
4. Use Claude Code as usual. Come back to the Dashboard to see what was kept out.

From a terminal instead: `squeeze setup` (connect), `squeeze setup --remove` (disconnect), `squeeze auditor` (open the window).

**Upgrading from v0.1:** install over it (the Windows installer uses the same folder as v0.1's `install.ps1`) and connect once. Projects where you ran `squeeze init` keep working; you can leave their hooks or remove them with `squeeze uninstall`. On Windows, uninstalling squeeze-Auditor also disconnects it from Claude Code and the desktop app; on macOS and Linux run `squeeze setup --remove` first.

## What happens in a session

```
SessionStart ──▶ a recent handoff note exists?  ──yes──▶ load it into the new session
     │
You type a message
     │
UserPromptSubmit ──▶ session ≥ budget (150K)?    ──yes──▶ hold it once: "/compact or /clear, or send again"
     │
Claude works
     ├─ Bash ──▶ noisy command?                   ──yes──▶ "run it as squeeze run -- …"        (v0.1)
     └─ Read ──▶ whole read of a big file?        ──yes──▶ outline + ranged read               (v0.1)
                 same file, unchanged, read in full before? ──yes──▶ "use what you read"       (v0.2)
     │
Stop ──▶ session ≥ handoff size (120K)?          ──yes──▶ Claude writes .squeeze/handoff/<time>.md
     │                                                    and tells you to run /clear
PreCompact ──▶ forget which files were read (Claude no longer has them)
```

All hooks fail open: any error means no output, and Claude Code carries on. squeeze never approves a tool call for you.

## Claude desktop app

Connecting adds one entry to `claude_desktop_config.json`: an MCP server, `squeeze mcp`, with six tools.

| Tool | What it does |
|---|---|
| `read_log` | Read a log or text file already squeezed (or an exact line range) |
| `tail_log` | The last lines of a log, squeezed |
| `search_log` | Matching lines with a little context |
| `list_logs` | Files in an allowed folder |
| `save_handoff` / `load_handoff` | Save the current chat as a handoff note and start a new chat from it |

Claude can read only the folders you list in **Connections**. So instead of pasting a 5,000-line log into the chat, ask "read the last error in my app log", and Claude gets the short version. When a chat gets long, ask Claude to save a handoff, start a new chat, and ask it to load the handoff.

## Saving modes

| | Strong | Balanced (default) | Light |
|---|---:|---:|---:|
| Output left alone up to | 40 lines | 80 lines | 150 lines |
| Big file means more than | 250 lines | 400 lines | 800 lines |
| Budget pause at | 100K tokens | 150K tokens | 200K tokens |
| Handoff at | 80K tokens | 120K tokens | 160K tokens |

## Configuration

Global settings live in `~/.squeeze/config.json` (the Settings tab writes it). A project can override any key in its own `.squeeze/config.json`. All v0.1 keys still work ([list](../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.md#configuration)). New in v0.2:

| Key | Default | Meaning |
|---|---|---|
| `profile` | `"balanced"` | `strong`, `balanced`, `light` or `custom` |
| `dup_read` | `true` | Block full re-reads of unchanged files |
| `budget.enabled` / `budget.tokens` | `true` / `150000` | Budget pause and its size |
| `handoff.enabled` / `handoff.tokens` | `true` / `120000` | Handoff and its size |
| `handoff.mode` | `"note"` | `note` (write a note, load it after `/clear`) or `remind` (only suggest `/compact`) |
| `handoff.max_age_hours` | `48` | Notes older than this are not loaded |
| `mcp_prune.enabled` / `unused_days` | `false` / `14` | Turn off MCP servers a project has not used |
| `desktop.allowed_dirs` | `[]` | Folders the desktop app may read |

Put `SQUEEZE_SKIP` anywhere in a message to send it past the budget pause.

## Commands

| Command | What it does |
|---|---|
| `squeeze auditor` | Open the squeeze-Auditor window |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Connect or disconnect Claude Code (all projects) and the desktop app |
| `squeeze doctor [--dir D] [--desktop] [--json]` | Fixed cost per message, in the terminal |
| `squeeze mcp` | The MCP server the desktop app runs |
| `squeeze init`, `run`, `compress`, `clip`, `report`, `bench`, `uninstall` | As in [v0.1](../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.md#usage) |

## Privacy and safety

- **Local only.** No network code except the window's own `127.0.0.1` page, which needs a key that changes every launch.
- **Backups.** Before changing `~/.claude/settings.json`, `~/.claude.json` or `claude_desktop_config.json`, squeeze copies the original to `~/.squeeze/backups/`.
- **Read-only where it can be.** squeeze reads Claude Code's session logs for numbers; it never changes them and never reads your credentials.
- **Measuring an MCP server** starts that server's own command for a few seconds to ask for its tool list, just as Claude Code would.

## FAQ

**Q. Won't the budget pause get in the way?**
A. It holds a message only once per stage. Send the same message again and it goes through. You can raise the budget, pick Light, or turn it off in Settings.

**Q. Does a handoff lose context?**
A. Some detail, by design: the note keeps the goal, what is done, decisions, files touched and next steps. Notes are plain files in `.squeeze/handoff/`; read or edit them before you `/clear`. Prefer to decide yourself? Set the handoff style to "Only remind me".

**Q. Why not the browser and the phone?**
A. A browser tab and a phone app cannot run an installed program next to Claude. v0.3 will take a different approach for those.

**Q. Is the dashboard my plan's usage limit?**
A. No. It is what Claude Code's own logs on this computer say. Anthropic does not offer an official way for other programs to read plan usage.

**Q. Can I go back to v0.1?**
A. Yes: `squeeze setup --remove`, then install v0.1.0 (`SQUEEZE_VERSION=v0.1.0` with the install script).

## Feedback

Please leave improvements and problems in [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose): a blocker that fired when it should not have, installer trouble, numbers that look wrong, ideas for v0.3 and v1.0. Questions go to [Discussions](https://github.com/jongwooish/claude-squeeze/discussions). Any of the six languages is fine.

## Contributing

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
cargo run --manifest-path CARGO/Cargo.toml -- auditor     # open the window from source
```

Code map: `src/guards.rs` (repeated read, budget, handoff), `src/mcp.rs` and `src/doctor.rs` (fixed cost), `src/transcripts.rs` and `src/usage.rs` (dashboard numbers), `src/auditor.rs` and `ui/index.html` (the window), `src/mcp_server.rs` (desktop app tools), `packaging/` (installers). User guide in Korean: [docs/guide](docs/guide/index.md).
