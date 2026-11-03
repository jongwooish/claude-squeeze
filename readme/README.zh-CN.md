# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.1_squeeze-sentinel/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.1_squeeze-sentinel/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#安装) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](../README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md)

**把 Claude 的 token 用在思考上，而不是刷屏的日志上。**

squeeze 让冗长的命令输出和对大文件的整体读取不进入 Claude 的上下文。Claude 仍然能看到每一条错误和警告，以及开头和结尾的几行。完整日志保存在你的磁盘上，需要时随时查看。

- [效果](#效果) · [版本](#版本) · [适用范围](#适用范围) · [v0.1 Sentinel](#v01-sentinel) · [安装](#安装) · [用法](#用法) · [工作原理](#工作原理) · [路线图](#路线图) · [FAQ](#faq)

## 为什么需要

Claude Code 每次请求都会重新发送整个对话，而命令输出也是对话的一部分。一次 `npm install`、一次测试或一次 `journalctl` 就可能增加数千个 token，并在之后的每一轮里一直跟着。可 Claude 真正需要的只是“通过了”这一行，或者三行错误。

squeeze 在这些输出进入上下文**之前**就把它们拦下。

## 效果

用 squeeze 处理 7 份真实日志的结果（[测量方法](../v0.1_squeeze-sentinel/docs/bench.md)）：

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

## 版本

每个版本都有自己的文件夹和 README。发布（release）是打在整个仓库上的标签。

| 版本 | 名称 | 新增内容 | 状态 |
|---|---|---|---|
| [v0.1](../v0.1_squeeze-sentinel/README.zh-CN.md) | **Sentinel** | 拦截高输出命令和整文件读取的 Claude Code 钩子，`squeeze run`、`compress`、`clip`、`report` | ✅ v0.1.0 |
| v0.2 | **Interceptor** | 在 claude.ai 粘贴长文本时询问是否压缩的浏览器扩展 | 🛠 计划中 |
| v0.3 | **Conduit** | 面向 Claude 桌面应用的 MCP 服务器：Claude 读取的日志已经压缩好 | 🛠 计划中 |

### 各版本之间的关系

可以安装的东西有两种：**`squeeze` 程序**（CLI），以及从 v0.2 开始的**浏览器扩展**。

| 版本 | 安装什么 | 需要 v0.1 吗？ |
|---|---|---|
| v0.1 Sentinel | `squeeze` 程序 | — |
| v0.2 Interceptor | 浏览器扩展 | **不需要。** 扩展自带压缩引擎，可以单独使用。如果你也用 Claude Code，请同时安装 v0.1。 |
| v0.3 Conduit | 无需新装：它是 `squeeze` 程序里的一个命令（`squeeze mcp`） | **需要。** 把 `squeeze` 更新到 v0.3 或更高即可，没有单独的下载。 |

- **程序是累积的。** 新版 `squeeze` 包含之前所有功能，不需要同时安装 v0.1 和 v0.3。
- **文件夹说明的是每个版本新增了什么。** 不是要你安装某个文件夹。
- **设置会延续。** `.squeeze/config.json` 和 `squeeze init` 安装的钩子在更新后继续有效。如果某个版本改动了它们，CHANGELOG 会写明，`squeeze init` 会负责更新。

| 你在用… | 安装 |
|---|---|
| 只用 Claude Code | `squeeze`（目前是 v0.1） |
| 只用 claude.ai | 现在用 `squeeze clip`，发布后用 v0.2 扩展 |
| Claude 桌面应用，希望 Claude 自己读日志 | 发布后的 `squeeze` v0.3 |
| 以上全部 | `squeeze`（最新）+ v0.2 扩展 |

## 适用范围

| 在哪里 | 效果 | 方式 |
|---|---|---|
| **Claude Code** | 自动 | `squeeze init` 安装钩子，高输出命令和大文件读取会被引导走简短路线。 |
| **claude.ai（网页、桌面、移动端）** | 仅在粘贴长文本时，手动 | 复制日志 → `squeeze clip` → 粘贴。普通聊天不受影响；squeeze 看不到你的聊天。 |
| **其他终端工具** | 手动 | `<command> 2>&1 \| squeeze compress` |

在 Pro 套餐中，Claude Code 和 claude.ai 共用同一个用量上限，所以在 Claude Code 里省下的 token 也会留给聊天使用。

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

v0.1 完整文档：[v0.1_squeeze-sentinel/README.zh-CN.md](../v0.1_squeeze-sentinel/README.zh-CN.md)。

## 安装

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 | iex

# 从源码安装（任何装有 Rust 1.80+ 的系统）
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

安装脚本会校验发布中的 `checksums.txt`（SHA-256）。用 `SQUEEZE_VERSION=v0.1.0` 固定版本，用 `SQUEEZE_INSTALL_DIR` 更改安装目录。

## 快速开始

```bash
cd ~/my-project
squeeze init      # 钩子、配置、.gitignore 条目、CLAUDE.md 说明
claude            # 照常使用 Claude Code
squeeze report    # 查看挡在上下文之外的量
```

`squeeze init` 会修改 4 个文件，`squeeze uninstall` 可以全部撤销：

| 文件 | 改动 |
|---|---|
| `.claude/settings.json` | 添加两个 `PreToolUse` 钩子（`Bash`、`Read`）。其他设置和钩子保持不变，写入前先备份。 |
| `.squeeze/config.json` | 默认配置。 |
| `.gitignore` | 添加 `.squeeze/`（原始日志可能包含敏感信息）。 |
| `CLAUDE.md` | 在 `<!-- squeeze:start -->` 标记之间添加 4 行说明。 |

## 用法

| 命令 | 作用 |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | 安装到项目 |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | 移除钩子和说明；`--purge` 同时删除 `.squeeze/` |
| `squeeze run -- <command>` | 运行命令；输出过长时给出摘要 |
| `<command> 2>&1 \| squeeze compress` | 压缩管道输出 |
| `squeeze clip [--print]` | 压缩剪贴板（用于 claude.ai） |
| `squeeze report [--since 7d] [--global] [--json]` | 查看节省量 |
| `squeeze bench <dir> [--markdown]` | 回放 `*.log` 文件并测量 |
| `squeeze version` | 显示版本 |

在命令前加 `SQUEEZE_SKIP=1` 可以让这一次拿到原始输出。配置在 `.squeeze/config.json`（[全部选项](../v0.1_squeeze-sentinel/README.zh-CN.md#配置)）。

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

- [x] **v0.1 Sentinel**：Claude Code 钩子、`run`、`compress`、`clip`、`report`、`bench`
- [ ] **v0.2 Interceptor**：用于 claude.ai 粘贴的浏览器扩展，同一引擎编译为 WebAssembly
- [ ] **v0.3 Conduit**：`squeeze mcp`，面向 Claude 桌面应用的只读 MCP 服务器
- [ ] Claude Code 实际 A/B 基准测试（同样的任务，开关 squeeze 对比）
- [ ] v1.0：命令和配置稳定

## FAQ

**问：Claude 会漏掉重要信息吗？**
答：在所有测试日志中，每一条不同的错误和警告都被保留了。如果摘要不够，最后一行给出了完整日志的路径，Claude 可以按范围读取。

**问：squeeze 会把我的数据发到别处吗？**
答：不会。它没有任何网络代码，日志只留在你机器上的 `.squeeze/` 中。

**问：会弄坏我的 Claude Code 会话吗？**
答：钩子在失败时会退让（fail open）：输入异常、文件不存在或内部出错时都不输出任何内容，Claude Code 照常继续。

**问：普通的 claude.ai 聊天也能省 token 吗？**
答：只有在先运行 `squeeze clip` 再粘贴长文本时才行。v0.2 会在浏览器里把这一步自动化。

**问：为什么是 0.x 版本？**
答：主版本号为 0 时，命令名和配置可能还会变化。v1.0 意味着它们已经稳定。

## 仓库结构

```
claude-squeeze/
├── README.md                    ← 项目概览（英文，默认）
├── readme/                      ← 译文：韩语、日语、中文、西班牙语、法语
├── v0.1_squeeze-sentinel/       ← v0.1：CLI + Claude Code 钩子（Rust，无依赖）
│   ├── README.md                   v0.1 完整文档（6 种语言）
│   ├── src/
│   ├── docs/                       基准测试方法、用户指南（韩语）
│   └── testdata/fixtures/          测试和基准测试使用的日志
├── install.sh / install.ps1
├── changelog/                   ← CHANGELOG（6 种语言）
└── Cargo.toml                   ← 工作区
```

## 参与贡献

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release -- bench v0.1_squeeze-sentinel/testdata/fixtures
```

欢迎提交新的测试日志：把真实的 `*.log`（去掉敏感信息和个人路径）放进 `v0.1_squeeze-sentinel/testdata/fixtures/`。只要丢失任何一条不同的错误或警告，测试就会失败。问题和想法：[GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues)。

## 许可证

MIT，见 [LICENSE](../LICENSE)。squeeze 是非官方的社区工具，与 Anthropic 无关。
