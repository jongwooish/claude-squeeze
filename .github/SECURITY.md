# Security policy

squeeze runs inside your Claude Code sessions and writes logs to your disk, so security reports matter.

## What counts

- A hook that approves a tool call, or that can be made to run a command it should not
- Raw logs or the ledger written somewhere other than `.squeeze/`, or with loose permissions
- `squeeze init` / `uninstall` damaging `.claude/settings.json`, `CLAUDE.md` or `.gitignore`
- `squeeze setup` or squeeze-Auditor damaging `~/.claude/settings.json`, `~/.claude.json` or `claude_desktop_config.json`
- The squeeze-Auditor window reachable from anything other than `127.0.0.1`, or without its per-launch key
- The desktop app tools (`squeeze mcp`) reading outside the allowed folders
- The installers skipping or bypassing the checksum check

## How to report

Use GitHub's private reporting: open the **Security** tab of this repository and click **Report a vulnerability**. Please do not open a public issue with details.

If that button is not available, open a short public issue that says only "security report, please contact me", without any details, and you will be contacted.

## Supported versions

Only the latest release gets fixes while squeeze is at version 0.x.
