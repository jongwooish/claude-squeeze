# v0.1 Sentinel

[English](README.md) | **한국어** | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md)

> 💬 **피드백 환영.** 개선사항, v1.0 출시, 버전 업데이트, 설치 오류 등 어떤 문제든 [Issues에 남겨주세요](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6개 언어 모두 가능).

← [squeeze로 돌아가기](../../../readme/README.ko.md)

Sentinel은 Claude Code와 셸 사이를 지키는 보초입니다. Claude가 컨텍스트를 가득 채울 명령(설치, 빌드, 테스트, 로그)을 실행하거나 거대한 파일을 통째로 읽으려 하면, Sentinel이 짧은 버전으로 돌려보냅니다. Claude는 에러와 경고를 빠짐없이 받습니다.

단일 실행 파일, Rust로 작성, **의존성 0개**. macOS, Linux, Windows에서 동작합니다.

**필요한 것:** 없음. 이것이 기본 `squeeze` 프로그램입니다. 이후 버전은 이 위에 쌓입니다: v0.3 Conduit은 같은 프로그램에 명령을 추가하고(업데이트하면 생김), v0.2 Interceptor는 이 프로그램이 있든 없든 동작하는 별도의 브라우저 확장입니다. [버전끼리의 관계](../../../readme/README.ko.md#버전끼리의-관계) 참고.

## 효과

테스트 로그 7개 재생, 추정 토큰 83,325 → 5,128 (**−93.8%**), 서로 다른 에러·경고 전부 보존. 자세한 내용과 방법: [docs/bench.md](../docs/bench.md) (영어).

`squeeze run -- pytest -v` 후 Claude가 보는 것 (217줄 → 39줄). `pytest-fail.log` 테스트 로그의 실제 출력입니다:

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

## 기능

- **Bash 가드**: 출력이 많은 명령(`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log` 등)을 `squeeze run -- <명령>`으로 다시 실행하게 합니다.
- **Read 가드**: 400줄이 넘는 파일을 `offset`/`limit` 없이 읽으려 하면 우회합니다. Claude는 줄 번호가 붙은 목차를 받고 구간을 고릅니다.
- **똑똑한 압축**: 처음 8줄과 마지막 20줄, 모든 에러·경고와 문맥, 스택 트레이스를 남깁니다. 반복 메시지는 개수로 묶고, 에러를 경고보다 먼저 남깁니다.
- **종료 코드 유지**: `squeeze run`은 원래 명령의 종료 코드로 끝납니다.
- **전체 로그는 디스크에**: `.squeeze/raw/`, 최근 50개 유지, 본인만 읽기 가능(0600). `.squeeze/`는 `.gitignore`에 추가됩니다.
- claude.ai용 **`squeeze clip`**: 채팅에 로그를 붙여넣기 전에 클립보드를 압축합니다.
- **`squeeze report`**: 컨텍스트에 들어갈 뻔한 양, 실제로 들어간 양, 가장 많이 아낀 것.
- **안전 설계**: 훅은 절대 도구 호출을 승인하지 않고, 오류가 나면 조용히 물러나며(fail open), `squeeze uninstall`이 추가한 것을 모두 지웁니다.

## 설치

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 | iex
# 소스에서
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

설치 스크립트는 설치 전에 릴리스의 `checksums.txt`를 확인합니다. `SQUEEZE_VERSION=v0.1.0`으로 버전 고정, `SQUEEZE_INSTALL_DIR`로 폴더 변경.

## 빠른 시작

```bash
cd ~/my-project
squeeze init
claude
```

`squeeze init`은 네 가지를 바꾸고, `squeeze uninstall`이 되돌립니다:

| 파일 | 변경 |
|---|---|
| `.claude/settings.json` | `PreToolUse` 훅 2개(`Bash`, `Read`) 추가. 다른 설정과 훅은 유지, 먼저 `.squeeze.bak` 백업 작성. |
| `.squeeze/config.json` | 기본 설정 (아래 참고). |
| `.gitignore` | `.squeeze/` 추가. 원본 로그에 비밀 정보가 있을 수 있기 때문. |
| `CLAUDE.md` | `<!-- squeeze:start -->` 표시 사이에 4줄 안내 추가. |

## 사용법

| 명령 | 하는 일 |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | 프로젝트에 설치. `squeeze`가 `PATH`에 없으면 `--bin`에 절대 경로. |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | 훅과 CLAUDE.md 안내 제거. `--purge`는 `.squeeze/`도 삭제. |
| `squeeze run -- <명령>` | 명령 실행. 출력이 길면 짧은 요약 출력. |
| `<명령> 2>&1 \| squeeze compress` | 파이프 출력 압축. |
| `squeeze clip [--print]` | 클립보드를 압축본으로 교체 (`--print`는 보여주기만). |
| `squeeze report [--since 7d] [--global] [--json] [--top N]` | 이 프로젝트의 절약량. `--global`은 `squeeze clip` 절약량. |
| `squeeze bench <폴더> [--markdown]` | `*.log` 파일을 재생해서 절약량 측정. |
| `squeeze hook pre-bash` / `pre-read` | Claude Code가 호출하는 훅 진입점. |
| `squeeze version` | 버전 출력. |

어떤 명령의 원본 출력이 꼭 필요하면 Claude(또는 사용자)가 앞에 `SQUEEZE_SKIP=1`을 붙이면 됩니다.

## 설정

`.squeeze/config.json` (모든 키는 선택):

| 키 | 기본값 | 뜻 |
|---|---|---|
| `compress.max_lines` / `max_bytes` | `80` / `8000` | 둘 다 이하인 출력은 그대로 둠. |
| `compress.head_lines` / `tail_lines` | `8` / `20` | 처음 / 끝에서 항상 남길 줄 수. |
| `compress.context` | `2` | 에러·경고 앞뒤로 남길 줄 수. |
| `compress.budget` | `120` | 남길 줄 수의 대략적인 상한. |
| `guard_bash` / `guard_read` | `true` / `true` | 각 가드 켜기/끄기. |
| `read_max_lines` | `400` | 이보다 긴 파일은 구간 읽기로 유도. |
| `extra_noisy` | `[]` | 출력이 많다고 볼 명령 앞부분, 예: `"./deploy.sh"`. |
| `quiet` | `[]` | 가드가 절대 건드리지 않을 명령 앞부분, 예: `"make docs"`. |
| `keep_raw` | `50` | `.squeeze/raw/`에 남길 원본 로그 수. |

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
        ▼
전체 출력 ──▶ .squeeze/raw/<시각>.log
짧은 출력 ──▶ Claude  (에러, 경고, 처음, 끝, 로그 경로가 있는 마지막 줄)
```

왜 명령을 몰래 바꾸지 않고 "거부 + 힌트"일까요? 명령을 바꿔치기하는 훅은 그 명령을 승인까지 해야 해서, 사용자의 권한 확인을 건너뛰게 됩니다. squeeze는 절대 그러지 않습니다.

더 보기: [docs/guide/how-it-works.md](../docs/guide/how-it-works.md), [docs/guide/cli.md](../docs/guide/cli.md).

## 어디에 적용되나

- **Claude Code**: 훅을 통해 자동.
- **claude.ai**: 긴 텍스트를 붙여넣을 때 `squeeze clip`을 쓰는 경우만. 평소 채팅은 영향 없음. squeeze는 채팅을 보거나 바꿀 수 없습니다.
- **다른 터미널 도구**: `squeeze compress`로 파이프.

## FAQ

**Q. Claude가 중요한 걸 놓칠 수 있나요?**
A. 모든 테스트 로그에서 서로 다른 에러·경고 메시지가 전부 남았습니다. 요약으로 부족하면 마지막 줄에 전체 로그 경로가 있어서 Claude가 구간 읽기로 볼 수 있습니다.

**Q. 내 데이터를 어딘가로 보내나요?**
A. 아니요. 네트워크 코드가 없습니다. 로그는 내 컴퓨터의 `.squeeze/`에 남습니다.

**Q. 세션이 망가질 수 있나요?**
A. 훅은 실패 시 물러납니다. 이상한 입력, 없는 파일, 패닉이 나도 아무것도 출력하지 않고 Claude Code는 평소처럼 진행합니다.

**Q. Claude가 큰 파일을 계속 막히는데, 저는 전체가 필요해요.**
A. 힌트에 `limit=<줄 수>`로 다시 읽으라고 나와 있고, 그렇게 하면 통과합니다. 아니면 `"guard_read": false`로 끄세요.

**Q. 평범한 claude.ai 채팅에서도 토큰이 줄어드나요?**
A. `squeeze clip`을 거쳐 긴 텍스트를 붙여넣을 때만 줄어듭니다.

## 피드백

개선사항과 문제점은 [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose)에 남겨주세요: v1.0에 바라는 점, 버전 업데이트나 설치 오류, 압축하면서 뭔가 빠진 경우, 그 밖의 소프트웨어 문제 모두 환영합니다. 양식(버그, 압축 문제, 설치·업데이트·버전 문제, 기능 제안) 중 하나를 고르면 됩니다. 질문과 로드맵 이야기는 [Discussions](https://github.com/jongwooish/claude-squeeze/discussions)로. 6개 언어 중 아무 언어로나 써도 됩니다.

## 기여하기

```bash
cargo test                       # 단위 테스트 (테스트 로그 검사 포함)
cargo clippy -- -D warnings
cargo run -- bench V0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures
```

새 테스트 로그를 환영합니다: 실제 `*.log`(비밀 정보와 개인 경로를 먼저 지우고)를 `testdata/fixtures/`에 추가하세요. 서로 다른 에러·경고가 하나라도 사라지면 테스트가 실패합니다.

이슈와 아이디어: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).
