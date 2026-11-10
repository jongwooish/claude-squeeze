# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#安装) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **欢迎反馈。** 改进建议、v1.0 发布、版本更新、安装错误或任何其他问题，请[提交 Issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose)（支持 6 种语言）。

**把 Claude 的 token 用在思考上，而不是刷屏的日志上。**

squeeze 让冗长的命令输出、对大文件的整体读取和重复读取、过大的会话以及闲置的 MCP 服务器不进入 Claude 的上下文。Claude 仍然能看到每一条错误和警告。可安装的应用 **squeeze-Auditor** 显示 token 用在了哪里，并让你选择 squeeze 的力度。

- [效果](#效果) · [版本](#版本) · [适用范围](#适用范围) · [v0.2 squeeze-Auditor](#v02-squeeze-auditor) · [v0.1 Sentinel](#v01-sentinel) · [安装](#安装) · [用法](#用法) · [工作原理](#工作原理) · [路线图](#路线图) · [FAQ](#faq)

## 为什么需要

Claude Code 每次请求都会重新发送整个对话，而命令输出也是对话的一部分。一次 `npm install`、一次测试或一次 `journalctl` 就可能增加数千个 token，并在之后的每一轮里一直跟着。可 Claude 真正需要的只是“通过了”这一行，或者三行错误。

squeeze 在这些输出进入上下文**之前**就把它们拦下。

## 效果

用 squeeze 处理 7 份真实日志的结果（[测量方法](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/docs/bench.md)）：

| 日志 | 行数 前 → 后 | Token 前 → 后（估算） | 节省 | 保留的不同错误/警告 |
|---|---:|---:|---:|---:|
| `pytest`，2 个失败 | 217 → 39 | 4,040 → 516 | **87.2%** | 11 / 11 |
| `go test -v`，1 个失败 | 488 → 16 | 4,247 → 202 | **95.2%** | 4 / 4 |
| `make`，gcc 警告 + 1 个错误 | 370 → 44 | 7,046 → 850 | **87.9%** | 7 / 7 |
| `tsc`，2 个类型错误 | 124 → 9 | 1,310 → 172 | **86.9%** | 3 / 3 |
| `dpkg -l` 软件包列表 | 1049 → 63 | 35,237 → 2,106 | **94.0%** | 8 / 8 |
| 服务器日志，OOM 被杀（合成数据） | 1400 → 37 | 31,161 → 997 | **96.8%** | 5 / 5 |
| 很短的 `npm` 输出 | 16 → 16 | 287 → 287 | 0.0%（原样保留） | 12 / 12 |
| **合计** | | **83,325 → 5,128** | **93.8%** | **50 / 50** |

**有什么用**

- **在这些测试中没有丢失任何重要信息。** 每一条不同的错误和警告都被保留。重复的消息只显示一次并附上次数（`… [same message 29 more times] …`）。
- **短输出绝不改动。** 不超过 80 行且不超过 8 KB 时，原样返回。
- **每一轮都在节省。** 一份 31,000 token 的日志如果在接下来 20 轮里一直留在上下文中，就是约 620,000 个输入 token。压缩到约 1,000 token 后，同样 20 轮只需约 20,000。
- **更晚碰到上限。** 噪音少了，上下文填满得更慢，更晚触发对话压缩和套餐用量上限。
- **Claude 更专注。** “构建为什么失败？”的答案就在眼前，而不是埋在 1,000 行里。

**坦白的局限：** token 数是估算值（字节 ÷ 4），这些数字来自回放已保存的日志，而不是在 Claude Code 中实际开关对比的结果（实测对比在计划中）。在你自己的项目里运行 `squeeze report` 可以看到真实数字。

**v0.2 新增四种拦截**，阻止同样的 token 被一遍遍发送：重复读取未变化的文件、超出预算的会话、应该从简短交接笔记重新开始的长会话，以及闲置 MCP 服务器之类的固定开销。效果取决于你的使用方式，所以 [v0.2 README](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.zh-CN.md#效果) 给出计算，squeeze-Auditor 仪表盘显示你自己的数字。

## 版本

每个版本都有自己的文件夹和 README。发布是整个仓库上的标签。

| 版本 | 名称 | 新增内容 | 状态 |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.zh-CN.md) | **Sentinel** | 拦截输出多的命令和整体文件读取的 Claude Code 钩子，`squeeze run`、`compress`、`clip`、`report` | ✅ v0.1.0 |
| [v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.zh-CN.md) | **squeeze-Auditor** | 四种会话拦截（重复读取、预算暂停、交接、固定开销），带用量仪表盘和节省模式的可安装应用 squeeze-Auditor，支持 Claude 桌面应用 | ✅ v0.2.0 |
| v0.3 | *（名称待定）* | 浏览器中的 claude.ai 和手机应用，以新的方式解决 token 方面的抱怨 | 🛠 计划中 |

### 各版本之间的关系

| 版本 | 安装什么 | 需要之前的版本吗？ |
|---|---|---|
| v0.1 Sentinel | `squeeze` 程序 | — |
| v0.2 squeeze-Auditor | 同一个 `squeeze` 程序，外加安装程序、开始菜单应用（`squeeze-auditor`）和新钩子 | **不需要，它包含 v0.1。** 可以覆盖 v0.1 安装，也可以单独安装。 |
| v0.3 | 计划中 | 另行公布 |

- **程序是累积的。** 新的 `squeeze` 包含之前所有功能。不会同时安装 v0.1 和 v0.2。
- **文件夹说明每个版本新增了什么。** 你不需要安装文件夹。
- **设置会延续。** v0.1 `squeeze init` 生成的 `.squeeze/config.json` 和钩子在更新后仍然有效。v0.2 新增全局 `~/.squeeze/config.json`，并为所有项目一次性连接 Claude Code。

| 你使用… | 安装 |
|---|---|
| Claude Code | squeeze-Auditor（v0.2）→ **连接 Claude Code** |
| Claude 桌面应用 | squeeze-Auditor（v0.2）→ **连接桌面应用** |
| 浏览器中的 claude.ai 或手机应用 | 目前长文本粘贴用 `squeeze clip`；之后是 v0.3 |

## 适用范围

| 在哪里 | 效果 | 方式 |
|---|---|---|
| **Claude Code** | 自动，所有项目 | squeeze-Auditor（或 `squeeze setup`）把钩子加入 `~/.claude/settings.json`。v0.2 的四种拦截和 v0.1 的两种守卫全部生效。 |
| **Claude 桌面应用**（Windows、macOS） | Claude 读取文件或保存交接时 | 连接后会添加 `squeeze mcp` 工具：Claude 只从你允许的文件夹读取已压缩的日志，并能保存和载入交接笔记。squeeze 看不到也改不了对话本身。 |
| **claude.ai（网页、手机）** | 手动，粘贴长文本时 | 复制日志，运行 `squeeze clip`，再粘贴。计划在 v0.3 支持。 |
| **其他终端工具** | 手动 | `<命令> 2>&1 \| squeeze compress` |

在 Pro 套餐中，Claude Code 和 claude.ai 共用同一个用量上限，所以在 Claude Code 里省下的 token 也会留给聊天使用。

## v0.2 squeeze-Auditor

v0.1 一次看一条输出。**v0.2 看整个会话**：Claude 已经读过什么、对话变得多大、每条消息附带了什么。它拦下四种浪费：

| 拦截 | 拦什么 | 怎么拦 |
|---|---|---|
| **重复读取** | 完整重读一个没有变化的文件 | 拒绝这次 `Read`：“你已经有了”。文件变化或压缩后重新允许。 |
| **预算暂停** | 向一个巨大的会话（默认 150K token）继续发消息 | 暂停一次，建议 `/compact` 或 `/clear`。再发一次就会照常发送。 |
| **交接** | 每次都重读全部历史的长会话（默认 120K） | Claude 写一份简短的交接笔记；`/clear` 之后，新会话从笔记开始，而不是整个对话。 |
| **固定开销** | 每条消息都会发送的 `CLAUDE.md`、技能、代理和 MCP 工具列表 | 逐项测量；闲置的 MCP 服务器一键关闭（Claude Code 自己的开关），或 14 天未用自动关闭。 |

**squeeze-Auditor** 是从开始菜单或“应用程序”打开的应用：显示真实 Claude Code 用量与 squeeze 拦下的量的仪表盘、节省模式（强、均衡、轻）、全部设置、固定开销诊断、交接笔记，以及与 Claude Code 和桌面应用的连接。

![squeeze-Auditor 仪表盘](../v0.2-Auditor/v0.2.0_squeeze-auditor/docs/images/auditor-en-dashboard.png)

v0.2 完整文档：[v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.zh-CN.md](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.zh-CN.md)。

## v0.1 Sentinel

Sentinel 是守在 Claude Code 和 shell 之间的哨兵。单个可执行文件，Rust 编写，**零依赖**，支持 macOS / Linux / Windows。

- **Bash 守卫**：高输出命令（`npm install`、`go test`、`cargo build`、`pytest`、`make`、`apt-get install`、`journalctl`、`docker build`、`kubectl logs`、`git log` 等）会被引导改用 `squeeze run -- <command>` 执行。已经限制了输出的命令（`| tail -20`、`-n 50`、`> file`、`--quiet`）不受影响。
- **Read 守卫**：不带 `offset`/`limit` 读取超过 400 行的文件时会被引导。Claude 会拿到带行号的大纲，然后只读需要的范围。
- **智能压缩**：保留前 8 行和后 20 行、每条错误和警告及其前后 2 行、堆栈跟踪；重复消息合并计数；优先保留错误而非警告。
- **保留退出码**：`squeeze run` 返回原命令自己的退出码。
- **完整日志存在磁盘上**：`.squeeze/raw/`，保留最新 50 份，仅本人可读。`.squeeze/` 会加入 `.gitignore`。
- 面向 claude.ai 的 **`squeeze clip`**，查看真实节省量的 **`squeeze report`**。
- **安全设计**：钩子绝不替你批准任何操作，出错时安静退出（fail open），`squeeze uninstall` 会移除它添加的一切。没有任何网络代码。

运行 `squeeze run -- pytest -v` 后 Claude 看到的内容（真实输出，217 行 → 39 行）：

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
… (两段 traceback 完整保留) …
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

v0.1 完整文档：[v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.zh-CN.md](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.zh-CN.md)。

## 安装

**安装程序（推荐）。** 从[最新发布](https://github.com/jongwooish/claude-squeeze/releases/latest)下载：

| 系统 | 文件 |
|---|---|
| Windows 10/11 | `squeeze-Auditor-0.2.0-setup-x64.exe`（ARM：`-setup-arm64.exe`） |
| macOS | `squeeze-Auditor-0.2.0-macos-arm64.pkg`（Intel：`-macos-x86_64.pkg`） |
| Linux（Debian、Ubuntu） | `squeeze-auditor_0.2.0_amd64.deb`（ARM：`_arm64.deb`） |

安装程序尚未进行代码签名。Windows SmartScreen：**更多信息 → 仍要运行**。macOS：右键 → **打开**。详见 [v0.2 安装](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.zh-CN.md#安装)。

**终端：**

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# 从源码（任何装有 Rust 1.80+ 的系统）
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

每个发布文件都列在 `checksums.txt`（SHA-256）中，安装脚本会校验。`SQUEEZE_VERSION=v0.1.0` 可固定版本；`SQUEEZE_INSTALL_DIR` 可更改安装文件夹。

## 快速开始

1. 打开 **squeeze-Auditor**，点击 **连接 Claude Code**（所有项目）。使用桌面应用？在 **连接** 中连接并重启它。
2. 照常使用 Claude Code。
3. 回到仪表盘查看拦下了什么。在 **节省模式** 或 **设置** 中调整力度。

用终端：

```bash
squeeze setup      # 连接 Claude Code（如已安装桌面应用，也一并连接）
claude             # 照常使用 Claude Code
squeeze auditor    # 打开窗口
squeeze setup --remove
```

连接只修改 `~/.claude/settings.json`（桌面应用为 `claude_desktop_config.json`）。其他设置和钩子会保留，修改前原文件会先备份到 `~/.squeeze/backups/`。v0.1 按项目的 `squeeze init` 仍然可用。

## 用法

| 命令 | 作用 |
|---|---|
| `squeeze auditor` | 打开 squeeze-Auditor 窗口 |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | 连接或断开 Claude Code（所有项目）和桌面应用 |
| `squeeze doctor [--dir D] [--desktop] [--json]` | 每条消息都会发送的内容（固定开销） |
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | 只给一个项目安装钩子（v0.1 方式） |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | 移除项目的钩子和说明；`--purge` 还会删除 `.squeeze/` |
| `squeeze run -- <命令>` | 运行命令；输出过长时给出简短摘要 |
| `<命令> 2>&1 \| squeeze compress` | 压缩管道输出 |
| `squeeze clip [--print]` | 压缩剪贴板（用于 claude.ai） |
| `squeeze report [--since 7d] [--global] [--json]` | 查看节省量 |
| `squeeze bench <dir> [--markdown]` | 回放 `*.log` 文件并测量 |
| `squeeze mcp` | 桌面应用运行的 MCP 服务器 |
| `squeeze version` | 打印版本 |

在命令前加 `SQUEEZE_SKIP=1` 可以获得一次原始输出；在消息中加入 `SQUEEZE_SKIP` 可以跳过预算暂停。设置：[v0.2 配置](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.zh-CN.md#配置)。

## 工作原理

```
Claude 想要运行:  npm test
        │
        ▼
PreToolUse 钩子 ── squeeze hook pre-bash
        │  高输出? ── 否 ──▶ 保持沉默，走正常权限流程
        │  是
        ▼
拒绝 + 提示: “请用 squeeze run -- 'npm test' 运行”
        │
        ▼
Claude 运行: squeeze run -- 'npm test'   （照常应用你的权限规则）
        │
        ├─▶ 完整输出 ── .squeeze/raw/<时间>.log
        └─▶ 简短输出 ── Claude（错误、警告、开头、结尾、日志路径）
```

为什么是“拒绝 + 提示”，而不是悄悄改写命令？改写命令的钩子还必须批准该命令，这会跳过你的权限确认。squeeze 绝不替你批准任何操作。

### 与包装器的区别

像 [chaude](https://github.com/int-ttt/chaude) 这样的工具用自己的界面包装 Claude Code CLI，通过轮换会话、把工作交给更便宜的模型来节省 token。squeeze 负责问题的另一面：

- **不是包装器。** 通过 Claude Code 自带的钩子工作，你照常使用 `claude`。
- **在门口拦住浪费**，而不是事后清理塞满的上下文。
- **两者可以一起使用。**

## 路线图

- [x] **v0.1 Sentinel**：Claude Code 钩子，`run`、`compress`、`clip`、`report`、`bench`
- [x] **v0.2 squeeze-Auditor**：重复读取、预算、交接和固定开销拦截；带仪表盘和节省模式的可安装应用；Claude 桌面应用工具
- [ ] **v0.3**：浏览器中的 claude.ai 和手机应用，以新的方式解决大家最常提到的 token 问题
- [ ] 代码签名的安装程序
- [ ] 真实的 Claude Code A/B 基准测试（同样的任务，开关 squeeze）
- [ ] v1.0：命令和配置稳定

## FAQ

**问：Claude 会漏掉重要的东西吗？**
答：在所有测试日志中，每一条不同的错误和警告都被保留了。如果摘要不够，最后一行给出完整日志的路径。拦截重复读取时，也会告诉 Claude 真正需要时如何重新读取。

**问：squeeze 会把我的数据发到别处吗？**
答：不会。它没有网络代码；squeeze-Auditor 窗口是只有你的电脑能打开的 `127.0.0.1` 页面。

**问：它会弄坏我的 Claude Code 会话吗？**
答：钩子出错时安全放行：输入异常、文件缺失或内部崩溃都不会输出任何内容，Claude Code 照常继续。squeeze 从不替你批准工具调用。

**问：普通的 claude.ai 聊天也能省 token 吗？**
答：只有在运行 `squeeze clip` 后粘贴长文本时才会。浏览器和手机应用是 v0.3 的重点。

**问：仪表盘显示的是我套餐的用量额度吗？**
答：不是。它读取的是本机 Claude Code 的日志。没有让其他程序读取套餐用量的官方方式。

**问：为什么是 0.x 版本？**
答：主版本号为 0 时，命令名和配置可能还会变化。v1.0 表示它们已经稳定。

## 仓库结构

```
claude-squeeze/
├── README/                      ← 6 种语言的概览（README.md = 英语）
├── CHANGELOG/                   ← 6 种语言的 CHANGELOG
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   版本概览
│   └── v0.1.0_squeeze-sentinel/   ← 发布时的 v0.1.0（冻结）
├── v0.2-Auditor/                ← v0.2 squeeze-Auditor
│   ├── README.md                   版本概览
│   └── v0.2.0_squeeze-auditor/    ← v0.2：当前的 squeeze 程序（Rust，无依赖）
│       ├── README.md               v0.2 完整文档（英语）
│       ├── README/                 翻译（5 种语言）
│       ├── src/                    引擎、钩子、拦截、squeeze-Auditor 服务器
│       ├── ui/                     squeeze-Auditor 窗口（一个 HTML 文件）
│       ├── packaging/              Windows、macOS、Linux 安装程序和图标
│       ├── docs/                   基准测试、用户指南（韩语）、截图
│       └── testdata/fixtures/      测试和基准测试使用的日志
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock（Rust 工作区）
├── .github/                     ← Issue 表单、CI、发布、首页 README
├── CLAUDE.md
└── LICENSE
```

## 参与贡献

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
```

欢迎提交新的测试日志：把真实的 `*.log`（删除机密信息和个人路径）放到 `v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures/`。只要丢失任何一条不同的错误或警告，测试就会失败。Bug 和想法：[GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues)。

## 社区

**发现问题或有想法？请在 [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) 中留言。** 改进建议、v1.0 发布、版本更新、安装错误以及任何软件问题，每一条反馈都会帮助打磨下一个版本。


| 我想要… | 去这里 |
|---|---|
| 报告 bug 或会话被破坏 | [Issues → Bug report](https://github.com/jongwooish/claude-squeeze/issues/new?template=bug_report.yml) |
| 报告重要行丢失、节省太少、命令误拦或漏拦 | [Issues → Compression problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=compression.yml) |
| 报告安装、更新或版本问题 | [Issues → Install, update or version problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=version_problem.yml) |
| 提出功能建议 | [Issues → Feature idea](https://github.com/jongwooish/claude-squeeze/issues/new?template=feature_request.yml) |
| 提问或寻求排障帮助 | [Discussions → Q&A](https://github.com/jongwooish/claude-squeeze/discussions/categories/q-a) |
| 讨论路线图以及 v1.0 应包含什么 | [Discussions → Ideas](https://github.com/jongwooish/claude-squeeze/discussions/categories/ideas) |
| 获取新版本消息 | [Discussions → Announcements](https://github.com/jongwooish/claude-squeeze/discussions/categories/announcements) 和 [Releases](https://github.com/jongwooish/claude-squeeze/releases)（点击 **Watch → Custom → Releases** 接收通知） |

六种语言任选其一即可。安全问题见 [SECURITY.md](../.github/SECURITY.md)，贡献方式见 [CONTRIBUTING.md](../.github/CONTRIBUTING.md)。

## 许可证

MIT，见 [LICENSE](../LICENSE)。squeeze 是非官方的社区工具，与 Anthropic 无关。
