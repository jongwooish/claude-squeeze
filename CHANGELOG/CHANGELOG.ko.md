# 변경 기록

[English](CHANGELOG.md) | **한국어** | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | [Français](CHANGELOG.fr.md)

주요 변경 사항을 여기에 적습니다. 버전은 [유의적 버전](https://semver.org/lang/ko/)을 따릅니다. 앞자리가 0인 동안은 명령과 설정이 바뀔 수 있습니다.

## [0.2.0] - 2026-11-04 — squeeze-Auditor

v0.2는 출력 하나가 아니라 세션 전체를 봅니다. 코드는 [`v0.2-Auditor/v0.2.0_squeeze-auditor/`](../v0.2-Auditor/v0.2.0_squeeze-auditor/). 0.1.0의 기능을 모두 포함합니다.

### 추가

- 세션 차단 4가지 (Claude Code):
  - **중복 읽기**: 이번 세션에서 이미 읽었고 바뀌지 않은 파일을 다시 통째로 `Read`하면 거부. 파일이 바뀌거나 컨텍스트가 압축되면 다시 허용.
  - **예산 일시 정지** (`UserPromptSubmit`): 세션이 예산(기본 150K 토큰)을 넘으면 메시지를 한 번 멈추고 `/compact`나 `/clear`를 권함. 다시 보내면 그대로 보내짐.
  - **이어가기** (`Stop` + `SessionStart`): 세션이 이어가기 크기(기본 120K)를 넘으면 Claude가 `.squeeze/handoff/`에 메모를 쓰고, `/clear` 후 새 세션이 메모로 시작. `remind` 모드는 `/compact`만 권함.
  - **고정비**: 메시지마다 보내지는 `CLAUDE.md`와 불러온 파일, 스킬, 에이전트, MCP 도구 목록을 잼. Claude Code 자체 스위치로 프로젝트별 MCP 서버 끄기, 또는 N일 안 쓰면 자동으로 끄기(기본 꺼짐).
- **squeeze-Auditor** 앱 (`squeeze-auditor`, `squeeze auditor`): Claude Code 세션 기록의 실제 사용량과 squeeze가 막은 양의 대시보드, 현재 5시간 구간, 절약 모드(강하게·균형·가볍게·직접 설정), 모든 설정, 측정·켜기/끄기가 있는 고정비 진단, 이어가기 메모, 연결 관리. 이 컴퓨터 안에서만(`127.0.0.1`, 1회용 열쇠). 영어·한국어.
- **Claude 데스크톱 앱** 지원: `squeeze mcp`, 허용한 폴더로 제한된 `read_log`, `tail_log`, `search_log`, `list_logs`, `save_handoff`, `load_handoff` 도구를 가진 MCP 서버.
- `squeeze setup [--claude-code] [--desktop] [--remove]`: 모든 프로젝트(`~/.claude/settings.json`)와 데스크톱 앱에 한 번에 연결. 백업은 `~/.squeeze/backups/`.
- `squeeze doctor`: 메시지당 고정비를 터미널에서.
- 설치 파일: Windows Setup(x64, ARM64, 사용자별 설치, 시작 메뉴 아이콘, 연결 해제까지 하는 제거), `squeeze-Auditor.app`이 든 macOS `.pkg`, 메뉴 항목이 있는 Linux `.deb`. 아직 코드 서명 없음.
- 전체 설정 `~/.squeeze/config.json`(프로젝트의 `.squeeze/config.json`이 여전히 우선)과 전체 기록 `~/.squeeze/ledger.jsonl`.

### 변경

- 설치 스크립트가 `squeeze-auditor`도 설치.
- 릴리스 제목을 CHANGELOG 제목에서 가져옴. CI가 설치 파일도 만들고, 동결된 v0.1.0 스냅샷도 테스트.
- 동결된 v0.1.0 크레이트 이름을 `squeeze-sentinel`로 변경(프로그램 이름은 그대로 `squeeze`). `cargo install --git … squeeze`는 v0.2를 빌드.

### 계획

- v0.3: 브라우저의 claude.ai와 모바일 앱, 새로운 접근.

## [0.1.0] - 2026-11-03 — Sentinel

첫 릴리스. 코드는 [`v0.1-Sentinel/v0.1.0_squeeze-sentinel/`](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/)에 있습니다.

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
