# v0.2.0 squeeze-Auditor

[English](../README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **フィードバック歓迎。** 改善案、v1.0 リリース、バージョン更新、インストールエラーなど、どんな問題でも [Issue を作成してください](https://github.com/jongwooish/claude-squeeze/issues/new/choose)（6 言語いずれも可）。

← [squeeze に戻る](../../../README/README.ja.md)

v0.1 Sentinel は長い出力を 1 つずつ止めていました。**v0.2 Auditor はセッション全体を見ます。** Claude がすでに何を読んだかを覚え、会話がどれだけ大きくなったかを見張り、文字を打つ前からメッセージごとに一緒に送られるものを点検します。そして、サイズだけでは見えない 4 種類のムダを止めます。

インストール型のプログラムです。セットアップを実行し、スタートメニュー（またはアプリケーション）から **squeeze-Auditor** を開くと、トークンがどこに使われたか、squeeze が何を止めたかを確認し、どれだけ強く節約するかを変えられます。

**動く場所:** Claude Code（すべてのプロジェクト）と Claude デスクトップアプリ（Windows、macOS）。ブラウザの claude.ai とモバイルアプリは v0.3 で扱います。
**必要なもの:** なし。v0.2 は v0.1 と同じ `squeeze` プログラムで、v0.1 の機能をすべて含み、下の機能が加わりました。[バージョン同士の関係](../../../README/README.ja.md#バージョン同士の関係)を参照。

![squeeze-Auditor ダッシュボード](../docs/images/auditor-en-dashboard.png)

## 新機能: 4 つのブロック

| | ブロック | 何を捕まえるか | どう止めるか |
|---|---|---|---|
| 1 | **重複読み込み** | 変わっていないファイルを Claude がまた丸ごと読もうとする | `Read` を拒否し「もう読んだので、その内容を使って」と伝えます。ファイルが変わるか、会話が圧縮されるとすぐ再び許可します。 |
| 2 | **予算で一時停止** | すでにとても大きくなったセッションにメッセージを送り続ける | 送信前に一度止めて `/compact` か `/clear` を勧めます。もう一度送る（↑、Enter）とそのまま送られます。セッションがさらに 50% 大きくなるまで再び聞きません。 |
| 3 | **引き継ぎ（ハンドオフ）** | 長いセッションが毎回過去の会話全体を読み直す | セッションが引き継ぎサイズを超えると、Claude が止まる前に短い引き継ぎメモを書きます。`/clear` すると、新しいセッションは会話全体ではなくメモ（約 1,000 トークン）から始まります。 |
| 4 | **固定費** | *すべての*メッセージに付いてくるもの: `CLAUDE.md` とインポート、スキルとエージェントの説明、MCP サーバーのツール一覧 | squeeze-Auditor が 1 つずつ測ります。使っていない MCP サーバーはワンクリックでオフ（`/mcp` と同じ Claude Code 自身のスイッチ）。任意: プロジェクトで 14 日間使われていないサーバーを自動でオフ。 |

### v0.1 との違い

| | v0.1 Sentinel | v0.2 Auditor |
|---|---|---|
| 見るもの | 出力・ファイル **1 つ**のサイズ | **セッション**: 以前読んだもの、今のサイズ、毎回付いてくるもの |
| 判断基準 | 「この出力は長いか?」 | 記憶（同じファイル、未変更）、累積（セッションサイズ）、履歴（一度も使われないサーバー） |
| フック | `PreToolUse`（Bash、Read） | さらに `UserPromptSubmit`、`Stop`、`SessionStart`、`PreCompact` |
| インストール | プロジェクトごとに `squeeze init` | インストーラーか `squeeze setup`、**全**プロジェクトに 1 回 |
| 確認 | ターミナルの `squeeze report` | squeeze-Auditor ウィンドウ |

v0.1 の機能はそのまま動きます: 出力の多いコマンドは `squeeze run` へ、大きなファイルは目次へ。

## squeeze-Auditor ウィンドウ

| タブ | できること |
|---|---|
| **ダッシュボード** | Claude Code 自身のセッションログから読んだ実際の使用量（今日、7 日、30 日）と、squeeze が防いだ量。現在の 5 時間枠と消費ペース。種類別のブロック。最近のセッションのサイズと予算比。 |
| **節約モード** | 強め、バランス（既定）、軽め、それぞれの数値。またはカスタム。 |
| **設定** | すべての値: いつ出力を縮めるか、何行残すか、大きなファイルの基準、予算、引き継ぎのサイズと方式、固定費のルール、決して触らないコマンド。 |
| **固定費** | プロジェクトごと（またはデスクトップアプリ）: メモリファイル、インポート、スキル、エージェント、MCP サーバーごとのメッセージあたりトークン。MCP サーバーの計測、オン・オフ。小さくするヒント。 |
| **引き継ぎ** | すべての引き継ぎメモをその場で読める。 |
| **接続** | Claude Code と Claude デスクトップアプリの接続・解除。デスクトップアプリが読めるフォルダの指定。 |

ウィンドウは squeeze が `127.0.0.1` だけで表示する小さなページで、起動ごとに変わるキーが必要です。Edge か Chrome のアプリウィンドウ（なければ既定のブラウザ）で表示し、閉じると少し後に自動で終了します。外部には何も送りません。英語と韓国語に対応。

![固定費タブ](../docs/images/auditor-en-fixed.png)

## 効果

**出力圧縮**（v0.1 から変更なし）: 実際のログ 7 件、推定 83,325 → 5,128 トークン（**−93.8%**）、異なるエラー・警告はすべて残る。方法: [docs/bench.md](../docs/bench.md)。

**4 つのブロック**は別の方法で節約します。同じトークンが何度も送られるのを止めます。効果は使い方しだいなので、ベンチマークではなく計算で示します。

| ブロック | 例 | 防ぐ量 |
|---|---|---|
| 重複読み込み | 変わっていない 600 行のファイル（約 7,000 トークン）を再読込 | 約 7,000 トークン、さらにセッションの以降のメッセージごとに同じ量 |
| 予算停止 / 引き継ぎ | 150,000 トークンのセッションでさらに 20 回 vs 1,000 トークンのメモから新しいセッション | 約 3,000,000 入力トークンの再送 vs 約 20,000 + 新しい作業 |
| 固定費 | ツール 12 個の使われない MCP サーバー（テストサーバーの計測値 1,882 トークン）、メッセージ 100 回 | 約 188,000 トークン |

**正直な限界。**
- トークン数は推定（バイト ÷ 4）です。長いセッションの大部分は Claude のプロンプトキャッシュから読み直され、キャッシュは新しい入力より安いので、実際のコスト削減はトークン数より小さくなります。それでもコンテキストウィンドウを埋め、使用量に数えられます。
- ダッシュボードの使用量はこのコンピューターの Claude Code セッションログから来ています。claude.ai やデスクトップアプリ自体の会話は含まれず、プランの公式な上限でもありません。5 時間枠は Claude Code の数え方をまねた推定です。
- squeeze はデスクトップアプリの会話を見られません。デスクトップアプリでは、Claude が squeeze のツールでログを読むとき、または引き継ぎの保存を頼んだときだけ節約します。

## インストール

[最新リリース](https://github.com/jongwooish/claude-squeeze/releases/latest)からダウンロード:

| システム | ファイル | 次に |
|---|---|---|
| Windows 10/11（ほとんどの PC） | `squeeze-Auditor-0.2.0-setup-x64.exe` | 実行。管理者権限不要。スタートメニュー → **squeeze-Auditor** |
| Windows on ARM | `squeeze-Auditor-0.2.0-setup-arm64.exe` | 同じ |
| macOS、Apple シリコン | `squeeze-Auditor-0.2.0-macos-arm64.pkg` | 開く。アプリケーション → **squeeze-Auditor** |
| macOS、Intel | `squeeze-Auditor-0.2.0-macos-x86_64.pkg` | 同じ |
| Linux（Debian、Ubuntu） | `squeeze-auditor_0.2.0_amd64.deb` / `_arm64.deb` | `sudo apt install ./squeeze-auditor_0.2.0_amd64.deb`、その後アプリメニュー |

**インストーラーはまだコード署名されていない**ため、Windows と macOS が最初にブロックすることがあります。[システムにブロックされたとき](#システムにブロックされたとき)を見てください。

ターミナルでのインストール（両プログラム、スタートメニューのアイコンなし）:

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# ソースから（Rust 1.80+）
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

### システムにブロックされたとき

リリースファイルはまだコード署名されていないため、Windows と macOS は作成者を確認できません。ファイルはこのリポジトリから GitHub Actions がビルドしています。警告を越える前に、リリースページの `checksums.txt` と一致するか確認してください。

```bash
# macOS
shasum -a 256 -c checksums.txt --ignore-missing
# Linux
sha256sum -c checksums.txt --ignore-missing
# Windows (PowerShell): リリースページのハッシュと比べる
Get-FileHash .\squeeze-Auditor-0.2.0-setup-x64.exe -Algorithm SHA256
```

**Windows**
- **SmartScreen**（「Windows によって PC が保護されました」）: **詳細情報** → **実行**。
- **スマート アプリ コントロール**（「スマート アプリ コントロールにより、安全でない可能性のあるアプリがブロックされました」）: それでも実行するボタンはなく、すべてのプログラムを検査するため、インストール後の `squeeze.exe` と `squeeze-auditor.exe` もブロックされます。仮想マシンか Windows サンドボックスで試すか、オフにしてください: Windows セキュリティ → アプリとブラウザー コントロール → スマート アプリ コントロールの設定 → **オフ**。Windows 11 25H2 以降は後でオンに戻せます。それより前のバージョンでは、Windows を再インストールするまでオフのままです。
- **ブラウザーで落とした zip**: 展開する前に右クリック → プロパティ → **許可する**、または `Unblock-File .\squeeze-*.zip`。
- **ウイルス対策ソフト**: 誤検知としてベンダーに報告するか、`%LOCALAPPDATA%\Programs\squeeze` を除外に追加してください。

**macOS**
- **「開けません」/「開発元を確認できない」**: システム設定 → プライバシーとセキュリティ → 下へスクロール → ファイル名の横の **このまま開く** → パスワードを入力。macOS 15 以降では、右クリック → 開く でこの確認を飛ばせなくなりました。
- **「壊れているため開けません」**: ダウンロードの印が残っています。`xattr -dr com.apple.quarantine /Applications/squeeze-Auditor.app` を実行してください。
- **`.tar.gz` のターミナル用プログラム**: `xattr -d com.apple.quarantine squeeze squeeze-auditor`。

**Linux**
- `.deb` は `sudo apt install ./squeeze-auditor_0.2.0_amd64.deb` で入れてください。一部のソフトウェアセンターはディストリビューション外のパッケージとして警告しますが、ターミナルの apt は警告しません。
- `.tar.gz` のプログラムで **「Permission denied」**: `chmod +x squeeze squeeze-auditor`。フォルダーが `noexec` でマウントされている場合（一部の `/tmp` や USB）、`~/.local/bin` に移してください。
- ほかのディストリビューション（Fedora、Arch など）は `.tar.gz` か上のターミナルインストールを使ってください。

## クイックスタート

1. インストールして **squeeze-Auditor** を開きます。（Windows のインストーラーは最後に Claude Code への接続まで行えます。チェックはそのままに。）
2. **Claude Code に接続** をクリック。全プロジェクト用のフックを `~/.claude/settings.json` に追加します。他の設定とフックは残し、変更前にバックアップします。
3. Claude デスクトップアプリを使うなら、**接続** タブで **接続** をクリックし、Claude が読めるログフォルダを追加して、デスクトップアプリを再起動します。
4. いつも通り Claude Code を使います。ダッシュボードで何を防いだか確認しましょう。

ターミナルでは: `squeeze setup`（接続）、`squeeze setup --remove`（解除）、`squeeze auditor`（ウィンドウを開く）。

**v0.1 からのアップグレード:** 上からインストールし（Windows のインストーラーは v0.1 の `install.ps1` と同じフォルダを使います）、一度接続してください。`squeeze init` したプロジェクトもそのまま動きます。そのフックは残しても、`squeeze uninstall` で消しても構いません。Windows では squeeze-Auditor をアンインストールすると Claude Code とデスクトップアプリへの接続も解除されます。macOS と Linux では先に `squeeze setup --remove` を実行してください。

## セッション中に起きること

```
SessionStart ──▶ 最近の引き継ぎメモがある?       ──はい──▶ 新しいセッションに読み込む
     │
メッセージを入力
     │
UserPromptSubmit ──▶ セッション ≥ 予算(150K)?    ──はい──▶ 一度止める:「/compact か /clear、またはもう一度送信」
     │
Claude が作業
     ├─ Bash ──▶ 出力の多いコマンド?             ──はい──▶「squeeze run -- … で実行」          (v0.1)
     └─ Read ──▶ 大きなファイルを丸ごと?         ──はい──▶ 目次 + 範囲読み込み                 (v0.1)
                 前に丸ごと読んだ未変更のファイル? ──はい──▶「読んだ内容を使って」           (v0.2)
     │
Stop ──▶ セッション ≥ 引き継ぎサイズ(120K)?     ──はい──▶ Claude が .squeeze/handoff/<時刻>.md を書き
     │                                                      /clear するよう案内
PreCompact ──▶ 読んだファイルの記録を消す（Claude はもう持っていないため）
```

すべてのフックはエラー時に何も出力せず終了し、Claude Code は普段通り続きます。squeeze がツールの実行を代わりに承認することはありません。

## Claude デスクトップアプリ

接続すると `claude_desktop_config.json` に 1 項目が追加されます: ツール 6 個を持つ MCP サーバー `squeeze mcp`。

| ツール | すること |
|---|---|
| `read_log` | ログ・テキストファイルを縮めた状態で（または正確な行範囲で）読む |
| `tail_log` | ログの末尾を縮めて |
| `search_log` | 一致する行と前後少し |
| `list_logs` | 許可されたフォルダのファイル一覧 |
| `save_handoff` / `load_handoff` | 今の会話を引き継ぎメモとして保存し、新しい会話で読み込む |

Claude が読めるのは **接続** に書いたフォルダだけです。5,000 行のログを貼り付ける代わりに「アプリのログの最後のエラーを読んで」と頼めば、Claude は短い版を受け取ります。会話が長くなったら引き継ぎの保存を頼み、新しい会話で読み込むよう頼みましょう。

## 節約モード

| | 強め | バランス（既定） | 軽め |
|---|---:|---:|---:|
| この行数までは出力そのまま | 40 行 | 80 行 | 150 行 |
| 大きなファイルの基準 | 250 行 | 400 行 | 800 行 |
| 予算で一時停止 | 100K トークン | 150K トークン | 200K トークン |
| 引き継ぎ開始 | 80K トークン | 120K トークン | 160K トークン |

## 設定

全体の設定は `~/.squeeze/config.json` にあります（設定タブが書き込みます）。プロジェクトの `.squeeze/config.json` でどのキーも上書きできます。v0.1 のキーもすべて有効です（[一覧](../../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.ja.md#設定)）。v0.2 の新しいキー:

| キー | 既定値 | 意味 |
|---|---|---|
| `profile` | `"balanced"` | `strong`、`balanced`、`light`、`custom` |
| `dup_read` | `true` | 未変更ファイルの丸ごと再読込をブロック |
| `budget.enabled` / `budget.tokens` | `true` / `150000` | 予算で一時停止とそのサイズ |
| `handoff.enabled` / `handoff.tokens` | `true` / `120000` | 引き継ぎとそのサイズ |
| `handoff.mode` | `"note"` | `note`（メモを書き、`/clear` 後に読み込む）か `remind`（`/compact` を勧めるだけ） |
| `handoff.max_age_hours` | `48` | これより古いメモは読み込まない |
| `mcp_prune.enabled` / `unused_days` | `false` / `14` | プロジェクトで使われない MCP サーバーをオフ |
| `desktop.allowed_dirs` | `[]` | デスクトップアプリが読めるフォルダ |

メッセージのどこかに `SQUEEZE_SKIP` を入れると予算の一時停止を通過します。

## コマンド

| コマンド | すること |
|---|---|
| `squeeze auditor` | squeeze-Auditor ウィンドウを開く |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Claude Code（全プロジェクト）とデスクトップアプリの接続・解除 |
| `squeeze doctor [--dir D] [--desktop] [--json]` | メッセージあたりの固定費をターミナルで |
| `squeeze mcp` | デスクトップアプリが起動する MCP サーバー |
| `squeeze init`、`run`、`compress`、`clip`、`report`、`bench`、`uninstall` | [v0.1](../../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.ja.md#使い方) と同じ |

## プライバシーと安全

- **このコンピューター内だけ。** ウィンドウの `127.0.0.1` ページ以外にネットワークコードはなく、そのページも起動ごとに変わるキーが必要です。
- **バックアップ。** `~/.claude/settings.json`、`~/.claude.json`、`claude_desktop_config.json` を変更する前に、元のファイルを `~/.squeeze/backups/` にコピーします。
- **できる限り読み取りのみ。** 数値のために Claude Code のセッションログを読みますが変更はせず、認証情報は決して読みません。
- **MCP サーバーの計測**は、Claude Code と同じようにそのサーバーのコマンドを数秒実行してツール一覧を尋ねます。

## FAQ

**Q. 予算の一時停止は邪魔になりませんか?**
A. 段階ごとに一度だけ止めます。同じメッセージをもう一度送ればそのまま送られます。設定で予算を上げる、軽めモードにする、またはオフにできます。

**Q. 引き継ぎで文脈を失いませんか?**
A. 細部の一部は意図的に捨てます。メモには目標、済んだこと、決定、触ったファイル、次の手順が残ります。メモは `.squeeze/handoff/` の普通のファイルなので、`/clear` の前に読んだり直したりできます。自分で判断したいなら、引き継ぎ方式を「知らせるだけ」にしてください。

**Q. なぜブラウザとスマホは対象外?**
A. ブラウザのタブやスマホアプリでは、Claude の横でインストール型プログラムを動かせません。v0.3 で別の方法を取ります。

**Q. ダッシュボードの数値はプランの上限ですか?**
A. いいえ。このコンピューターの Claude Code のログが示す数値です。他のプログラムがプランの使用量を読む公式な方法を Anthropic は提供していません。

**Q. v0.1 に戻せますか?**
A. はい。`squeeze setup --remove` の後、v0.1.0 をインストールしてください（インストールスクリプトに `SQUEEZE_VERSION=v0.1.0`）。

## フィードバック

改善案や問題は [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) へ: 止めるべきでないときに止めた、インストールの問題、おかしな数値、v0.3 と v1.0 のアイデア。質問は [Discussions](https://github.com/jongwooish/claude-squeeze/discussions) へ。6 言語いずれも歓迎です。

## コントリビュート

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
cargo run --manifest-path CARGO/Cargo.toml -- auditor     # ソースからウィンドウを開く
```

コードの地図: `src/guards.rs`（重複読み込み、予算、引き継ぎ）、`src/mcp.rs` と `src/doctor.rs`（固定費）、`src/transcripts.rs` と `src/usage.rs`（ダッシュボードの数値）、`src/auditor.rs` と `ui/index.html`（ウィンドウ）、`src/mcp_server.rs`（デスクトップアプリのツール）、`packaging/`（インストーラー）。韓国語のユーザーガイド: [docs/guide](../docs/guide/index.md)。
