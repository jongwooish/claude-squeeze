# 📌 Issue guide · 이슈 작성 가이드

**English** · [한국어](#한국어) · [日本語](#日本語) · [简体中文](#简体中文) · [Español](#español) · [Français](#français)

<details open>
<summary><b>English</b></summary>

## English

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
SQUEEZE_SKIP=1 <command>   # the raw output, to compare with the summary
```

When squeeze compresses output, the last line shows where the full log was saved (`.squeeze/raw/...`). Attaching that file (after cleaning it, see below) helps a lot with compression problems.

### 🔒 Please protect your secrets

Logs can contain things you do not want to publish. Before pasting, remove or replace:

- API keys, tokens, passwords, cookies
- Internal hostnames, IP addresses, company names
- Personal paths like `C:\Users\your-name\...` or `/home/your-name/...` (replace with `~/`)

Found a **security problem**? Please do not post it publicly. Follow [SECURITY.md](SECURITY.md) to report it privately.

### What happens next

- We read every issue and add a label (`bug`, `enhancement`, `question`, …).
- This is a personal open-source project, so a reply can take a few days. Thank you for your patience.
- We may ask a follow-up question. Answering it is often the fastest way to a fix.
- Fixes and new features are listed in the [CHANGELOG](../changelog/CHANGELOG.md) and announced in [Releases](https://github.com/jongwooish/claude-squeeze/releases). Click **Watch → Custom → Releases** to get notified.

### A few kind rules

- One problem per issue, so each one can be tracked and closed.
- Be friendly. Everyone here is trying to help.
- Any of the six languages is fine. English or Korean gets the fastest reply.

Thank you for helping make claude-squeeze better! 🙏

</details>

<details>
<summary><b>한국어</b></summary>

## 한국어

안녕하세요, 찾아와 주셔서 고마워요! 👋
claude-squeeze는 여러분이 알려주는 문제점과 바라는 점 덕분에 좋아집니다. 한 줄짜리 제보도 큰 도움이 돼요. 이 페이지는 빨리 해결되는 이슈를 쓰는 방법을 알려드립니다.

### 이런 걸 남길 수 있어요

- 🐞 **뭔가 고장났어요**: squeeze가 멈추거나, 이상한 걸 출력하거나, Claude Code 세션이 이상하게 동작할 때
- ✂️ **압축이 이상해요**: 중요한 줄이 사라졌거나, 너무 많이 남았거나, 명령이 잘못 잡히거나 안 잡힐 때
- 📦 **설치·업데이트·버전 문제**: 설치 스크립트가 실패했거나, 릴리스 파일이 없거나, 업데이트 후 뭔가 달라졌을 때
- 💡 **아이디어와 개선점**: 새 기능, 더 나은 기본값, **v1.0**에 들어갔으면 하는 것
- ❓ **질문**: 버그인지 잘 모르겠어도 편하게 물어보세요. 아무도 뭐라 안 해요.

### 올리기 전에 (2분이면 돼요)

1. **버전 확인**: `squeeze version`을 실행해보세요. [최신 릴리스](https://github.com/jongwooish/claude-squeeze/releases/latest)가 아니라면 먼저 업데이트해 주세요. 이미 고쳐졌을 수도 있어요.
2. **기존 이슈 검색**: 누군가 이미 올렸을 수 있어요. 있다면 새로 만들지 말고 👍나 자신의 상황을 댓글로 달아주세요.
3. **[FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/readme/README.ko.md#faq) 훑어보기**: 자주 묻는 질문은 거기 답이 있어요.

### 알맞은 양식 고르기

**New issue**를 누르고 골라주세요:

| 이럴 때 | 이 양식 |
|---|---|
| squeeze가 멈추거나, 출력이 틀리거나, 세션이 망가졌을 때 | **Bug report / 버그 신고** |
| 중요한 줄이 빠졌거나, 명령이 잘못 잡히거나 안 잡혔을 때 | **Compression problem / 압축 문제** |
| 설치, 업데이트, 특정 버전이 문제일 때 | **Install / update / version problem / 설치·업데이트·버전 문제** |
| 아이디어나 v1.0에 바라는 점이 있을 때 | **Feature idea / 기능 제안** |

잘 모르겠으면 가장 비슷한 걸 고르세요. 필요하면 저희가 옮길게요.

### 좋은 제보에 들어가면 좋은 것

- **환경**: squeeze 버전, 운영체제, 셸 (윈도우라면 Git Bash, WSL, PowerShell 중 무엇인지)
- 본인이나 Claude가 실행한 **정확한 명령어**
- **기대한 결과**와 **실제로 일어난 일**
- **출력은 스크린샷 말고 텍스트로** (스크린샷은 추가로 붙이면 좋아요)
- 가능한 한 짧은 **재현 방법**

정보를 모을 때 쓰기 좋은 명령어:

```bash
squeeze version            # 내 버전
squeeze report --json      # 최근에 squeeze가 컨텍스트에서 막아낸 것
SQUEEZE_SKIP=1 <명령>       # 원본 출력 (요약과 비교할 때)
```

squeeze가 출력을 압축하면 마지막 줄에 전체 로그가 저장된 위치(`.squeeze/raw/...`)가 나와요. 압축 문제라면 그 파일을 (아래처럼 정리한 뒤) 첨부해 주시면 정말 큰 도움이 됩니다.

### 🔒 비밀 정보는 꼭 지켜주세요

로그에는 공개하고 싶지 않은 정보가 들어 있을 수 있어요. 붙여넣기 전에 지우거나 바꿔주세요:

- API 키, 토큰, 비밀번호, 쿠키
- 내부 호스트명, IP 주소, 회사 이름
- `C:\Users\내이름\...`이나 `/home/내이름/...` 같은 개인 경로 (`~/`로 바꿔주세요)

**보안 문제**를 발견했다면 공개로 올리지 말고 [SECURITY.md](SECURITY.md)의 방법대로 비공개로 알려주세요.

### 올린 다음에는

- 모든 이슈를 읽고 라벨(`bug`, `enhancement`, `question` 등)을 붙여요.
- 개인 오픈소스 프로젝트라 답변에 며칠 걸릴 수 있어요. 기다려 주셔서 고마워요.
- 추가로 질문을 드릴 수 있어요. 답해주시는 게 가장 빨리 고치는 길인 경우가 많아요.
- 수정 사항과 새 기능은 [CHANGELOG](../changelog/CHANGELOG.ko.md)에 적고 [Releases](https://github.com/jongwooish/claude-squeeze/releases)에서 알려드려요. **Watch → Custom → Releases**를 누르면 알림을 받을 수 있어요.

### 몇 가지 부탁

- 이슈 하나에 문제 하나만. 그래야 하나씩 추적하고 닫을 수 있어요.
- 친절하게 대화해요. 여기 있는 모두가 돕고 싶어서 왔어요.
- 6개 언어 중 아무거나 괜찮아요. 영어나 한국어로 쓰면 가장 빨리 답할 수 있어요.

claude-squeeze를 더 좋게 만들어 주셔서 고마워요! 🙏

</details>

<details>
<summary><b>日本語</b></summary>

## 日本語

こんにちは、来てくれてありがとうございます！ 👋
claude-squeeze は、皆さんが教えてくれる問題点や要望のおかげで良くなっていきます。一行だけの報告でも大歓迎です。このページでは、早く解決される Issue の書き方を紹介します。

### ここに書けること

- 🐞 **何かが壊れた**: squeeze が止まった、おかしな出力をした、Claude Code のセッションが変な動きをした
- ✂️ **圧縮がおかしい**: 重要な行が消えた、残しすぎた、コマンドが誤って誘導された／されなかった
- 📦 **インストール・更新・バージョンの問題**: インストーラーが失敗した、リリースファイルがない、更新後に何かが変わった
- 💡 **アイデアと改善**: 新機能、より良い既定値、**v1.0** に入れてほしいもの
- ❓ **質問**: バグか分からなくても気軽にどうぞ。誰も気にしません。

### 投稿する前に（2 分で終わります）

1. **バージョンを確認**: `squeeze version` を実行してください。[最新リリース](https://github.com/jongwooish/claude-squeeze/releases/latest)でなければ、まず更新を。すでに直っているかもしれません。
2. **既存の Issue を検索**: 誰かがもう報告しているかもしれません。その場合は新しく作らず、👍 や状況のコメントを付けてください。
3. **[FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/readme/README.ja.md#faq) をざっと見る**: よくある質問はそこに答えがあります。

### 合うフォームを選ぶ

**New issue** を押して選んでください:

| こんなとき | このフォーム |
|---|---|
| squeeze が止まった、出力が間違っている、セッションが壊れた | **Bug report / 버그 신고** |
| 重要な行が消えた、コマンドの誤検出・見逃し | **Compression problem / 압축 문제** |
| インストール、更新、特定のバージョンが問題 | **Install / update / version problem / 설치·업데이트·버전 문제** |
| アイデアや v1.0 への要望 | **Feature idea / 기능 제안** |

迷ったら一番近いものを選んでください。必要ならこちらで移動します。

### 良い報告に含めてほしいこと

- **環境**: squeeze のバージョン、OS、シェル（Windows なら Git Bash、WSL、PowerShell のどれか）
- あなたや Claude が実行した**正確なコマンド**
- **期待した結果**と**実際に起きたこと**
- **出力はスクリーンショットではなくテキストで**（スクリーンショットは追加ならぜひ）
- できるだけ短い**再現手順**

情報集めに便利なコマンド:

```bash
squeeze version            # バージョン
squeeze report --json      # 最近 squeeze がコンテキストから締め出したもの
SQUEEZE_SKIP=1 <command>   # 元の出力（要約と比べるとき）
```

squeeze が出力を圧縮すると、最後の行に完全なログの保存先（`.squeeze/raw/...`）が表示されます。圧縮の問題なら、そのファイルを（下記のように整理してから）添付してもらえると大変助かります。

### 🔒 秘密情報は必ず守ってください

ログには公開したくない情報が含まれていることがあります。貼る前に削除または置き換えてください:

- API キー、トークン、パスワード、Cookie
- 内部ホスト名、IP アドレス、会社名
- `C:\Users\あなたの名前\...` や `/home/あなたの名前/...` のような個人的なパス（`~/` に置き換え）

**セキュリティの問題**を見つけたら、公開せずに [SECURITY.md](SECURITY.md) の方法で非公開で知らせてください。

### 投稿したあとは

- すべての Issue を読んでラベル（`bug`、`enhancement`、`question` など）を付けます。
- 個人のオープンソースプロジェクトなので、返信まで数日かかることがあります。ご了承ください。
- 追加で質問することがあります。答えてもらうのが一番早い解決につながることがよくあります。
- 修正や新機能は [CHANGELOG](../changelog/CHANGELOG.ja.md) に記載し、[Releases](https://github.com/jongwooish/claude-squeeze/releases) でお知らせします。**Watch → Custom → Releases** で通知を受け取れます。

### いくつかのお願い

- 1 つの Issue に 1 つの問題。そうすれば一つずつ追跡して閉じられます。
- 親切に話しましょう。ここにいる全員が助けたいと思っています。
- 6 言語のどれでも構いません。英語か韓国語だと一番早く返信できます。

claude-squeeze をより良くしてくれてありがとうございます！ 🙏

</details>

<details>
<summary><b>简体中文</b></summary>

## 简体中文

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

发现了**安全问题**？请不要公开发布，按照 [SECURITY.md](SECURITY.md) 私下报告。

### 提交之后

- 我们会阅读每一个 Issue 并加上标签（`bug`、`enhancement`、`question` 等）。
- 这是个人开源项目，回复可能需要几天，感谢你的耐心。
- 我们可能会追问一些细节，及时回答往往是最快的修复途径。
- 修复和新功能会写进 [CHANGELOG](../changelog/CHANGELOG.zh-CN.md)，并在 [Releases](https://github.com/jongwooish/claude-squeeze/releases) 中公布。点击 **Watch → Custom → Releases** 即可收到通知。

### 几条友好约定

- 一个 Issue 只说一个问题，方便逐个跟踪和关闭。
- 友善交流，这里的每个人都想帮忙。
- 六种语言任选。用英语或韩语能得到最快的回复。

感谢你帮助 claude-squeeze 变得更好！🙏

</details>

<details>
<summary><b>Español</b></summary>

## Español

¡Hola, y gracias por pasarte! 👋
claude-squeeze mejora gracias a que la gente nos cuenta qué falló y qué le gustaría. Cada aporte cuenta, aunque sea de una sola línea. Esta página explica cómo escribir un issue que se resuelva rápido.

### Qué puedes publicar aquí

- 🐞 **Algo se rompió**: squeeze falló, mostró algo raro o una sesión de Claude Code se comportó de forma extraña
- ✂️ **La compresión no fue bien**: desapareció una línea importante, se conservó demasiado o un comando se redirigió (o no)
- 📦 **Problemas de instalación, actualización o versión**: falló el instalador, faltaba un archivo de la versión o algo cambió tras actualizar
- 💡 **Ideas y mejoras**: funciones nuevas, mejores valores por defecto y lo que te gustaría ver en la **v1.0**
- ❓ **Preguntas**: ¿no sabes si es un error? Pregunta igualmente. A nadie le molestará.

### Antes de publicar (2 minutos)

1. **Comprueba tu versión**: ejecuta `squeeze version`. Si no es la [última versión](https://github.com/jongwooish/claude-squeeze/releases/latest), actualiza primero. Puede que ya esté arreglado.
2. **Busca issues existentes**: quizá alguien ya lo informó. Si es así, añade un 👍 o un comentario con tus detalles en lugar de abrir uno nuevo.
3. **Echa un vistazo a las [FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/readme/README.es.md#faq)**: allí están las preguntas comunes.

### Elige el formulario adecuado

Pulsa **New issue** y elige:

| Si… | Elige |
|---|---|
| squeeze falló, dio una salida incorrecta o rompió una sesión | **Bug report / 버그 신고** |
| se perdió una línea importante o un comando se detectó (o no) | **Compression problem / 압축 문제** |
| el problema es instalar, actualizar o una versión concreta | **Install / update / version problem / 설치·업데이트·버전 문제** |
| tienes una idea o un deseo para la v1.0 | **Feature idea / 기능 제안** |

¿Dudas? Elige el más parecido. Lo moveremos si hace falta.

### Qué hace que un informe sea genial

- **Tu entorno**: versión de squeeze, sistema operativo y shell (en Windows: Git Bash, WSL o PowerShell)
- **El comando exacto** que ejecutaste tú o Claude
- **Lo que esperabas** y **lo que pasó realmente**
- **La salida como texto**, no como captura (las capturas son bienvenidas como extra)
- **Pasos para reproducirlo**, lo más cortos posible

Comandos útiles para reunir información:

```bash
squeeze version            # tu versión
squeeze report --json      # lo que squeeze dejó fuera del contexto recientemente
SQUEEZE_SKIP=1 <comando>   # la salida original, para compararla con el resumen
```

Cuando squeeze comprime una salida, la última línea indica dónde se guardó el log completo (`.squeeze/raw/...`). Adjuntar ese archivo (tras limpiarlo, ver abajo) ayuda muchísimo con los problemas de compresión.

### 🔒 Protege tus secretos

Los logs pueden contener cosas que no quieres publicar. Antes de pegar, quita o reemplaza:

- Claves de API, tokens, contraseñas, cookies
- Nombres de host internos, direcciones IP, nombres de empresa
- Rutas personales como `C:\Users\tu-nombre\...` o `/home/tu-nombre/...` (cámbialas por `~/`)

¿Encontraste un **problema de seguridad**? No lo publiques. Sigue [SECURITY.md](SECURITY.md) para informarlo en privado.

### Qué pasa después

- Leemos cada issue y le ponemos una etiqueta (`bug`, `enhancement`, `question`, …).
- Es un proyecto personal de código abierto, así que la respuesta puede tardar unos días. Gracias por tu paciencia.
- Puede que te hagamos una pregunta. Responderla suele ser el camino más rápido hacia la solución.
- Las correcciones y novedades se anotan en el [CHANGELOG](../changelog/CHANGELOG.es.md) y se anuncian en [Releases](https://github.com/jongwooish/claude-squeeze/releases). Pulsa **Watch → Custom → Releases** para recibir avisos.

### Unas pocas normas amables

- Un problema por issue, para poder seguir y cerrar cada uno.
- Sé amable. Todos aquí quieren ayudar.
- Cualquiera de los seis idiomas sirve. En inglés o coreano la respuesta es más rápida.

¡Gracias por ayudar a mejorar claude-squeeze! 🙏

</details>

<details>
<summary><b>Français</b></summary>

## Français

Bonjour, et merci de votre visite ! 👋
claude-squeeze s'améliore grâce aux personnes qui nous disent ce qui n'a pas marché et ce qu'elles aimeraient. Chaque retour compte, même d'une seule ligne. Cette page explique comment écrire une issue qui sera corrigée rapidement.

### Ce que vous pouvez publier ici

- 🐞 **Quelque chose ne marche pas** : squeeze a planté, affiché quelque chose d'étrange, ou une session Claude Code s'est comportée bizarrement
- ✂️ **La compression semblait fausse** : une ligne importante a disparu, trop de lignes ont été gardées, ou une commande a (ou n'a pas) été redirigée
- 📦 **Problème d'installation, de mise à jour ou de version** : l'installateur a échoué, un fichier de release manquait, ou quelque chose a changé après une mise à jour
- 💡 **Idées et améliorations** : nouvelles fonctionnalités, meilleurs réglages par défaut, et ce que vous aimeriez voir dans la **v1.0**
- ❓ **Questions** : pas sûr que ce soit un bug ? Demandez quand même, personne ne s'en offusquera.

### Avant de publier (2 minutes)

1. **Vérifiez votre version** : lancez `squeeze version`. Si ce n'est pas la [dernière release](https://github.com/jongwooish/claude-squeeze/releases/latest), mettez à jour d'abord. Le problème est peut-être déjà corrigé.
2. **Cherchez dans les issues existantes** : quelqu'un l'a peut-être déjà signalé. Dans ce cas, ajoutez un 👍 ou un commentaire avec vos détails plutôt qu'une nouvelle issue.
3. **Parcourez la [FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/readme/README.fr.md#faq)** : les questions courantes y sont traitées.

### Choisissez le bon formulaire

Cliquez sur **New issue** et choisissez :

| Si… | Choisissez |
|---|---|
| squeeze a planté, donné une sortie fausse ou cassé une session | **Bug report / 버그 신고** |
| une ligne importante a disparu, ou une commande a (ou n'a pas) été interceptée | **Compression problem / 압축 문제** |
| le problème vient de l'installation, d'une mise à jour ou d'une version précise | **Install / update / version problem / 설치·업데이트·버전 문제** |
| vous avez une idée ou un souhait pour la v1.0 | **Feature idea / 기능 제안** |

Un doute ? Choisissez le plus proche, nous le déplacerons si besoin.

### Ce qui fait un excellent rapport

- **Votre environnement** : version de squeeze, système d'exploitation et shell (sous Windows : Git Bash, WSL ou PowerShell)
- **La commande exacte** lancée par vous ou par Claude
- **Ce que vous attendiez** et **ce qui s'est réellement passé**
- **La sortie en texte**, pas en capture d'écran (les captures sont bienvenues en plus)
- **Les étapes pour reproduire**, aussi courtes que possible

Commandes pratiques pour rassembler les informations :

```bash
squeeze version            # votre version
squeeze report --json      # ce que squeeze a récemment gardé hors du contexte
SQUEEZE_SKIP=1 <commande>  # la sortie brute, pour la comparer au résumé
```

Quand squeeze compresse une sortie, la dernière ligne indique où le log complet a été enregistré (`.squeeze/raw/...`). Joindre ce fichier (après l'avoir nettoyé, voir ci-dessous) aide énormément pour les problèmes de compression.

### 🔒 Protégez vos secrets

Les logs peuvent contenir des choses à ne pas publier. Avant de coller, retirez ou remplacez :

- Clés d'API, jetons, mots de passe, cookies
- Noms d'hôtes internes, adresses IP, noms d'entreprise
- Chemins personnels comme `C:\Users\votre-nom\...` ou `/home/votre-nom/...` (remplacez par `~/`)

Vous avez trouvé un **problème de sécurité** ? Ne le publiez pas. Suivez [SECURITY.md](SECURITY.md) pour le signaler en privé.

### Et ensuite ?

- Nous lisons chaque issue et lui ajoutons une étiquette (`bug`, `enhancement`, `question`, …).
- C'est un projet open source personnel : une réponse peut prendre quelques jours. Merci de votre patience.
- Nous pourrons vous poser une question complémentaire. Y répondre est souvent le chemin le plus court vers une correction.
- Les corrections et nouveautés sont listées dans le [CHANGELOG](../changelog/CHANGELOG.fr.md) et annoncées dans les [Releases](https://github.com/jongwooish/claude-squeeze/releases). Cliquez sur **Watch → Custom → Releases** pour être notifié.

### Quelques règles bienveillantes

- Un problème par issue, pour pouvoir suivre et fermer chacune.
- Restez aimable : tout le monde ici essaie d'aider.
- N'importe laquelle des six langues convient. L'anglais ou le coréen obtiennent la réponse la plus rapide.

Merci d'aider à améliorer claude-squeeze ! 🙏

</details>
