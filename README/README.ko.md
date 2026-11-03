# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.1-Sentinel/v0.1_squeeze-sentinel/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.1-Sentinel/v0.1_squeeze-sentinel/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#설치) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](README.md) | **한국어** | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **피드백 환영.** 개선사항, v1.0 출시, 버전 업데이트, 설치 오류 등 어떤 문제든 [Issues에 남겨주세요](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6개 언어 모두 가능).

**Claude 토큰은 로그 쓰레기가 아니라 생각하는 데 쓰세요.**

squeeze는 긴 명령 출력과 큰 파일 읽기가 Claude의 컨텍스트에 들어가지 않게 막습니다. Claude는 에러와 경고, 처음과 마지막 줄은 그대로 봅니다. 전체 로그는 필요할 때를 위해 내 디스크에 저장됩니다.

- [효과](#효과) · [버전](#버전) · [어디에 적용되나](#어디에-적용되나) · [v0.1 Sentinel](#v01-sentinel) · [설치](#설치) · [사용법](#사용법) · [동작 원리](#동작-원리) · [로드맵](#로드맵) · [FAQ](#faq)

## 왜 필요한가

Claude Code는 요청할 때마다 대화 전체를 다시 보내고, 명령 출력도 대화의 일부입니다. `npm install`, 테스트 실행, `journalctl` 한 번이 수천 토큰을 추가하고, 그게 이후 모든 턴에 따라붙습니다. 정작 Claude에게 필요한 건 "통과" 한 줄이나 에러 세 줄인데도요.

squeeze는 그 출력이 컨텍스트에 들어가기 **전에** 막습니다.

## 효과

실제 로그 7개를 squeeze에 통과시킨 결과입니다 ([측정 방법](../v0.1-Sentinel/v0.1_squeeze-sentinel/docs/bench.md)):

| 로그 | 줄 수 전 → 후 | 토큰 전 → 후 (추정) | 절약 | 에러·경고 메시지 보존 |
|---|---:|---:|---:|---:|
| `pytest` 실행, 실패 2개 | 217 → 39 | 4,040 → 516 | **87.2%** | 11 / 11 |
| `go test -v`, 실패 1개 | 488 → 16 | 4,247 → 202 | **95.2%** | 4 / 4 |
| `make`, gcc 경고 + 에러 1개 | 370 → 44 | 7,046 → 850 | **87.9%** | 7 / 7 |
| `tsc`, 타입 에러 2개 | 124 → 9 | 1,310 → 172 | **86.9%** | 3 / 3 |
| `dpkg -l` 패키지 목록 | 1049 → 63 | 35,237 → 2,106 | **94.0%** | 8 / 8 |
| 서버 저널, OOM 강제 종료 (합성 데이터) | 1400 → 37 | 31,161 → 997 | **96.8%** | 5 / 5 |
| 짧은 `npm` 출력 | 16 → 16 | 287 → 287 | 0.0% (그대로 둠) | 12 / 12 |
| **합계** | | **83,325 → 5,128** | **93.8%** | **50 / 50** |

**왜 유용한가**

- **이 테스트에서 중요한 건 하나도 잃지 않았습니다.** 서로 다른 에러·경고 메시지가 전부 남았습니다. 반복되는 메시지는 한 번만 보여주고 개수를 붙입니다 (`… [same message 29 more times] …`).
- **짧은 출력은 절대 건드리지 않습니다.** 80줄·8KB 이하면 원본 그대로입니다.
- **절약은 매 턴 반복됩니다.** 31,000토큰짜리 로그가 다음 20턴 동안 컨텍스트에 남아 있으면 입력 토큰이 약 620,000개입니다. 약 1,000토큰으로 줄이면 같은 20턴에 약 20,000개입니다.
- **한도에 늦게 닿습니다.** 잡음이 적으면 컨텍스트가 천천히 차서, 대화 압축이나 요금제 한도에 더 늦게 도달합니다.
- **Claude가 더 잘 집중합니다.** "빌드가 왜 실패했지?"의 답이 1,000줄 속에 묻히지 않고 바로 앞에 있습니다.

**솔직한 한계:** 토큰 수는 추정값(바이트 ÷ 4)이고, 이 숫자는 저장된 로그를 재생해서 얻은 것이지 Claude Code를 실제로 켜고 끄며 비교한 결과가 아닙니다 (실사용 비교는 계획 중). 내 프로젝트에서 `squeeze report`를 실행하면 실제 숫자를 볼 수 있습니다.

## 버전

버전마다 폴더와 README가 따로 있습니다. 릴리스는 레포 전체에 태그로 붙습니다.

| 버전 | 이름 | 추가되는 것 | 상태 |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1_squeeze-sentinel/README/README.ko.md) | **Sentinel** | 출력이 많은 명령과 큰 파일 통째 읽기를 잡는 Claude Code 훅, `squeeze run`, `compress`, `clip`, `report` | ✅ v0.1.0 |
| v0.2 | **Interceptor** | claude.ai에 긴 텍스트를 붙여넣을 때 줄일지 물어보는 브라우저 확장 | 🛠 계획 중 |
| v0.3 | **Conduit** | Claude 데스크톱 앱용 MCP 서버: Claude가 로그를 이미 줄인 상태로 읽음 | 🛠 계획 중 |

### 버전끼리의 관계

설치할 수 있는 건 두 가지입니다: **`squeeze` 프로그램**(CLI)과, v0.2부터의 **브라우저 확장**.

| 버전 | 무엇을 설치하나 | v0.1이 필요한가? |
|---|---|---|
| v0.1 Sentinel | `squeeze` 프로그램 | — |
| v0.2 Interceptor | 브라우저 확장 | **아니요.** 압축 엔진이 확장 안에 들어 있어 단독으로 동작합니다. Claude Code도 쓴다면 v0.1도 설치하세요. |
| v0.3 Conduit | 새로 설치할 것 없음: `squeeze` 프로그램 안의 명령(`squeeze mcp`) | **예.** `squeeze`를 v0.3 이상으로 업데이트하면 됩니다. 따로 내려받는 파일은 없습니다. |

- **프로그램은 누적됩니다.** 새 `squeeze`에는 이전 기능이 전부 들어 있습니다. v0.1과 v0.3을 나란히 설치할 일은 없습니다.
- **폴더는 각 버전에서 추가된 것을 설명합니다.** 폴더를 설치하는 게 아닙니다.
- **설정은 이어집니다.** `.squeeze/config.json`과 `squeeze init`으로 넣은 훅은 업데이트 후에도 그대로 동작합니다. 바뀌는 버전이 있으면 CHANGELOG에 적고 `squeeze init`이 갱신합니다.

| 이런 걸 쓴다면… | 설치할 것 |
|---|---|
| Claude Code만 | `squeeze` (지금은 v0.1) |
| claude.ai만 | 지금은 `squeeze clip`, 나오면 v0.2 확장 |
| Claude 데스크톱 앱에서 Claude가 직접 로그를 읽게 하고 싶다 | 나오면 `squeeze` v0.3 |
| 전부 | `squeeze` (최신) + v0.2 확장 |

## 어디에 적용되나

| 어디서 | 효과 | 방법 |
|---|---|---|
| **Claude Code** | 자동 | `squeeze init`이 훅을 설치합니다. 출력이 많은 명령과 큰 파일 읽기가 우회됩니다. |
| **claude.ai (웹, 데스크톱, 모바일)** | 긴 텍스트를 붙여넣을 때만, 수동 | 로그 복사 → `squeeze clip` → 붙여넣기. 평소 채팅은 바뀌지 않습니다. squeeze는 채팅을 볼 수 없습니다. |
| **다른 터미널 도구** | 수동 | `<명령> 2>&1 \| squeeze compress` |

Pro 요금제에서는 Claude Code와 claude.ai가 같은 사용량 한도를 나눠 쓰므로, Claude Code에서 아낀 토큰은 채팅에도 남습니다.

## v0.1 Sentinel

Sentinel은 Claude Code와 셸 사이를 지키는 보초입니다. 단일 실행 파일, Rust, **의존성 0개**, macOS / Linux / Windows.

- **Bash 가드**: 출력이 많은 명령(`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log` 등)을 `squeeze run -- <명령>`으로 다시 실행하게 합니다. 이미 출력이 제한된 명령(`| tail -20`, `-n 50`, `> file`, `--quiet`)은 건드리지 않습니다.
- **Read 가드**: 400줄이 넘는 파일을 `offset`/`limit` 없이 읽으려 하면 우회합니다. Claude는 줄 번호가 붙은 목차를 받고 필요한 구간만 읽습니다.
- **똑똑한 압축**: 처음 8줄과 마지막 20줄, 모든 에러·경고와 앞뒤 2줄, 스택 트레이스를 남깁니다. 반복은 개수로 묶고, 에러를 경고보다 먼저 고릅니다.
- **종료 코드 유지**: `squeeze run`은 원래 명령의 종료 코드를 그대로 돌려줍니다.
- **전체 로그는 디스크에**: `.squeeze/raw/`, 최근 50개 유지, 본인만 읽기 가능. `.squeeze/`는 `.gitignore`에 추가됩니다.
- claude.ai용 **`squeeze clip`**, 실제 절약량을 보는 **`squeeze report`**.
- **안전 설계**: 훅은 절대 대신 승인하지 않고, 오류가 나면 조용히 물러나며(fail open), `squeeze uninstall`이 추가한 것을 모두 지웁니다. 네트워크 코드가 없습니다.

`squeeze run -- pytest -v` 후 Claude가 보는 것 (실제 출력, 217줄 → 39줄):

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
… (트레이스백 2개는 전부 남음) …
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

v0.1 전체 문서: [v0.1-Sentinel/v0.1_squeeze-sentinel/README.ko.md](../v0.1-Sentinel/v0.1_squeeze-sentinel/README/README.ko.md). 한국어 가이드: [docs/guide](../v0.1-Sentinel/v0.1_squeeze-sentinel/docs/guide/index.md).

## 설치

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# 소스에서 (Rust 1.80+ 가 있는 모든 OS)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

설치 스크립트는 릴리스의 `checksums.txt`(SHA-256)를 확인합니다. `SQUEEZE_VERSION=v0.1.0`으로 버전 고정, `SQUEEZE_INSTALL_DIR`로 설치 폴더 변경.

## 빠른 시작

```bash
cd ~/my-project
squeeze init      # 훅, 설정, .gitignore 항목, CLAUDE.md 안내
claude            # 평소처럼 Claude Code 사용
squeeze report    # 컨텍스트에서 막아낸 양 확인
```

`squeeze init`은 파일 4개를 바꾸고, `squeeze uninstall`이 되돌립니다:

| 파일 | 변경 |
|---|---|
| `.claude/settings.json` | `PreToolUse` 훅 2개(`Bash`, `Read`) 추가. 다른 설정과 훅은 유지, 먼저 백업 작성. |
| `.squeeze/config.json` | 기본 설정. |
| `.gitignore` | `.squeeze/` 추가 (원본 로그에 비밀 정보가 있을 수 있음). |
| `CLAUDE.md` | `<!-- squeeze:start -->` 표시 사이에 4줄 안내 추가. |

## 사용법

| 명령 | 하는 일 |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | 프로젝트에 설치 |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | 훅과 안내 제거, `--purge`는 `.squeeze/`도 삭제 |
| `squeeze run -- <명령>` | 명령 실행, 출력이 길면 요약 |
| `<명령> 2>&1 \| squeeze compress` | 파이프 출력 압축 |
| `squeeze clip [--print]` | 클립보드 압축 (claude.ai용) |
| `squeeze report [--since 7d] [--global] [--json]` | 절약량 보기 |
| `squeeze bench <폴더> [--markdown]` | `*.log` 재생해서 측정 |
| `squeeze version` | 버전 출력 |

명령 앞에 `SQUEEZE_SKIP=1`을 붙이면 그 명령만 원본 출력을 받습니다. 설정은 `.squeeze/config.json` ([전체 키](../v0.1-Sentinel/v0.1_squeeze-sentinel/README/README.ko.md#설정)).

## 동작 원리

```
Claude가 실행하려 함:  npm test
        │
        ▼
PreToolUse 훅 ── squeeze hook pre-bash
        │  출력이 많은 명령? ── 아니요 ──▶ 조용히 통과, 평소 권한 확인
        │  예
        ▼
거부 + 힌트: "squeeze run -- 'npm test' 로 실행하세요"
        │
        ▼
Claude가 실행: squeeze run -- 'npm test'   (평소 권한 규칙 그대로 적용)
        │
        ├─▶ 전체 출력 ── .squeeze/raw/<시각>.log
        └─▶ 짧은 출력 ── Claude  (에러, 경고, 처음, 끝, 로그 경로)
```

왜 명령을 몰래 바꾸지 않고 "거부 + 힌트"일까요? 명령을 바꿔치기하는 훅은 그 명령을 승인까지 해야 해서, 사용자의 권한 확인을 건너뛰게 됩니다. squeeze는 절대 대신 승인하지 않습니다.

### 래퍼 도구와 다른 점

[chaude](https://github.com/int-ttt/chaude) 같은 도구는 Claude Code CLI를 자체 화면으로 감싸고, 세션을 갈아타거나 싼 모델에 일을 넘겨 토큰을 아낍니다. squeeze는 문제의 반대편을 맡습니다:

- **래퍼가 아닙니다.** Claude Code 자체 훅을 통해 동작하므로 `claude`를 그대로 씁니다.
- **꽉 찬 컨텍스트를 치우는 대신, 처음부터 잡음이 못 들어오게** 합니다.
- **둘을 함께 쓸 수 있습니다.**

## 로드맵

- [x] **v0.1 Sentinel**: Claude Code 훅, `run`, `compress`, `clip`, `report`, `bench`
- [ ] **v0.2 Interceptor**: claude.ai 붙여넣기용 브라우저 확장, 같은 엔진을 WebAssembly로
- [ ] **v0.3 Conduit**: `squeeze mcp`, Claude 데스크톱 앱용 읽기 전용 MCP 서버
- [ ] Claude Code 실사용 A/B 벤치마크 (같은 작업, squeeze 켜고/끄고)
- [ ] v1.0: 명령과 설정 안정화

## FAQ

**Q. Claude가 중요한 걸 놓칠 수 있나요?**
A. 모든 테스트 로그에서 서로 다른 에러·경고 메시지가 전부 남았습니다. 요약으로 부족하면 마지막 줄에 전체 로그 경로가 있어서 Claude가 구간 읽기로 확인할 수 있습니다.

**Q. 내 데이터를 어딘가로 보내나요?**
A. 아니요. 네트워크 코드가 없습니다. 로그는 내 컴퓨터의 `.squeeze/`에만 남습니다.

**Q. Claude Code 세션이 망가질 수 있나요?**
A. 훅은 실패 시 물러납니다(fail open). 이상한 입력, 없는 파일, 내부 오류가 나도 아무것도 출력하지 않고 Claude Code는 평소처럼 진행합니다.

**Q. 평범한 claude.ai 채팅에서도 토큰이 줄어드나요?**
A. `squeeze clip`을 거쳐 긴 텍스트를 붙여넣을 때만 줄어듭니다. v0.2에서 브라우저 안에서 자동으로 되게 할 예정입니다.

**Q. 왜 0.x 버전인가요?**
A. 앞자리가 0인 동안은 명령 이름이나 설정이 바뀔 수 있다는 뜻입니다. v1.0은 안정됐다는 약속입니다.

## 레포 구조

```
claude-squeeze/
├── README/                      ← 프로젝트 소개 6개 언어 (README.md = 영어)
├── CHANGELOG/                   ← CHANGELOG 6개 언어
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   버전 소개
│   └── v0.1_squeeze-sentinel/   ← v0.1: CLI + Claude Code 훅 (Rust, 의존성 없음)
│       ├── README.md               v0.1 전체 문서 (영어)
│       ├── README/                 번역본 (5개 언어)
│       ├── src/
│       ├── docs/                   벤치마크 방법, 사용자 가이드(한국어)
│       └── testdata/fixtures/      테스트와 벤치마크에 쓰는 로그
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock (Rust 워크스페이스)
├── .github/                     ← 이슈 양식, CI, 릴리스, 첫 화면용 README
├── CLAUDE.md
└── LICENSE
```

## 기여하기

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures
```

새 테스트 로그를 환영합니다: 실제 `*.log`(비밀 정보와 개인 경로 제거)를 `v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures/`에 추가하세요. 에러·경고가 하나라도 사라지면 테스트가 실패합니다. 버그와 아이디어: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).

## 커뮤니티

**문제를 찾았거나 아이디어가 있다면 [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose)에 남겨주세요.** 개선사항, v1.0 출시, 버전 업데이트, 설치 오류 등 어떤 소프트웨어 문제든 다음 버전을 만드는 데 큰 도움이 됩니다.


| 이럴 때 | 여기로 |
|---|---|
| 버그나 세션이 망가지는 문제 | [Issues → Bug report](https://github.com/jongwooish/claude-squeeze/issues/new?template=bug_report.yml) |
| 중요한 줄이 빠짐, 절약이 적음, 명령이 잘못 잡히거나 안 잡힘 | [Issues → Compression problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=compression.yml) |
| 설치·업데이트·버전 문제 | [Issues → Install, update or version problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=version_problem.yml) |
| 기능 제안 | [Issues → Feature idea](https://github.com/jongwooish/claude-squeeze/issues/new?template=feature_request.yml) |
| 질문, 문제 해결 도움 | [Discussions → Q&A](https://github.com/jongwooish/claude-squeeze/discussions/categories/q-a) |
| 로드맵, v1.0에 들어갈 것에 대한 의견 | [Discussions → Ideas](https://github.com/jongwooish/claude-squeeze/discussions/categories/ideas) |
| 새 버전 소식 받기 | [Discussions → Announcements](https://github.com/jongwooish/claude-squeeze/discussions/categories/announcements)와 [Releases](https://github.com/jongwooish/claude-squeeze/releases) (**Watch → Custom → Releases**를 누르면 알림이 옵니다) |

6개 언어 중 아무 언어로나 써도 됩니다. 보안 문제는 [SECURITY.md](../.github/SECURITY.md), 기여 방법은 [CONTRIBUTING.md](../.github/CONTRIBUTING.md)를 보세요.

## 라이선스

MIT. [LICENSE](../LICENSE) 참고. squeeze는 비공식 커뮤니티 도구이며 Anthropic과 관련이 없습니다.
