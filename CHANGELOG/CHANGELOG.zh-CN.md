# 更新日志

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | **简体中文** | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

这里记录所有重要变更。版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。主版本号为 0 期间，命令和配置可能还会变化。

## [0.2.0] - 2026-11-04 — squeeze-Auditor

v0.2 关注整个会话，而不是一条输出。代码位于 [`v0.2-Auditor/v0.2.0_squeeze-auditor/`](../v0.2-Auditor/v0.2.0_squeeze-auditor/)。包含 0.1.0 的全部功能。

### 新增

- 四种会话拦截（Claude Code）：
  - **重复读取**：对本会话中已读过且未变化的文件再次完整 `Read` 时拒绝。文件变化或上下文被压缩后重新允许。
  - **预算暂停**（`UserPromptSubmit`）：会话超过预算（默认 150K token）时暂停一次消息，建议 `/compact` 或 `/clear`。再发一次即可照常发送。
  - **交接**（`Stop` + `SessionStart`）：会话超过交接大小（默认 120K）时，Claude 在 `.squeeze/handoff/` 写下笔记；`/clear` 后新会话从笔记开始。`remind` 模式只建议 `/compact`。
  - **固定开销**：测量每条消息都会发送的 `CLAUDE.md` 及其导入、技能、代理和 MCP 工具列表。用 Claude Code 自己的开关按项目关闭 MCP 服务器，或 N 天未用自动关闭（默认关闭）。
- **squeeze-Auditor** 应用（`squeeze-auditor`、`squeeze auditor`）：仪表盘显示来自 Claude Code 会话日志的真实用量与 squeeze 拦下的量、当前 5 小时窗口、节省模式（强、均衡、轻、自定义）、全部设置、带测量与开关的固定开销诊断、交接笔记、连接管理。仅在本机（`127.0.0.1`，一次性密钥）。英语和韩语。
- **Claude 桌面应用**支持：`squeeze mcp`，一个 MCP 服务器，提供 `read_log`、`tail_log`、`search_log`、`list_logs`、`save_handoff`、`load_handoff`，仅限你允许的文件夹。
- `squeeze setup [--claude-code] [--desktop] [--remove]`：一次连接所有项目（`~/.claude/settings.json`）和桌面应用，备份在 `~/.squeeze/backups/`。
- `squeeze doctor`：在终端查看每条消息的固定开销。
- 安装程序：Windows Setup（x64、ARM64、按用户安装、开始菜单图标、卸载时断开连接）、包含 `squeeze-Auditor.app` 的 macOS `.pkg`、带菜单项的 Linux `.deb`。尚未代码签名。
- 全局配置 `~/.squeeze/config.json`（项目的 `.squeeze/config.json` 仍可覆盖）和全局记录 `~/.squeeze/ledger.jsonl`。

### 变更

- 安装脚本也会安装 `squeeze-auditor`。
- 发布标题改为取自 CHANGELOG 标题。CI 也会构建安装程序并测试冻结的 v0.1.0 快照。
- 冻结的 v0.1.0 crate 改名为 `squeeze-sentinel`（程序名仍是 `squeeze`），因此 `cargo install --git … squeeze` 构建的是 v0.2。

### 计划

- v0.3：浏览器中的 claude.ai 和手机应用，采用新的方法。

## [0.1.0] - 2026-11-03 — Sentinel

首个版本。代码位于 [`v0.1-Sentinel/v0.1.0_squeeze-sentinel/`](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/)。

### 新增

- Claude Code `PreToolUse` 钩子：
  - **Bash 守卫**：把高输出命令（安装、构建、测试、日志）引导到 `squeeze run`。已限制输出的命令（`| tail`、`-n 50`、重定向、`--quiet`）不受影响。
  - **Read 守卫**：把对超过 400 行文件的整体读取，引导为附带大纲的按范围读取。
  - 钩子绝不批准任何工具调用，遇到任何错误都会安静退出（fail open）。
- `squeeze run -- <command>`：运行命令、压缩长输出、保存完整日志、保留退出码。
- `squeeze compress`：压缩管道输出。
- `squeeze clip`：压缩剪贴板以便粘贴到 claude.ai（macOS、Windows、Linux）。
- `squeeze report`：按项目统计节省量（clip 用 `--global`，另有 `--json`、`--since`）。
- `squeeze bench`：回放日志，测量节省量和保留的消息。
- `squeeze init` / `squeeze uninstall`：安装并干净地移除钩子、配置、`.gitignore` 条目和 `CLAUDE.md` 说明。
- 压缩器：保留开头和结尾、每一条不同的错误和警告及其上下文、堆栈跟踪；合并重复；优先保留错误。
- 7 份基准测试日志：估算 token 减少 93.8%，不同消息保留 50/50。
- macOS/Linux（`install.sh`）和 Windows（`install.ps1`）安装脚本，带 SHA-256 校验。
- 英语、韩语、日语、简体中文、西班牙语、法语 README。

### 计划

- v0.2 Interceptor：用于 claude.ai 粘贴的浏览器扩展。
- v0.3 Conduit：面向 Claude 桌面应用的 MCP 服务器。
- Claude Code 实际 A/B 基准测试。
