# 📌 Issue 提交指南

[English](ISSUE_GUIDE.md) | [한국어](ISSUE_GUIDE.ko.md) | [日本語](ISSUE_GUIDE.ja.md) | **简体中文** | [Español](ISSUE_GUIDE.es.md) | [Français](ISSUE_GUIDE.fr.md)

你好，感谢你的到来！👋
claude-squeeze 之所以不断变好，靠的是大家反馈的问题和愿望。哪怕只有一行的反馈也非常有帮助。本页介绍如何写出能被快速解决的 Issue。

### 可以在这里提交什么

- 🐞 **出故障了**：squeeze 崩溃、输出异常，或 Claude Code 会话行为奇怪
- ✂️ **压缩不对劲**：重要的行不见了、保留太多，或命令被误拦／漏拦
- 📦 **安装、更新或版本问题**：安装脚本失败、发布文件缺失，或更新后出现变化
- 💡 **想法和改进**：新功能、更好的默认值，以及你希望 **v1.0** 包含的内容
- ❓ **问题**：不确定是不是 bug？尽管问，没人会介意。

### 提交之前（只需 2 分钟）

1. **确认版本**：运行 `squeeze version`。如果不是[最新版本](https://github.com/jongwooish/claude-squeeze/releases/latest)，请先更新，问题可能已经修复。
2. **搜索已有 Issue**：也许已经有人报告过。如果有，请点 👍 或在评论中补充你的情况，而不是新建。
3. **浏览一下 [FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/readme/README.zh-CN.md#faq)**：常见问题在那里已有解答。

### 选择合适的表单

点击 **New issue** 然后选择：

| 如果… | 选择 |
|---|---|
| squeeze 崩溃、输出错误或会话出问题 | **Bug report / 버그 신고** |
| 重要的行被丢弃，或命令被误拦／漏拦 | **Compression problem / 압축 문제** |
| 安装、更新或某个版本有问题 | **Install / update / version problem / 설치·업데이트·버전 문제** |
| 有想法或对 v1.0 的期望 | **Feature idea / 기능 제안** |

拿不准就选最接近的，必要时我们会帮你调整。

### 好的反馈应包含

- **你的环境**：squeeze 版本、操作系统、shell（Windows 上请说明是 Git Bash、WSL 还是 PowerShell）
- 你或 Claude 执行的**确切命令**
- **期望的结果**和**实际发生的情况**
- **输出请用文本**，不要只给截图（截图作为补充很好）
- 尽量简短的**复现步骤**

收集信息时常用的命令：

```bash
squeeze version            # 你的版本
squeeze report --json      # 最近 squeeze 挡在上下文之外的内容
SQUEEZE_SKIP=1 <command>   # 原始输出（与摘要对比）
```

squeeze 压缩输出后，最后一行会显示完整日志的保存位置（`.squeeze/raw/...`）。如果是压缩问题，把那个文件（按下文清理后）附上会非常有帮助。

### 🔒 请保护好你的敏感信息

日志里可能有你不想公开的内容。粘贴前请删除或替换：

- API 密钥、令牌、密码、Cookie
- 内部主机名、IP 地址、公司名称
- 像 `C:\Users\你的名字\...` 或 `/home/你的名字/...` 这样的个人路径（替换为 `~/`）

发现了**安全问题**？请不要公开发布，按照 [SECURITY.md](../SECURITY.md) 私下报告。

### 提交之后

- 我们会阅读每一个 Issue 并加上标签（`bug`、`enhancement`、`question` 等）。
- 这是个人开源项目，回复可能需要几天，感谢你的耐心。
- 我们可能会追问一些细节，及时回答往往是最快的修复途径。
- 修复和新功能会写进 [CHANGELOG](../../changelog/CHANGELOG.zh-CN.md)，并在 [Releases](https://github.com/jongwooish/claude-squeeze/releases) 中公布。点击 **Watch → Custom → Releases** 即可收到通知。

### 几条友好约定

- 一个 Issue 只说一个问题，方便逐个跟踪和关闭。
- 友善交流，这里的每个人都想帮忙。
- 六种语言任选。用英语或韩语能得到最快的回复。

感谢你帮助 claude-squeeze 变得更好！🙏
