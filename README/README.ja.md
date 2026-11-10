# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#インストール) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **フィードバック歓迎。** 改善点、v1.0 のリリース、バージョン更新、インストールエラーなど、どんな問題でも [Issue を作成してください](https://github.com/jongwooish/claude-squeeze/issues/new/choose)（6 言語対応）。

**Claude のトークンは、ログの洪水ではなく思考に使おう。**

squeeze は、長いコマンド出力、大きなファイルや重複したファイル読み込み、大きくなりすぎたセッション、使われていない MCP サーバーを Claude のコンテキストに入れないようにします。Claude はエラーと警告をそのまま見られます。インストール型アプリ **squeeze-Auditor** で、トークンがどこに使われたかを確認し、squeeze の強さを選べます。

- [効果](#効果) · [バージョン](#バージョン) · [どこで効くか](#どこで効くか) · [v0.2 squeeze-Auditor](#v02-squeeze-auditor) · [v0.1 Sentinel](#v01-sentinel) · [インストール](#インストール) · [使い方](#使い方) · [仕組み](#仕組み) · [ロードマップ](#ロードマップ) · [FAQ](#faq)

## なぜ必要か

Claude Code はリクエストのたびに会話全体を送り直し、コマンド出力も会話の一部です。`npm install`、テスト実行、`journalctl` を一度実行するだけで数千トークンが増え、それ以降のすべてのターンについて回ります。Claude に必要なのは「成功した」の一行やエラー三行だけなのに。

squeeze はその出力がコンテキストに入る**前に**止めます。

## 効果

実際のログ 7 件を squeeze に通した結果です（[測定方法](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/docs/bench.md)）:

| ログ | 行数 前 → 後 | トークン 前 → 後（推定） | 削減 | エラー・警告メッセージの保持 |
|---|---:|---:|---:|---:|
| `pytest` 実行、失敗 2 件 | 217 → 39 | 4,040 → 516 | **87.2%** | 11 / 11 |
| `go test -v`、失敗 1 件 | 488 → 16 | 4,247 → 202 | **95.2%** | 4 / 4 |
| `make`、gcc 警告 + エラー 1 件 | 370 → 44 | 7,046 → 850 | **87.9%** | 7 / 7 |
| `tsc`、型エラー 2 件 | 124 → 9 | 1,310 → 172 | **86.9%** | 3 / 3 |
| `dpkg -l` パッケージ一覧 | 1049 → 63 | 35,237 → 2,106 | **94.0%** | 8 / 8 |
| サーバージャーナル、OOM kill（合成データ） | 1400 → 37 | 31,161 → 997 | **96.8%** | 5 / 5 |
| 短い `npm` 出力 | 16 → 16 | 287 → 287 | 0.0%（そのまま） | 12 / 12 |
| **合計** | | **83,325 → 5,128** | **93.8%** | **50 / 50** |

**何がうれしいか**

- **このテストでは重要な情報を何も失っていません。** 異なるエラー・警告メッセージはすべて残りました。繰り返しは一度だけ表示して件数を付けます（`… [same message 29 more times] …`）。
- **短い出力には一切手を付けません。** 80 行・8 KB 以下なら原文のままです。
- **削減は毎ターン効きます。** 31,000 トークンのログが次の 20 ターンの間コンテキストに残ると、入力は約 620,000 トークンです。約 1,000 トークンに縮めれば同じ 20 ターンで約 20,000 です。
- **上限に届くのが遅くなります。** ノイズが減るとコンテキストがゆっくり埋まるので、会話の圧縮やプランの上限に達するのが遅くなります。
- **Claude が集中しやすくなります。** 「なぜビルドが失敗した?」の答えが 1,000 行に埋もれず、目の前にあります。

**正直な限界:** トークン数は推定値（バイト ÷ 4）で、この数値は保存したログを再生したものであり、Claude Code を実際にオン・オフして比較した結果ではありません（実運用の比較は予定）。自分のプロジェクトで `squeeze report` を実行すると実際の数値がわかります。

**v0.2 は 4 つのブロックを追加します。** 同じトークンが何度も送られるのを止めます: 変わっていないファイルの再読込、予算を超えたセッション、短い引き継ぎメモからやり直すべき長いセッション、使われない MCP サーバーのような固定費。効果は使い方しだいなので、[v0.2 README](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.ja.md#効果) に計算を、squeeze-Auditor のダッシュボードにあなた自身の数値を示します。

## バージョン

バージョンごとにフォルダと README があります。リリースはリポジトリ全体に付くタグです。

| バージョン | 名前 | 追加されるもの | 状態 |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.ja.md) | **Sentinel** | 出力の多いコマンドと丸ごと読み込みを捕まえる Claude Code フック、`squeeze run`、`compress`、`clip`、`report` | ✅ v0.1.0 |
| [v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.ja.md) | **squeeze-Auditor** | セッションの 4 つのブロック（重複読み込み、予算で一時停止、引き継ぎ、固定費）、使用量ダッシュボードと節約モードを備えたインストール型アプリ、Claude デスクトップアプリ対応 | ✅ v0.2.0 |
| v0.3 | *（名前は未定）* | ブラウザの claude.ai とモバイルアプリ、トークンの不満への新しいアプローチ | 🛠 予定 |

### バージョン同士の関係

| バージョン | インストールするもの | 以前のバージョンが必要? |
|---|---|---|
| v0.1 Sentinel | `squeeze` プログラム | — |
| v0.2 squeeze-Auditor | 同じ `squeeze` プログラム + インストーラー、スタートメニューのアプリ（`squeeze-auditor`）、新しいフック | **いいえ、v0.1 を含みます。** v0.1 の上にも、単体でもインストールできます。 |
| v0.3 | 予定 | 後日お知らせ |

- **プログラムは累積です。** 新しい `squeeze` には以前の機能がすべて入っています。v0.1 と v0.2 を並べてインストールすることはありません。
- **フォルダは各バージョンが何を加えたかの説明です。** フォルダをインストールするわけではありません。
- **設定は引き継がれます。** v0.1 の `squeeze init` による `.squeeze/config.json` とフックは更新後も動きます。v0.2 は全体設定 `~/.squeeze/config.json` を追加し、Claude Code に全プロジェクト用として一度接続します。

| 使っているもの | インストール |
|---|---|
| Claude Code | squeeze-Auditor（v0.2）→ **Claude Code に接続** |
| Claude デスクトップアプリ | squeeze-Auditor（v0.2）→ **デスクトップアプリに接続** |
| ブラウザの claude.ai、モバイルアプリ | 今は長い貼り付けに `squeeze clip`、のちに v0.3 |

## どこで効くか

| どこで | 効果 | 方法 |
|---|---|---|
| **Claude Code** | 自動、全プロジェクト | squeeze-Auditor（または `squeeze setup`）が `~/.claude/settings.json` にフックを追加。v0.2 の 4 つのブロックと v0.1 の 2 つのガードがすべて動きます。 |
| **Claude デスクトップアプリ**（Windows、macOS） | Claude がファイルを読むとき、引き継ぎを保存するとき | 接続すると `squeeze mcp` のツールが追加されます。Claude は許可したフォルダのログだけを縮めた状態で読み、引き継ぎメモを保存・読み込みできます。会話そのものを squeeze が見たり変えたりはできません。 |
| **claude.ai（ウェブ、モバイル）** | 手動、長いテキストを貼るとき | ログをコピーして `squeeze clip`、貼り付け。v0.3 で扱う予定。 |
| **その他のターミナルツール** | 手動 | `<コマンド> 2>&1 \| squeeze compress` |

Pro プランでは Claude Code と claude.ai が同じ利用上限を共有するので、Claude Code で節約したトークンはチャットにも残ります。

## v0.2 squeeze-Auditor

v0.1 は出力を 1 つずつ見ていました。**v0.2 はセッション全体を見ます**: Claude がすでに何を読んだか、会話がどれだけ大きくなったか、メッセージごとに何が付いてくるか。4 種類のムダを止めます。

| ブロック | 何を捕まえるか | どう止めるか |
|---|---|---|
| **重複読み込み** | 変わっていないファイルの丸ごと再読込 | `Read` を拒否:「もう持っている」。ファイルが変わるか圧縮後は再び許可。 |
| **予算で一時停止** | とても大きなセッション（既定 150K トークン）にメッセージを送り続ける | 一度止めて `/compact` か `/clear` を勧める。もう一度送ればそのまま送信。 |
| **引き継ぎ** | 過去の会話全体を毎回読み直す長いセッション（既定 120K） | Claude が短い引き継ぎメモを書き、`/clear` 後の新しいセッションは会話全体ではなくメモから始まる。 |
| **固定費** | メッセージごとに送られる `CLAUDE.md`、スキル、エージェント、MCP ツール一覧 | 1 つずつ測り、使わない MCP サーバーはワンクリックでオフ（Claude Code 自身のスイッチ）。14 日使われなければ自動でオフも可能。 |

**squeeze-Auditor** はスタートメニューやアプリケーションから開くアプリです: 実際の Claude Code 使用量と squeeze が防いだ量のダッシュボード、節約モード（強め、バランス、軽め）、すべての設定、固定費の診断、引き継ぎメモ、Claude Code とデスクトップアプリへの接続。

![squeeze-Auditor ダッシュボード](../v0.2-Auditor/v0.2.0_squeeze-auditor/docs/images/auditor-en-dashboard.png)

v0.2 の完全なドキュメント: [v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.ja.md](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.ja.md)。

## v0.1 Sentinel

Sentinel は Claude Code とシェルの間に立つ見張り番です。単一バイナリ、Rust、**依存関係ゼロ**、macOS / Linux / Windows 対応。

- **Bash ガード**: 出力の多いコマンド（`npm install`、`go test`、`cargo build`、`pytest`、`make`、`apt-get install`、`journalctl`、`docker build`、`kubectl logs`、`git log` など）を `squeeze run -- <command>` で実行し直させます。すでに出力が制限されたコマンド（`| tail -20`、`-n 50`、`> file`、`--quiet`）には触れません。
- **Read ガード**: 400 行を超えるファイルを `offset`/`limit` なしで読もうとすると迂回します。Claude は行番号付きの目次を受け取り、必要な範囲だけ読みます。
- **賢い圧縮**: 最初の 8 行と最後の 20 行、すべてのエラー・警告と前後 2 行、スタックトレースを残します。繰り返しは件数でまとめ、警告よりエラーを優先します。
- **終了コードを保持**: `squeeze run` は元のコマンドの終了コードを返します。
- **完全なログはディスクに**: `.squeeze/raw/`、最新 50 件を保持、本人のみ読み取り可。`.squeeze/` は `.gitignore` に追加されます。
- claude.ai 用の **`squeeze clip`**、実際の節約量を見る **`squeeze report`**。
- **安全な設計**: フックは決して代わりに承認せず、エラー時は静かに引き下がり（fail open）、`squeeze uninstall` で追加したものをすべて消します。ネットワークコードはありません。

`squeeze run -- pytest -v` の後に Claude が見るもの（実際の出力、217 行 → 39 行）:

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
… (2 つのトレースバックは全文保持) …
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

v0.1 の詳しいドキュメント: [v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.ja.md](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.ja.md)。

## インストール

**インストーラー（おすすめ）。** [最新リリース](https://github.com/jongwooish/claude-squeeze/releases/latest)から:

| システム | ファイル |
|---|---|
| Windows 10/11 | `squeeze-Auditor-0.2.0-setup-x64.exe`（ARM: `-setup-arm64.exe`） |
| macOS | `squeeze-Auditor-0.2.0-macos-arm64.pkg`（Intel: `-macos-x86_64.pkg`） |
| Linux（Debian、Ubuntu） | `squeeze-auditor_0.2.0_amd64.deb`（ARM: `_arm64.deb`） |

インストーラーはまだコード署名されていないため、Windows と macOS がブロックすることがあります。SmartScreen、スマート アプリ コントロール、Gatekeeper、Linux の実行権限: [システムにブロックされたとき](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.ja.md#システムにブロックされたとき)。

**ターミナル:**

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# ソースから（Rust 1.80+ がある任意の OS）
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

リリースのすべてのファイルは `checksums.txt`（SHA-256）に載っており、インストールスクリプトが確認します。`SQUEEZE_VERSION=v0.1.0` でバージョン固定、`SQUEEZE_INSTALL_DIR` でフォルダ変更。

## クイックスタート

1. **squeeze-Auditor** を開き、**Claude Code に接続** をクリック（全プロジェクト）。デスクトップアプリを使うなら **接続** タブで接続して再起動。
2. いつも通り Claude Code を使います。
3. ダッシュボードで何を防いだか確認。強さは **節約モード** か **設定** で変えます。

ターミナルでは:

```bash
squeeze setup      # Claude Code に接続（デスクトップアプリがあれば一緒に）
claude             # いつも通り Claude Code を使う
squeeze auditor    # ウィンドウを開く
squeeze setup --remove
```

接続で変わるのは `~/.claude/settings.json`（デスクトップアプリは `claude_desktop_config.json`）だけです。他の設定とフックは残し、変更前の元ファイルを `~/.squeeze/backups/` にバックアップします。v0.1 のプロジェクト単位の `squeeze init` もそのまま使えます。

## 使い方

| コマンド | すること |
|---|---|
| `squeeze auditor` | squeeze-Auditor ウィンドウを開く |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Claude Code（全プロジェクト）とデスクトップアプリの接続・解除 |
| `squeeze doctor [--dir D] [--desktop] [--json]` | メッセージごとに送られるもの（固定費） |
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | 1 つのプロジェクトだけにフックを入れる（v0.1 方式） |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | プロジェクトのフックと注記を削除。`--purge` は `.squeeze/` も削除 |
| `squeeze run -- <コマンド>` | コマンドを実行。出力が長ければ短い要約 |
| `<コマンド> 2>&1 \| squeeze compress` | パイプ出力を圧縮 |
| `squeeze clip [--print]` | クリップボードを圧縮（claude.ai 用） |
| `squeeze report [--since 7d] [--global] [--json]` | 節約量を表示 |
| `squeeze bench <dir> [--markdown]` | `*.log` を再生して測定 |
| `squeeze mcp` | デスクトップアプリが起動する MCP サーバー |
| `squeeze version` | バージョンを表示 |

コマンドの前に `SQUEEZE_SKIP=1` を付けると一度だけ生の出力を、メッセージに `SQUEEZE_SKIP` を入れると予算の一時停止を通過します。設定: [v0.2 の設定](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.ja.md#設定)。

## 仕組み

```
Claude が実行しようとする:  npm test
        │
        ▼
PreToolUse フック ── squeeze hook pre-bash
        │  出力が多い? ── いいえ ──▶ 何もしない。通常の権限確認へ
        │  はい
        ▼
拒否 + ヒント: 「squeeze run -- 'npm test' で実行して」
        │
        ▼
Claude が実行: squeeze run -- 'npm test'   （通常の権限ルールがそのまま適用）
        │
        ├─▶ 完全な出力 ── .squeeze/raw/<時刻>.log
        └─▶ 短い出力 ──── Claude（エラー、警告、先頭、末尾、ログのパス）
```

なぜコマンドをこっそり書き換えず「拒否 + ヒント」なのか? コマンドを書き換えるフックはそれを承認もしなければならず、ユーザーの権限確認を飛ばしてしまいます。squeeze は決して代わりに承認しません。

### ラッパーとの違い

[chaude](https://github.com/int-ttt/chaude) のようなツールは Claude Code CLI を独自の画面で包み、セッションの切り替えや安いモデルへの委任でトークンを節約します。squeeze は問題の反対側を担当します:

- **ラッパーではありません。** Claude Code 自身のフックで動くので、`claude` をそのまま使えます。
- **満杯のコンテキストを片付けるのではなく、ノイズを入り口で止めます。**
- **両方を併用できます。**

## ロードマップ

- [x] **v0.1 Sentinel**: Claude Code フック、`run`、`compress`、`clip`、`report`、`bench`
- [x] **v0.2 squeeze-Auditor**: 重複読み込み・予算・引き継ぎ・固定費のブロック、ダッシュボードと節約モードを備えたインストール型アプリ、Claude デスクトップアプリ用ツール
- [ ] **v0.3**: ブラウザの claude.ai とモバイルアプリ、よく聞かれるトークンの問題への新しいアプローチ
- [ ] コード署名付きインストーラー
- [ ] Claude Code の実地 A/B ベンチマーク（同じ作業で squeeze のオン・オフ）
- [ ] v1.0: コマンドと設定の安定化

## FAQ

**Q. Claude が大事なものを見落としませんか?**
A. すべてのテストログで、異なるエラー・警告メッセージはすべて残りました。要約で足りなければ最後の行に完全なログのパスがあります。重複読み込みを止めるときも、本当に必要なら読み直す方法を Claude に伝えます。

**Q. squeeze はデータをどこかに送りますか?**
A. いいえ。ネットワークコードはありません。squeeze-Auditor のウィンドウは自分のコンピューターだけが開ける `127.0.0.1` のページです。

**Q. Claude Code のセッションを壊しませんか?**
A. フックは失敗しても安全側に倒れます: 不正な入力、存在しないファイル、内部エラーでも何も出力せず、Claude Code は普段通り続きます。squeeze がツールの実行を代わりに承認することはありません。

**Q. 普通の claude.ai のチャットでもトークンは減りますか?**
A. `squeeze clip` で長いテキストを貼るときだけです。ブラウザとモバイルアプリは v0.3 で扱います。

**Q. ダッシュボードの数値はプランの上限ですか?**
A. いいえ。このコンピューターの Claude Code のログを読んだ数値です。他のプログラムがプランの使用量を読む公式な方法はありません。

**Q. なぜ 0.x なのですか?**
A. メジャーバージョンが 0 の間は、コマンド名や設定が変わる可能性があります。v1.0 は安定したという意味です。

## リポジトリ構成

```
claude-squeeze/
├── README/                      ← 6 言語の概要（README.md = 英語）
├── CHANGELOG/                   ← 6 言語の CHANGELOG
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   バージョン概要
│   └── v0.1.0_squeeze-sentinel/   ← リリースされた v0.1.0 のまま（凍結）
├── v0.2-Auditor/                ← v0.2 squeeze-Auditor
│   ├── README.md                   バージョン概要
│   └── v0.2.0_squeeze-auditor/    ← v0.2: 現在の squeeze プログラム（Rust、依存なし）
│       ├── README.md               v0.2 の完全なドキュメント（英語）
│       ├── README/                 翻訳（5 言語）
│       ├── src/                    エンジン、フック、ブロック、squeeze-Auditor サーバー
│       ├── ui/                     squeeze-Auditor ウィンドウ（HTML ファイル 1 つ）
│       ├── packaging/              Windows・macOS・Linux のインストーラー、アイコン
│       ├── docs/                   ベンチマーク、ユーザーガイド（韓国語）、スクリーンショット
│       └── testdata/fixtures/      テストとベンチマーク用のログ
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock（Rust ワークスペース）
├── .github/                     ← Issue フォーム、CI、リリース、トップページの README
├── CLAUDE.md
└── LICENSE
```

## コントリビュート

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
```

新しいテストログを歓迎します: 実際の `*.log`（秘密情報と個人のパスは削除）を `v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures/` に追加してください。異なるエラー・警告が 1 つでも失われるとテストが失敗します。バグやアイデア: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues)。

## コミュニティ

**問題を見つけたり、アイデアがあれば [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) に書いてください。** 改善点、v1.0 のリリース、バージョン更新、インストールエラーなど、どんなソフトウェアの問題でも次のバージョンづくりに役立ちます。


| こんなとき | 行き先 |
|---|---|
| バグやセッションが壊れる問題 | [Issues → Bug report](https://github.com/jongwooish/claude-squeeze/issues/new?template=bug_report.yml) |
| 重要な行が消えた、節約が少ない、コマンドの誤検出・見逃し | [Issues → Compression problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=compression.yml) |
| インストール・更新・バージョンの問題 | [Issues → Install, update or version problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=version_problem.yml) |
| 機能の提案 | [Issues → Feature idea](https://github.com/jongwooish/claude-squeeze/issues/new?template=feature_request.yml) |
| 質問、トラブルシューティング | [Discussions → Q&A](https://github.com/jongwooish/claude-squeeze/discussions/categories/q-a) |
| ロードマップ、v1.0 に入れるべきことへの意見 | [Discussions → Ideas](https://github.com/jongwooish/claude-squeeze/discussions/categories/ideas) |
| 新バージョンのお知らせ | [Discussions → Announcements](https://github.com/jongwooish/claude-squeeze/discussions/categories/announcements) と [Releases](https://github.com/jongwooish/claude-squeeze/releases)（**Watch → Custom → Releases** で通知を受け取れます） |

6 言語のどれで書いても構いません。セキュリティ問題は [SECURITY.md](../.github/SECURITY.md)、貢献方法は [CONTRIBUTING.md](../.github/CONTRIBUTING.md) を参照してください。

## ライセンス

MIT。[LICENSE](../LICENSE) を参照。squeeze は非公式のコミュニティツールであり、Anthropic とは関係ありません。
