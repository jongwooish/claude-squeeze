# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](v0.1_squeeze-sentinel/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](v0.1_squeeze-sentinel/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#インストール) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

**Claude のトークンは、ログの洪水ではなく思考に使おう。**

squeeze は、長いコマンド出力や大きなファイルの丸ごと読み込みを Claude のコンテキストに入れないようにします。Claude はエラーと警告、最初と最後の行はそのまま見られます。完全なログは必要なときのためにディスクに保存されます。

- [効果](#効果) · [バージョン](#バージョン) · [どこで効くか](#どこで効くか) · [v0.1 Sentinel](#v01-sentinel) · [インストール](#インストール) · [使い方](#使い方) · [仕組み](#仕組み) · [ロードマップ](#ロードマップ) · [FAQ](#faq)

## なぜ必要か

Claude Code はリクエストのたびに会話全体を送り直し、コマンド出力も会話の一部です。`npm install`、テスト実行、`journalctl` を一度実行するだけで数千トークンが増え、それ以降のすべてのターンについて回ります。Claude に必要なのは「成功した」の一行やエラー三行だけなのに。

squeeze はその出力がコンテキストに入る**前に**止めます。

## 効果

実際のログ 7 件を squeeze に通した結果です（[測定方法](v0.1_squeeze-sentinel/docs/bench.md)）:

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

## バージョン

バージョンごとにフォルダと README があります。リリースはリポジトリ全体へのタグです。

| バージョン | 名前 | 追加されるもの | 状態 |
|---|---|---|---|
| [v0.1](v0.1_squeeze-sentinel/README.ja.md) | **Sentinel** | 出力の多いコマンドと大きなファイルの丸読みを捕まえる Claude Code フック、`squeeze run`、`compress`、`clip`、`report` | ✅ v0.1.0 |
| v0.2 | **Interceptor** | claude.ai に長いテキストを貼り付けたとき縮めるか尋ねるブラウザ拡張 | 🛠 予定 |
| v0.3 | **Conduit** | Claude デスクトップアプリ用 MCP サーバー: Claude がログを縮めた状態で読む | 🛠 予定 |

### バージョン同士の関係

インストールできるものは 2 つです: **`squeeze` プログラム**（CLI）と、v0.2 からの**ブラウザ拡張**。

| バージョン | 何をインストールするか | v0.1 は必要? |
|---|---|---|
| v0.1 Sentinel | `squeeze` プログラム | — |
| v0.2 Interceptor | ブラウザ拡張 | **不要。** 圧縮エンジンを内蔵しているので単体で動きます。Claude Code も使うなら v0.1 も入れてください。 |
| v0.3 Conduit | 新しく入れるものはなし: `squeeze` プログラム内のコマンド（`squeeze mcp`） | **必要。** `squeeze` を v0.3 以上に更新するだけです。別のダウンロードはありません。 |

- **プログラムは累積的です。** 新しい `squeeze` には以前の機能がすべて含まれます。v0.1 と v0.3 を並べて入れることはありません。
- **フォルダは各バージョンで追加された内容の説明です。** フォルダをインストールするわけではありません。
- **設定は引き継がれます。** `.squeeze/config.json` と `squeeze init` で入れたフックは更新後もそのまま動きます。変更があるバージョンでは CHANGELOG に書き、`squeeze init` が更新します。

| 使っているもの | インストールするもの |
|---|---|
| Claude Code だけ | `squeeze`（現在は v0.1） |
| claude.ai だけ | 今は `squeeze clip`、公開後は v0.2 拡張 |
| Claude デスクトップアプリで Claude に直接ログを読ませたい | 公開後の `squeeze` v0.3 |
| 全部 | `squeeze`（最新）+ v0.2 拡張 |

## どこで効くか

| 場所 | 効果 | 方法 |
|---|---|---|
| **Claude Code** | 自動 | `squeeze init` がフックを入れます。出力の多いコマンドと大きなファイルの読み込みが迂回されます。 |
| **claude.ai（Web、デスクトップ、モバイル）** | 長いテキストを貼るときだけ、手動 | ログをコピー → `squeeze clip` → 貼り付け。普段のチャットは変わりません。squeeze はチャットを見られません。 |
| **ほかのターミナルツール** | 手動 | `<command> 2>&1 \| squeeze compress` |

Pro プランでは Claude Code と claude.ai が同じ利用上限を共有するので、Claude Code で節約したトークンはチャットにも残ります。

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

v0.1 の詳しいドキュメント: [v0.1_squeeze-sentinel/README.ja.md](v0.1_squeeze-sentinel/README.ja.md)。

## インストール

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 | iex

# ソースから（Rust 1.80+ がある任意の OS）
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

インストーラーはリリースの `checksums.txt`（SHA-256）を検証します。`SQUEEZE_VERSION=v0.1.0` でバージョン固定、`SQUEEZE_INSTALL_DIR` でインストール先を変更できます。

## クイックスタート

```bash
cd ~/my-project
squeeze init      # フック、設定、.gitignore の項目、CLAUDE.md の注記
claude            # いつも通り Claude Code を使う
squeeze report    # コンテキストから締め出した量を確認
```

`squeeze init` は 4 つのファイルを変更し、`squeeze uninstall` で元に戻せます:

| ファイル | 変更 |
|---|---|
| `.claude/settings.json` | `PreToolUse` フックを 2 つ（`Bash`、`Read`）追加。ほかの設定とフックは保持し、先にバックアップを作成。 |
| `.squeeze/config.json` | デフォルト設定。 |
| `.gitignore` | `.squeeze/` を追加（生ログに秘密情報が含まれる可能性があるため）。 |
| `CLAUDE.md` | `<!-- squeeze:start -->` マーカーの間に 4 行の注記を追加。 |

## 使い方

| コマンド | 内容 |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | プロジェクトに導入 |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | フックと注記を削除。`--purge` で `.squeeze/` も削除 |
| `squeeze run -- <command>` | コマンドを実行し、出力が長ければ要約 |
| `<command> 2>&1 \| squeeze compress` | パイプ出力を圧縮 |
| `squeeze clip [--print]` | クリップボードを圧縮（claude.ai 用） |
| `squeeze report [--since 7d] [--global] [--json]` | 節約量を表示 |
| `squeeze bench <dir> [--markdown]` | `*.log` を再生して測定 |
| `squeeze version` | バージョンを表示 |

コマンドの前に `SQUEEZE_SKIP=1` を付けると、そのコマンドだけ生の出力を得られます。設定は `.squeeze/config.json`（[全キー](v0.1_squeeze-sentinel/README.ja.md#設定)）。

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
- [ ] **v0.2 Interceptor**: claude.ai 貼り付け用ブラウザ拡張、同じエンジンを WebAssembly で
- [ ] **v0.3 Conduit**: `squeeze mcp`、Claude デスクトップアプリ用の読み取り専用 MCP サーバー
- [ ] Claude Code の実運用 A/B ベンチマーク（同じ作業で squeeze オン・オフ）
- [ ] v1.0: コマンドと設定の安定化

## FAQ

**Q. Claude が重要なことを見落とすことは?**
A. すべてのテストログで、異なるエラー・警告メッセージはすべて残りました。要約で足りなければ、最後の行に完全なログのパスがあり、Claude が範囲指定で読めます。

**Q. データをどこかに送信しますか?**
A. いいえ。ネットワークコードはありません。ログは自分のマシンの `.squeeze/` にだけ残ります。

**Q. Claude Code のセッションを壊すことは?**
A. フックは失敗時に引き下がります（fail open）。不正な入力、ファイルがない、内部エラーのどれでも何も出力せず、Claude Code は通常どおり進みます。

**Q. 普通の claude.ai チャットでもトークンは減りますか?**
A. `squeeze clip` を通して長いテキストを貼るときだけです。v0.2 でブラウザ内で自動化する予定です。

**Q. なぜ 0.x なのですか?**
A. メジャーバージョンが 0 の間は、コマンド名や設定が変わる可能性があります。v1.0 は安定の約束です。

## リポジトリ構成

```
squeeze/
├── README.md                    ← プロジェクト概要（6 言語）
├── v0.1_squeeze-sentinel/       ← v0.1: CLI + Claude Code フック（Rust、依存なし）
│   ├── README.md                   v0.1 の詳しいドキュメント（6 言語）
│   ├── src/
│   ├── docs/                       ベンチマーク方法、ユーザーガイド（韓国語）
│   └── testdata/fixtures/          テストとベンチマークで使うログ
├── install.sh / install.ps1
├── CHANGELOG.md（6 言語）
└── Cargo.toml                   ← ワークスペース
```

## コントリビュート

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --release -- bench v0.1_squeeze-sentinel/testdata/fixtures
```

新しいテストログを歓迎します: 実際の `*.log`（秘密情報と個人的なパスは削除）を `v0.1_squeeze-sentinel/testdata/fixtures/` に追加してください。エラー・警告が一つでも失われるとテストが失敗します。バグとアイデア: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues)。

## ライセンス

MIT。[LICENSE](LICENSE) を参照。squeeze は非公式のコミュニティツールであり、Anthropic とは関係ありません。
