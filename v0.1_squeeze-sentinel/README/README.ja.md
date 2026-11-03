# v0.1 Sentinel

[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **フィードバック歓迎。** 改善点、v1.0 のリリース、バージョン更新、インストールエラーなど、どんな問題でも [Issue を作成してください](https://github.com/jongwooish/claude-squeeze/issues/new/choose)（6 言語対応）。

← [squeeze に戻る](../../readme/README.ja.md)

Sentinel は Claude Code とシェルの間に立つ見張り番です。Claude がコンテキストをあふれさせるコマンド（インストール、ビルド、テスト、ログ）を実行しようとしたり、巨大なファイルを丸ごと読もうとしたりすると、Sentinel が短い版へ誘導します。Claude はエラーと警告をすべて受け取ります。

単一バイナリ、Rust 製、**依存関係ゼロ**。macOS、Linux、Windows で動作します。

**必要なもの:** なし。これが基本の `squeeze` プログラムです。以降のバージョンはこの上に積み上がります。v0.3 Conduit は同じプログラムにコマンドを追加し（更新すれば使えます）、v0.2 Interceptor はこのプログラムの有無にかかわらず動く別のブラウザ拡張です。[バージョン同士の関係](../../readme/README.ja.md#バージョン同士の関係)を参照。

## 効果

テストログ 7 件の再生で、推定トークン 83,325 → 5,128（**−93.8%**）、異なるエラー・警告はすべて保持。詳細と方法: [docs/bench.md](../docs/bench.md)（英語）。

`squeeze run -- pytest -v` の後に Claude が見るもの（217 行 → 39 行）。`pytest-fail.log` の実際の出力です:

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

## 機能

- **Bash ガード**: 出力の多いコマンド（`npm install`、`go test`、`cargo build`、`pytest`、`make`、`apt-get install`、`journalctl`、`docker build`、`kubectl logs`、`git log` など）を `squeeze run -- <command>` へ誘導します。
- **Read ガード**: 400 行を超えるファイルを `offset`/`limit` なしで読もうとすると誘導します。Claude は行番号付きの目次を受け取り、範囲を選びます。
- **賢い圧縮**: 最初の 8 行と最後の 20 行、すべてのエラー・警告とその前後、スタックトレースを残します。繰り返しは件数でまとめ、警告よりエラーを優先します。
- **終了コードを保持**: `squeeze run` は元のコマンドの終了コードで終わります。
- **完全なログはディスクに**: `.squeeze/raw/`、最新 50 件を保持、本人のみ読み取り可（0600）。`.squeeze/` は `.gitignore` に追加されます。
- claude.ai 用の **`squeeze clip`**: チャットにログを貼る前にクリップボードを圧縮します。
- **`squeeze report`**: コンテキストに入るはずだった量、実際に入った量、最大の節約。
- **安全な設計**: フックはツール呼び出しを決して承認せず、エラー時は静かに引き下がり（fail open）、`squeeze uninstall` で追加したものをすべて消します。

## インストール

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 | iex
# ソースから
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

インストーラーはインストール前にリリースの `checksums.txt` を確認します。`SQUEEZE_VERSION=v0.1.0` でバージョン固定、`SQUEEZE_INSTALL_DIR` でフォルダ変更。

## クイックスタート

```bash
cd ~/my-project
squeeze init
claude
```

`squeeze init` は 4 つの変更を行い、`squeeze uninstall` で元に戻せます:

| ファイル | 変更 |
|---|---|
| `.claude/settings.json` | `PreToolUse` フックを 2 つ（`Bash`、`Read`）追加。ほかの設定とフックは保持し、先に `.squeeze.bak` を作成。 |
| `.squeeze/config.json` | デフォルト設定（下記）。 |
| `.gitignore` | `.squeeze/` を追加。生ログに秘密情報が含まれる可能性があるため。 |
| `CLAUDE.md` | `<!-- squeeze:start -->` マーカーの間に 4 行の注記を追加。 |

## 使い方

| コマンド | 内容 |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | プロジェクトに導入。`squeeze` が `PATH` にない場合は `--bin` に絶対パス。 |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | フックと CLAUDE.md の注記を削除。`--purge` で `.squeeze/` も削除。 |
| `squeeze run -- <command>` | コマンドを実行し、出力が長ければ短い要約を表示。 |
| `<command> 2>&1 \| squeeze compress` | パイプ出力を圧縮。 |
| `squeeze clip [--print]` | クリップボードを圧縮版に置き換え（`--print` は表示のみ）。 |
| `squeeze report [--since 7d] [--global] [--json] [--top N]` | このプロジェクトの節約量。`--global` は `squeeze clip` の節約量。 |
| `squeeze bench <dir> [--markdown]` | `*.log` を再生して節約量を測定。 |
| `squeeze hook pre-bash` / `pre-read` | Claude Code が呼ぶフックの入口。 |
| `squeeze version` | バージョンを表示。 |

あるコマンドの生の出力がどうしても必要なら、Claude（またはあなた）が先頭に `SQUEEZE_SKIP=1` を付けます。

## 設定

`.squeeze/config.json`（すべて任意）:

| キー | 既定値 | 意味 |
|---|---|---|
| `compress.max_lines` / `max_bytes` | `80` / `8000` | 両方以下の出力はそのまま。 |
| `compress.head_lines` / `tail_lines` | `8` / `20` | 先頭 / 末尾で常に残す行数。 |
| `compress.context` | `2` | エラー・警告の前後に残す行数。 |
| `compress.budget` | `120` | 残す行数のおおよその上限。 |
| `guard_bash` / `guard_read` | `true` / `true` | 各ガードのオン・オフ。 |
| `read_max_lines` | `400` | これより長いファイルは範囲読みに誘導。 |
| `extra_noisy` | `[]` | 出力が多いとみなすコマンドの先頭部分。例: `"./deploy.sh"`。 |
| `quiet` | `[]` | ガードが決して触れないコマンドの先頭部分。例: `"make docs"`。 |
| `keep_raw` | `50` | `.squeeze/raw/` に残す生ログの数。 |

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
        ▼
完全な出力 ──▶ .squeeze/raw/<時刻>.log
短い出力 ────▶ Claude（エラー、警告、先頭、末尾、ログのパスを含む最終行）
```

なぜこっそり書き換えずに「拒否 + ヒント」なのか? コマンドを書き換えるフックはそれを承認もしなければならず、ユーザーの権限確認を飛ばしてしまいます。squeeze は決してそうしません。

詳しくは: [docs/guide/how-it-works.md](../docs/guide/how-it-works.md)（韓国語）。

## どこで効くか

- **Claude Code**: フックで自動。
- **claude.ai**: 長いテキストを貼るときに `squeeze clip` を使う場合のみ。普段のチャットには影響しません。squeeze はチャットを見ることも変えることもできません。
- **ほかのターミナルツール**: `squeeze compress` にパイプ。

## FAQ

**Q. Claude が重要なことを見落とすことは?**
A. すべてのテストログで、異なるエラー・警告メッセージはすべて残りました。要約で足りなければ最終行に完全なログのパスがあり、Claude が範囲指定で読めます。

**Q. データをどこかに送信しますか?**
A. いいえ。ネットワークコードはありません。ログはマシン上の `.squeeze/` に残ります。

**Q. セッションを壊すことは?**
A. フックは失敗時に引き下がります。不正な入力、ファイルがない、パニックのどれでも何も出力せず、Claude Code は通常どおり進みます。

**Q. 大きなファイルを Claude が何度も止められますが、全体が必要です。**
A. ヒントに `limit=<行数>` で読み直すよう書かれており、そうすれば通ります。または `"guard_read": false` でオフにしてください。

**Q. 普通の claude.ai チャットでもトークンは減りますか?**
A. `squeeze clip` を通して長いテキストを貼るときだけです。

## フィードバック

改善点や問題は [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) に残してください: v1.0 への要望、バージョン更新やインストールのエラー、圧縮で何かが消えた場合、その他のソフトウェアの問題すべて歓迎します。フォーム（バグ、圧縮の問題、インストール・更新・バージョンの問題、機能の提案）から選んでください。質問やロードマップの話は [Discussions](https://github.com/jongwooish/claude-squeeze/discussions) へ。6 言語のどれでも構いません。

## コントリビュート

```bash
cargo test                       # 単体テスト（テストログの検査を含む）
cargo clippy -- -D warnings
cargo run -- bench v0.1_squeeze-sentinel/testdata/fixtures
```

新しいテストログを歓迎します: 実際の `*.log`（先に秘密情報と個人的なパスを削除）を `testdata/fixtures/` に追加してください。異なるエラー・警告が一つでも失われるとテストが失敗します。

課題とアイデア: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues)。
