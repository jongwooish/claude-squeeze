# 📌 Issue guide

**English** | [한국어](ISSUE_GUIDE.ko.md) | [日本語](ISSUE_GUIDE.ja.md) | [简体中文](ISSUE_GUIDE.zh-CN.md) | [Español](ISSUE_GUIDE.es.md) | [Français](ISSUE_GUIDE.fr.md)

Hi, and thank you for stopping by! 👋
claude-squeeze gets better because people tell us what went wrong and what they wish it did. Every report counts, even a one-line one. This page explains how to write an issue that gets fixed quickly.

### What you can post here

- 🐞 **Something broke**: squeeze crashed, printed something odd, or a Claude Code session behaved strangely
- ✂️ **Compression felt wrong**: an important line disappeared, too much was kept, or a command was (or was not) redirected
- 📦 **Install, update or version trouble**: the installer failed, a release file was missing, or something changed after updating
- 💡 **Ideas and improvements**: new features, better defaults, and what you would like to see in **v1.0**
- ❓ **Questions**: not sure if it is a bug? Ask anyway. Nobody will mind.

### Before you post (2 minutes)

1. **Check your version**: run `squeeze version`. If it is not the [latest release](https://github.com/jongwooish/claude-squeeze/releases/latest), please update first. The problem may already be fixed.
2. **Search existing issues**: someone may have reported it. If so, add a 👍 or a comment with your details instead of a new issue.
3. **Skim the [FAQ](https://github.com/jongwooish/claude-squeeze#faq)**: common questions are answered there.

### Pick the right form

Click **New issue** and choose:

| If… | Choose |
|---|---|
| squeeze crashed, gave wrong output, or broke a session | **Bug report** |
| an important line was dropped, or a command was (not) caught | **Compression problem** |
| installing, updating, or a specific version is the problem | **Install / update / version problem** |
| you have an idea or a wish for v1.0 | **Feature idea** |

Not sure? Pick the closest one. We will move it if needed.

### What makes a great report

- **Your setup**: squeeze version, operating system, and shell (on Windows: Git Bash, WSL or PowerShell)
- **The exact command** you or Claude ran
- **What you expected** and **what actually happened**
- **The output as text**, not a screenshot (screenshots are fine as an extra)
- **Steps to reproduce**, as short as possible

Handy commands for collecting information:

```bash
squeeze version            # your version
squeeze report --json      # what squeeze kept out of context recently
squeeze doctor --json      # what is sent with every message (v0.2)
SQUEEZE_SKIP=1 <command>   # the raw output, to compare with the summary
```

When squeeze compresses output, the last line shows where the full log was saved (`.squeeze/raw/...`). Attaching that file (after cleaning it, see below) helps a lot with compression problems.

### 🔒 Please protect your secrets

Logs can contain things you do not want to publish. Before pasting, remove or replace:

- API keys, tokens, passwords, cookies
- Internal hostnames, IP addresses, company names
- Personal paths like `C:\Users\your-name\...` or `/home/your-name/...` (replace with `~/`)

Found a **security problem**? Please do not post it publicly. Follow [SECURITY.md](../SECURITY.md) to report it privately.

### What happens next

- We read every issue and add a label (`bug`, `enhancement`, `question`, …).
- This is a personal open-source project, so a reply can take a few days. Thank you for your patience.
- We may ask a follow-up question. Answering it is often the fastest way to a fix.
- Fixes and new features are listed in the [CHANGELOG](../../CHANGELOG/CHANGELOG.md) and announced in [Releases](https://github.com/jongwooish/claude-squeeze/releases). Click **Watch → Custom → Releases** to get notified.

### A few kind rules

- One problem per issue, so each one can be tracked and closed.
- Be friendly. Everyone here is trying to help.
- Any of the six languages is fine. English or Korean gets the fastest reply.

Thank you for helping make claude-squeeze better! 🙏
