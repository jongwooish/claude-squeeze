# 更新日志

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | **简体中文** | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

这里记录所有重要变更。版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。主版本号为 0 期间，命令和配置可能还会变化。

## [0.1.0] - 2026-11-03 — Sentinel

首个版本。代码位于 [`v0.1_squeeze-sentinel/`](v0.1_squeeze-sentinel/)。

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
