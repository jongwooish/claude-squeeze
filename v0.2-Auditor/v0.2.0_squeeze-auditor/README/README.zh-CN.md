# v0.2.0 squeeze-Auditor

[English](../README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **欢迎反馈。** 改进建议、v1.0 发布、版本更新、安装错误或任何其他问题，请[提交 Issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose)（6 种语言均可）。

← [返回 squeeze](../../../README/README.zh-CN.md)

v0.1 Sentinel 一次拦下一条长输出。**v0.2 Auditor 着眼于整个会话。** 它记得 Claude 已经读过什么，留意对话已经变得多大，并在你打字之前检查每条消息都会附带发送的内容。然后拦下四种仅凭大小看不出来的浪费。

它是一个可安装的程序：运行安装程序，从开始菜单（或“应用程序”）打开 **squeeze-Auditor**，就能看到 token 用在了哪里、squeeze 拦下了什么，并调整节省力度。

**适用于：** Claude Code（所有项目）和 Claude 桌面应用（Windows、macOS）。浏览器中的 claude.ai 和手机应用将在 v0.3 支持。
**前提：** 无。v0.2 与 v0.1 是同一个 `squeeze` 程序，包含 v0.1 的全部功能，并增加了以下功能。参见[各版本之间的关系](../../../README/README.zh-CN.md#各版本之间的关系)。

![squeeze-Auditor 仪表盘](../docs/images/auditor-en-dashboard.png)

## 新功能：四种拦截

| | 拦截 | 拦什么 | 怎么拦 |
|---|---|---|---|
| 1 | **重复读取** | Claude 又要完整读取一个没有变化的文件 | 拒绝这次 `Read`，并告诉它“已经读过了，用已有内容”。文件一变化或对话被压缩，就立即重新允许。 |
| 2 | **预算暂停** | 你不断向一个已经非常大的会话发送消息 | 发送前暂停一次，建议 `/compact` 或 `/clear`。再发一次（↑、回车）就会照常发送。会话再增长 50% 之前不会再问。 |
| 3 | **交接（Handoff）** | 长会话每次都要重新读取全部历史 | 会话超过交接大小时，Claude 在停下之前写一份简短的交接笔记。你执行 `/clear` 后，新会话从笔记（约 1,000 token）开始，而不是整个对话。 |
| 4 | **固定开销** | *每条*消息都会附带的内容：`CLAUDE.md` 及其导入、技能和代理的描述、MCP 服务器的工具列表 | squeeze-Auditor 逐项测量。闲置的 MCP 服务器一键关闭（使用 Claude Code 自己的开关，与 `/mcp` 相同）。可选：自动关闭项目 14 天未使用的服务器。 |

### 与 v0.1 的区别

| | v0.1 Sentinel | v0.2 Auditor |
|---|---|---|
| 关注 | **一条**输出或一个文件的大小 | **会话**：之前读过什么、现在多大、每次附带什么 |
| 判断依据 | “这条输出长吗？” | 记忆（同一文件，未变化）、累计（会话大小）、历史（从未使用的服务器） |
| 钩子 | `PreToolUse`（Bash、Read） | 另有 `UserPromptSubmit`、`Stop`、`SessionStart`、`PreCompact` |
| 安装 | 每个项目 `squeeze init` | 安装程序或 `squeeze setup`，对**所有**项目一次完成 |
| 查看 | 终端里的 `squeeze report` | squeeze-Auditor 窗口 |

v0.1 的功能照常运行：输出多的命令走 `squeeze run`，大文件给出目录。

## squeeze-Auditor 窗口

| 标签页 | 可以做什么 |
|---|---|
| **仪表盘** | 来自 Claude Code 自身会话日志的真实用量（今天、7 天、30 天），以及 squeeze 拦下的量。当前 5 小时窗口和消耗速度。按类型的拦截情况。最近会话的大小与预算对比。 |
| **节省模式** | 强、均衡（默认）、轻，以及各自设定的数值。或自定义。 |
| **设置** | 所有数值：何时压缩输出、保留几行、大文件标准、预算、交接大小与方式、固定开销规则、永不处理的命令。 |
| **固定开销** | 按项目（或桌面应用）：每个记忆文件、导入、技能、代理和 MCP 服务器每条消息的 token。测量 MCP 服务器、开关。缩小的建议。 |
| **交接** | 直接阅读所有交接笔记。 |
| **连接** | 连接或断开 Claude Code 和 Claude 桌面应用。选择桌面应用可读取的文件夹。 |

窗口是 squeeze 只在 `127.0.0.1` 上提供的一个小页面，每次启动都需要新的密钥。它以 Edge 或 Chrome 的应用窗口显示（否则用默认浏览器），关闭窗口后不久自行退出。不会向外发送任何内容。支持英语和韩语。

![固定开销标签页](../docs/images/auditor-en-fixed.png)

## 效果

**输出压缩**（来自 v0.1，未改变）：7 份真实日志，估算 83,325 → 5,128 token（**−93.8%**），每种不同的错误和警告都保留。方法：[docs/bench.md](../docs/bench.md)。

**四种拦截**以另一种方式节省：阻止同样的 token 被一遍遍发送。效果取决于你的使用方式，所以这里给出的是计算，而不是基准测试：

| 拦截 | 例子 | 拦下的量 |
|---|---|---|
| 重复读取 | 再次读取一个未变化的 600 行文件（约 7,000 token） | 约 7,000 token，且会话之后每条消息都省下同样的量 |
| 预算暂停 / 交接 | 在 150,000 token 的会话里再聊 20 条 vs 从 1,000 token 的笔记开始新会话 | 约 3,000,000 输入 token 的重复读取 vs 约 20,000 + 新工作 |
| 固定开销 | 一个闲置的 12 工具 MCP 服务器（测试服务器测得 1,882 token），100 条消息 | 约 188,000 token |

**坦白的局限。**
- token 数是估算（字节 ÷ 4）。长会话的大部分内容是从 Claude 的提示缓存中重新读取的，缓存比新输入便宜，所以实际节省的费用小于 token 数所显示的。但它仍会占满上下文窗口并计入用量。
- 仪表盘的用量来自本机 Claude Code 的会话日志。不包括 claude.ai 或桌面应用自身的对话，也不是你套餐的官方额度。5 小时窗口是对 Claude Code 计算方式的估算。
- squeeze 看不到桌面应用的对话。在桌面应用中，只有当 Claude 通过 squeeze 的工具读取日志，或你让它保存交接时才会节省。

## 安装

从[最新发布](https://github.com/jongwooish/claude-squeeze/releases/latest)下载：

| 系统 | 文件 | 然后 |
|---|---|---|
| Windows 10/11（大多数电脑） | `squeeze-Auditor-0.2.0-setup-x64.exe` | 运行。无需管理员权限。开始菜单 → **squeeze-Auditor** |
| Windows on ARM | `squeeze-Auditor-0.2.0-setup-arm64.exe` | 同上 |
| macOS，Apple 芯片 | `squeeze-Auditor-0.2.0-macos-arm64.pkg` | 打开。应用程序 → **squeeze-Auditor** |
| macOS，Intel | `squeeze-Auditor-0.2.0-macos-x86_64.pkg` | 同上 |
| Linux（Debian、Ubuntu） | `squeeze-auditor_0.2.0_amd64.deb` / `_arm64.deb` | `sudo apt install ./squeeze-auditor_0.2.0_amd64.deb`，然后打开应用菜单 |

**安装程序尚未进行代码签名**，所以系统会提示一次：
- **Windows** SmartScreen：“Windows 已保护你的电脑”→ **更多信息** → **仍要运行**。
- **macOS**：“无法打开，因为它来自身份不明的开发者”→ 右键点击文件 → **打开**，或 系统设置 → 隐私与安全性 → **仍要打开**。

所有文件都列在发布页面的 `checksums.txt`（SHA-256）中。

终端安装（两个程序，无开始菜单图标）：

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# 从源码（Rust 1.80+）
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

## 快速开始

1. 安装后打开 **squeeze-Auditor**。（Windows 安装程序可以顺便连接 Claude Code：保持勾选即可。）
2. 点击 **连接 Claude Code**。这会把 squeeze 的钩子加入 `~/.claude/settings.json`，作用于所有项目。其他设置和钩子会保留，修改前先写备份。
3. 使用 Claude 桌面应用？进入 **连接**，点击 **连接**，添加 Claude 可读取日志的文件夹，然后重启桌面应用。
4. 照常使用 Claude Code。回到仪表盘查看拦下了什么。

用终端：`squeeze setup`（连接）、`squeeze setup --remove`（断开）、`squeeze auditor`（打开窗口）。

**从 v0.1 升级：** 直接覆盖安装（Windows 安装程序使用与 v0.1 `install.ps1` 相同的文件夹），并连接一次。运行过 `squeeze init` 的项目照常工作；它们的钩子可以保留，也可以用 `squeeze uninstall` 删除。在 Windows 上卸载 squeeze-Auditor 会同时断开与 Claude Code 和桌面应用的连接；在 macOS 和 Linux 上请先运行 `squeeze setup --remove`。

## 会话中发生了什么

```
SessionStart ──▶ 有最近的交接笔记吗？          ──有──▶ 载入新会话
     │
你输入消息
     │
UserPromptSubmit ──▶ 会话 ≥ 预算(150K)？       ──是──▶ 暂停一次：“/compact 或 /clear，或再发一次”
     │
Claude 工作
     ├─ Bash ──▶ 输出多的命令？                 ──是──▶ “用 squeeze run -- … 运行”         (v0.1)
     └─ Read ──▶ 完整读取大文件？               ──是──▶ 目录 + 按范围读取                  (v0.1)
                 之前完整读过且未变化的文件？    ──是──▶ “用已读的内容”                     (v0.2)
     │
Stop ──▶ 会话 ≥ 交接大小(120K)？              ──是──▶ Claude 写入 .squeeze/handoff/<时间>.md
     │                                                 并提示你执行 /clear
PreCompact ──▶ 忘掉读过哪些文件（Claude 已不再持有它们）
```

所有钩子出错时都不输出任何内容，Claude Code 照常继续。squeeze 从不替你批准工具调用。

## Claude 桌面应用

连接会在 `claude_desktop_config.json` 中添加一项：带 6 个工具的 MCP 服务器 `squeeze mcp`。

| 工具 | 作用 |
|---|---|
| `read_log` | 读取已压缩的日志或文本文件（或精确的行范围） |
| `tail_log` | 日志末尾，已压缩 |
| `search_log` | 匹配的行及少量上下文 |
| `list_logs` | 允许文件夹中的文件 |
| `save_handoff` / `load_handoff` | 把当前对话保存为交接笔记，在新对话中载入 |

Claude 只能读取你在 **连接** 中列出的文件夹。与其把 5,000 行日志粘贴进对话，不如说“读一下我应用日志里最后的错误”，Claude 拿到的是精简版。对话变长时，让 Claude 保存交接，开一个新对话再让它载入。

## 节省模式

| | 强 | 均衡（默认） | 轻 |
|---|---:|---:|---:|
| 不超过此行数的输出保持原样 | 40 行 | 80 行 | 150 行 |
| 大文件标准 | 250 行 | 400 行 | 800 行 |
| 预算暂停 | 100K token | 150K token | 200K token |
| 开始交接 | 80K token | 120K token | 160K token |

## 配置

全局设置在 `~/.squeeze/config.json`（由设置标签页写入）。项目可在自己的 `.squeeze/config.json` 中覆盖任何键。v0.1 的键仍然有效（[列表](../../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.zh-CN.md#配置)）。v0.2 新增：

| 键 | 默认值 | 含义 |
|---|---|---|
| `profile` | `"balanced"` | `strong`、`balanced`、`light` 或 `custom` |
| `dup_read` | `true` | 拦截对未变化文件的完整重读 |
| `budget.enabled` / `budget.tokens` | `true` / `150000` | 预算暂停及其大小 |
| `handoff.enabled` / `handoff.tokens` | `true` / `120000` | 交接及其大小 |
| `handoff.mode` | `"note"` | `note`（写笔记，`/clear` 后载入）或 `remind`（只建议 `/compact`） |
| `handoff.max_age_hours` | `48` | 比这更旧的笔记不载入 |
| `mcp_prune.enabled` / `unused_days` | `false` / `14` | 关闭项目未使用的 MCP 服务器 |
| `desktop.allowed_dirs` | `[]` | 桌面应用可读取的文件夹 |

在消息任意位置加入 `SQUEEZE_SKIP` 即可跳过预算暂停。

## 命令

| 命令 | 作用 |
|---|---|
| `squeeze auditor` | 打开 squeeze-Auditor 窗口 |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | 连接或断开 Claude Code（所有项目）和桌面应用 |
| `squeeze doctor [--dir D] [--desktop] [--json]` | 在终端查看每条消息的固定开销 |
| `squeeze mcp` | 桌面应用运行的 MCP 服务器 |
| `squeeze init`、`run`、`compress`、`clip`、`report`、`bench`、`uninstall` | 与 [v0.1](../../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.zh-CN.md#用法) 相同 |

## 隐私与安全

- **只在本机。** 除了窗口自己的 `127.0.0.1` 页面外没有网络代码，而该页面也需要每次启动都会变化的密钥。
- **备份。** 修改 `~/.claude/settings.json`、`~/.claude.json` 或 `claude_desktop_config.json` 之前，squeeze 会把原文件复制到 `~/.squeeze/backups/`。
- **尽量只读。** 为统计数字读取 Claude Code 的会话日志，但从不修改它们，也从不读取你的登录凭据。
- **测量 MCP 服务器**会像 Claude Code 一样运行该服务器的命令几秒钟，询问它的工具列表。

## FAQ

**问：预算暂停会碍事吗？**
答：每个阶段只暂停一次。再发同一条消息就会照常发送。可以在设置里提高预算、选择轻模式或关闭它。

**问：交接会丢失上下文吗？**
答：会有意舍弃部分细节：笔记保留目标、已完成的事、决定、改动的文件和下一步。笔记是 `.squeeze/handoff/` 里的普通文件，`/clear` 之前可以阅读或修改。想自己决定？把交接方式设为“只提醒我”。

**问：为什么不支持浏览器和手机？**
答：浏览器标签页和手机应用无法在 Claude 旁边运行已安装的程序。v0.3 会换一种方法。

**问：仪表盘上的是我套餐的用量额度吗？**
答：不是。那是本机 Claude Code 日志记录的数字。Anthropic 没有提供让其他程序读取套餐用量的官方方式。

**问：能回到 v0.1 吗？**
答：可以：`squeeze setup --remove`，然后安装 v0.1.0（安装脚本加 `SQUEEZE_VERSION=v0.1.0`）。

## 反馈

请在 [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) 留下改进建议和问题：不该拦截时拦截了、安装问题、看起来不对的数字、对 v0.3 和 v1.0 的想法。提问请到 [Discussions](https://github.com/jongwooish/claude-squeeze/discussions)。6 种语言均可。

## 参与贡献

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
cargo run --manifest-path CARGO/Cargo.toml -- auditor     # 从源码打开窗口
```

代码地图：`src/guards.rs`（重复读取、预算、交接）、`src/mcp.rs` 和 `src/doctor.rs`（固定开销）、`src/transcripts.rs` 和 `src/usage.rs`（仪表盘数字）、`src/auditor.rs` 和 `ui/index.html`（窗口）、`src/mcp_server.rs`（桌面应用工具）、`packaging/`（安装程序）。韩语用户指南：[docs/guide](../docs/guide/index.md)。
