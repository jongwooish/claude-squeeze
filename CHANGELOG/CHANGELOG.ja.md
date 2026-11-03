# 変更履歴

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | **日本語** | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

主な変更をここに記録します。バージョンは [セマンティック バージョニング](https://semver.org/lang/ja/) に従います。メジャーバージョンが 0 の間は、コマンドや設定が変わる可能性があります。

## [0.1.0] - 2026-11-03 — Sentinel

最初のリリース。コードは [`V0.1-Sentinel/v0.1_squeeze-sentinel/`](../V0.1-Sentinel/v0.1_squeeze-sentinel/) にあります。

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
