# 変更履歴

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | **日本語** | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

主な変更をここに記録します。バージョンは [セマンティック バージョニング](https://semver.org/lang/ja/) に従います。メジャーバージョンが 0 の間は、コマンドや設定が変わる可能性があります。

## [0.2.0] - 2026-11-04 — squeeze-Auditor

v0.2 は出力 1 つではなくセッション全体を見ます。コードは [`v0.2-Auditor/v0.2.0_squeeze-auditor/`](../v0.2-Auditor/v0.2.0_squeeze-auditor/)。0.1.0 の機能をすべて含みます。

### 追加

- セッションの 4 つのブロック（Claude Code）:
  - **重複読み込み**: このセッションですでに読み、変わっていないファイルを丸ごと `Read` すると拒否。ファイルが変わるかコンテキストが圧縮されると再び許可。
  - **予算で一時停止**（`UserPromptSubmit`）: セッションが予算（既定 150K トークン）を超えると、メッセージを一度止めて `/compact` か `/clear` を勧める。もう一度送るとそのまま送信。
  - **引き継ぎ**（`Stop` + `SessionStart`）: セッションが引き継ぎサイズ（既定 120K）を超えると Claude が `.squeeze/handoff/` にメモを書き、`/clear` 後の新しいセッションはメモから始まる。`remind` モードは `/compact` を勧めるだけ。
  - **固定費**: メッセージごとに送られる `CLAUDE.md` とインポート、スキル、エージェント、MCP ツール一覧を測定。Claude Code 自身のスイッチでプロジェクトごとに MCP サーバーをオフ、または N 日使われなければ自動でオフ（既定は無効）。
- **squeeze-Auditor** アプリ（`squeeze-auditor`、`squeeze auditor`）: Claude Code のセッションログから読んだ実際の使用量と squeeze が防いだ量のダッシュボード、現在の 5 時間枠、節約モード（強め・バランス・軽め・カスタム）、すべての設定、計測とオン・オフ付きの固定費診断、引き継ぎメモ、接続。このコンピューター内のみ（`127.0.0.1`、使い捨てキー）。英語と韓国語。
- **Claude デスクトップアプリ**対応: `squeeze mcp`。許可したフォルダに限定した `read_log`、`tail_log`、`search_log`、`list_logs`、`save_handoff`、`load_handoff` を持つ MCP サーバー。
- `squeeze setup [--claude-code] [--desktop] [--remove]`: 全プロジェクト（`~/.claude/settings.json`）とデスクトップアプリに一度で接続。バックアップは `~/.squeeze/backups/`。
- `squeeze doctor`: メッセージあたりの固定費をターミナルで。
- インストーラー: Windows Setup（x64、ARM64、ユーザー単位、スタートメニューのアイコン、接続も解除するアンインストーラー）、`squeeze-Auditor.app` 入りの macOS `.pkg`、メニュー項目付きの Linux `.deb`。まだコード署名なし。
- 全体設定 `~/.squeeze/config.json`（プロジェクトの `.squeeze/config.json` が引き続き優先）と全体の記録 `~/.squeeze/ledger.jsonl`。

### 変更

- インストールスクリプトが `squeeze-auditor` もインストール。
- リリースのタイトルを CHANGELOG の見出しから取得。CI はインストーラーも作り、凍結した v0.1.0 スナップショットもテスト。
- 凍結した v0.1.0 のクレート名を `squeeze-sentinel` に変更（プログラム名は `squeeze` のまま）。`cargo install --git … squeeze` は v0.2 をビルド。

### 予定

- v0.3: ブラウザの claude.ai とモバイルアプリ、新しいアプローチ。

## [0.1.0] - 2026-11-03 — Sentinel

最初のリリース。コードは [`v0.1-Sentinel/v0.1.0_squeeze-sentinel/`](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/) にあります。

### 追加

- Claude Code の `PreToolUse` フック:
  - **Bash ガード**: 出力の多いコマンド（インストール、ビルド、テスト、ログ）を `squeeze run` へ誘導。すでに出力が制限されたコマンド（`| tail`、`-n 50`、リダイレクト、`--quiet`）には触れない。
  - **Read ガード**: 400 行を超えるファイルの丸読みを、目次付きの範囲読みへ誘導。
  - フックはツール呼び出しを決して承認せず、どんなエラーでも静かに引き下がる（fail open）。
- `squeeze run -- <command>`: 実行、長い出力の圧縮、完全なログの保存、終了コードの保持。
- `squeeze compress`: パイプ出力の圧縮。
- `squeeze clip`: claude.ai に貼るためにクリップボードを圧縮（macOS、Windows、Linux）。
- `squeeze report`: プロジェクトごとの節約量（clip は `--global`、`--json`、`--since`）。
- `squeeze bench`: ログを再生して節約量と保持されたメッセージを測定。
- `squeeze init` / `squeeze uninstall`: フック、設定、`.gitignore` の項目、`CLAUDE.md` の注記を導入し、きれいに削除。
- 圧縮器: 先頭と末尾、すべての異なるエラー・警告とその前後、スタックトレースを保持。繰り返しはまとめ、警告よりエラーを優先。
- ベンチマーク用テストログ 7 件: 推定トークン 93.8% 削減、異なるメッセージ 50/50 を保持。
- macOS/Linux（`install.sh`）と Windows（`install.ps1`）のインストーラー、SHA-256 検証付き。
- 英語、韓国語、日本語、簡体字中国語、スペイン語、フランス語の README。

### 予定

- v0.2 Interceptor: claude.ai への貼り付け用ブラウザ拡張。
- v0.3 Conduit: Claude デスクトップアプリ用 MCP サーバー。
- Claude Code の実運用 A/B ベンチマーク。
