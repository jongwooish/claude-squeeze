# 변경 기록

[English](CHANGELOG.md) | **한국어** | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

주요 변경 사항을 여기에 적습니다. 버전은 [유의적 버전](https://semver.org/lang/ko/)을 따릅니다. 앞자리가 0인 동안은 명령과 설정이 바뀔 수 있습니다.

## [0.1.0] - 2026-11-03 — Sentinel

첫 릴리스. 코드는 [`v0.1_squeeze-sentinel/`](v0.1_squeeze-sentinel/)에 있습니다.

### 추가

- Claude Code `PreToolUse` 훅:
  - **Bash 가드**: 출력이 많은 명령(설치, 빌드, 테스트, 로그)을 `squeeze run`으로 우회. 이미 출력이 제한된 명령(`| tail`, `-n 50`, 리다이렉트, `--quiet`)은 건드리지 않음.
  - **Read 가드**: 400줄이 넘는 파일의 통째 읽기를 목차와 함께 구간 읽기로 우회.
  - 훅은 절대 도구 호출을 승인하지 않고, 어떤 오류에도 조용히 물러남(fail open).
- `squeeze run -- <명령>`: 실행, 긴 출력 압축, 전체 로그 저장, 종료 코드 유지.
- `squeeze compress`: 파이프 출력 압축.
- `squeeze clip`: claude.ai에 붙여넣기 위해 클립보드 압축 (macOS, Windows, Linux).
- `squeeze report`: 프로젝트별 절약량 (`--global`은 clip, `--json`, `--since`).
- `squeeze bench`: 로그를 재생해서 절약량과 보존된 메시지 측정.
- `squeeze init` / `squeeze uninstall`: 훅, 설정, `.gitignore` 항목, `CLAUDE.md` 안내를 설치하고 깔끔하게 제거.
- 압축기: 처음과 끝, 서로 다른 모든 에러·경고와 문맥, 스택 트레이스 유지. 반복은 묶고, 에러를 경고보다 먼저 선택.
- 벤치마크용 테스트 로그 7개: 추정 토큰 93.8% 감소, 서로 다른 메시지 50/50 보존.
- macOS/Linux(`install.sh`), Windows(`install.ps1`) 설치 스크립트, SHA-256 검증 포함.
- 영어, 한국어, 일본어, 중국어(간체), 스페인어, 프랑스어 README.

### 계획

- v0.2 Interceptor: claude.ai 붙여넣기용 브라우저 확장.
- v0.3 Conduit: Claude 데스크톱 앱용 MCP 서버.
- Claude Code 실사용 A/B 벤치마크.
