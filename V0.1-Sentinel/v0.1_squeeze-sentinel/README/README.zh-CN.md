# v0.1 Sentinel

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文** | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **欢迎反馈。** 改进建议、v1.0 发布、版本更新、安装错误或任何其他问题，请[提交 Issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose)（支持 6 种语言）。

← [返回 squeeze](../../../readme/README.zh-CN.md)

Sentinel 是守在 Claude Code 和 shell 之间的哨兵。当 Claude 准备运行会塞满上下文的命令（安装、构建、测试、日志），或一次性读取巨大的文件时，Sentinel 会把它引导到简短的版本。Claude 依然能拿到每一条错误和警告。

单个可执行文件，Rust 编写，**零依赖**，支持 macOS、Linux 和 Windows。

**需要什么：** 无。这就是基础的 `squeeze` 程序。后续版本都建立在它之上：v0.3 Conduit 在同一个程序里新增命令（更新即可获得），v0.2 Interceptor 是一个独立的浏览器扩展，有没有它都能用。参见[各版本之间的关系](../../../readme/README.zh-CN.md#各版本之间的关系)。

## 效果

回放 7 份测试日志，估算 token 从 83,325 降到 5,128（**−93.8%**），每一条不同的错误和警告都被保留。详情和方法：[docs/bench.md](../docs/bench.md)（英文）。

运行 `squeeze run -- pytest -v` 后 Claude 看到的内容（217 行 → 39 行），这是 `pytest-fail.log` 的真实输出：

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

## 功能

- **Bash 守卫**：高输出命令（`npm install`、`go test`、`cargo build`、`pytest`、`make`、`apt-get install`、`journalctl`、`docker build`、`kubectl logs`、`git log` 等）会被引导到 `squeeze run -- <command>`。
- **Read 守卫**：不带 `offset`/`limit` 读取超过 400 行的文件时会被引导。Claude 拿到带行号的大纲后选择范围。
- **智能压缩**：保留前 8 行和后 20 行、每条错误和警告及其上下文、堆栈跟踪；重复消息合并计数；优先保留错误。
- **保留退出码**：`squeeze run` 以原命令的退出码结束。
- **完整日志存在磁盘上**：`.squeeze/raw/`，保留最新 50 份，仅本人可读（0600）。`.squeeze/` 会加入 `.gitignore`。
- 面向 claude.ai 的 **`squeeze clip`**：在把日志粘贴进聊天前压缩剪贴板。
- **`squeeze report`**：本会进入上下文的量、实际进入的量，以及节省最多的项目。
- **安全设计**：钩子绝不批准任何工具调用，出错时安静退出（fail open），`squeeze uninstall` 会移除它添加的一切。

## 安装

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 | iex
# 从源码
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

安装脚本会在安装前校验发布中的 `checksums.txt`。用 `SQUEEZE_VERSION=v0.1.0` 固定版本，用 `SQUEEZE_INSTALL_DIR` 更改目录。

## 快速开始

```bash
cd ~/my-project
squeeze init
claude
```

`squeeze init` 会做四处改动，`squeeze uninstall` 可以撤销：

| 文件 | 改动 |
|---|---|
| `.claude/settings.json` | 添加两个 `PreToolUse` 钩子（`Bash`、`Read`）。保留其他设置和钩子，先写入 `.squeeze.bak` 备份。 |
| `.squeeze/config.json` | 默认配置（见下）。 |
| `.gitignore` | 添加 `.squeeze/`，因为原始日志可能包含敏感信息。 |
| `CLAUDE.md` | 在 `<!-- squeeze:start -->` 标记之间添加 4 行说明。 |

## 用法

| 命令 | 作用 |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | 安装到项目。如果 `squeeze` 不在 `PATH` 中，用 `--bin` 指定绝对路径。 |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | 移除钩子和 CLAUDE.md 说明。`--purge` 同时删除 `.squeeze/`。 |
| `squeeze run -- <command>` | 运行命令；输出过长时打印简短摘要。 |
| `<command> 2>&1 \| squeeze compress` | 压缩管道输出。 |
| `squeeze clip [--print]` | 用压缩版替换剪贴板（`--print` 只显示）。 |
| `squeeze report [--since 7d] [--global] [--json] [--top N]` | 本项目的节省量。`--global` 显示 `squeeze clip` 的节省量。 |
| `squeeze bench <dir> [--markdown]` | 回放 `*.log` 文件并测量节省量。 |
| `squeeze hook pre-bash` / `pre-read` | Claude Code 调用的钩子入口。 |
| `squeeze version` | 显示版本。 |

如果某条命令确实需要原始输出，Claude（或你）可以在前面加上 `SQUEEZE_SKIP=1`。

## 配置

`.squeeze/config.json`（所有键均可选）：

| 键 | 默认值 | 含义 |
|---|---|---|
| `compress.max_lines` / `max_bytes` | `80` / `8000` | 同时不超过两者的输出保持原样。 |
| `compress.head_lines` / `tail_lines` | `8` / `20` | 开头 / 结尾始终保留的行数。 |
| `compress.context` | `2` | 每条错误或警告前后保留的行数。 |
| `compress.budget` | `120` | 保留行数的大致上限。 |
| `guard_bash` / `guard_read` | `true` / `true` | 打开或关闭各个守卫。 |
| `read_max_lines` | `400` | 超过此行数的文件会被引导为按范围读取。 |
| `extra_noisy` | `[]` | 视为高输出的命令前缀，例如 `"./deploy.sh"`。 |
| `quiet` | `[]` | 守卫永不处理的命令前缀，例如 `"make docs"`。 |
| `keep_raw` | `50` | `.squeeze/raw/` 中保留的原始日志数量。 |

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
        ▼
完整输出 ──▶ .squeeze/raw/<时间>.log
简短输出 ──▶ Claude（错误、警告、开头、结尾，以及带日志路径的最后一行）
```

为什么是“拒绝 + 提示”而不是悄悄改写命令？改写命令的钩子还必须批准该命令，这会跳过你的权限确认。squeeze 从不这样做。

更多：[docs/guide/how-it-works.md](../docs/guide/how-it-works.md)（韩语）。

## 适用范围

- **Claude Code**：通过钩子自动生效。
- **claude.ai**：仅在粘贴长文本前使用 `squeeze clip` 时有效。普通聊天不受影响；squeeze 无法看到或修改你的聊天。
- **其他终端工具**：通过管道传给 `squeeze compress`。

## FAQ

**问：Claude 会漏掉重要信息吗？**
答：在所有测试日志中，每一条不同的错误和警告都被保留了。如果摘要不够，最后一行给出完整日志的路径，Claude 可以按范围读取。

**问：squeeze 会把我的数据发到别处吗？**
答：不会。它没有网络代码，日志留在你机器上的 `.squeeze/` 中。

**问：会弄坏会话吗？**
答：钩子在失败时会退让。输入异常、文件不存在或发生 panic 时都不输出任何内容，Claude Code 照常继续。

**问：Claude 读大文件总被拦住，但我需要整个文件。**
答：提示里会告诉 Claude 用 `limit=<行数>` 重新读取，这样就能通过。或者设置 `"guard_read": false` 关闭它。

**问：普通的 claude.ai 聊天也能省 token 吗？**
答：只有在先运行 `squeeze clip` 再粘贴长文本时才行。

## 反馈

请把改进建议和问题留在 [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose)：对 v1.0 的期望、版本更新或安装错误、压缩时丢失了内容，以及其他任何软件问题都欢迎。选择一个表单（bug、压缩问题、安装/更新/版本问题、功能建议）即可。提问和路线图讨论请到 [Discussions](https://github.com/jongwooish/claude-squeeze/discussions)。六种语言任选。

## 参与贡献

```bash
cargo test                       # 单元测试（包括测试日志检查）
cargo clippy -- -D warnings
cargo run -- bench V0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures
```

欢迎提交新的测试日志：把真实的 `*.log`（先去掉敏感信息和个人路径）放进 `testdata/fixtures/`。只要丢失任何一条不同的错误或警告，测试就会失败。

问题和想法：[GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues)。
