# CLI

← [가이드 목차](index.md)

## 설치

| 방법 | 명령 |
|---|---|
| macOS / Linux | `curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh \| bash` |
| Windows (PowerShell) | `irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 \| iex` |
| 소스에서 빌드 | `cargo install --git https://github.com/jongwooish/claude-squeeze squeeze` |

설치 스크립트는 릴리스의 `checksums.txt`로 SHA-256을 확인한 뒤 설치합니다.

| 환경 변수 | 뜻 |
|---|---|
| `SQUEEZE_VERSION` | 특정 버전 설치 (예: `v0.1.0`). 기본은 최신 |
| `SQUEEZE_INSTALL_DIR` | 설치 폴더. 기본은 `/usr/local/bin`, Windows는 `%LOCALAPPDATA%\Programs\squeeze` |
| `SQUEEZE_SKIP=1` | 명령 앞에 붙이면 Bash 가드가 그 명령을 건드리지 않음 |

## 명령

### `squeeze init`

```bash
squeeze init [--dir D] [--bin PATH] [--dry-run]
```

프로젝트에 설치합니다. `.claude/settings.json`(훅 2개, 기존 설정 유지, `.squeeze.bak` 백업), `.squeeze/config.json`, `.gitignore`, `CLAUDE.md`를 바꿉니다. 여러 번 실행해도 훅이 중복되지 않습니다. `squeeze`가 PATH에 없으면 `--bin`에 절대 경로를 주세요. `--dry-run`은 바꿀 내용만 보여줍니다.

`settings.json`이 올바른 JSON이 아니면 덮어쓰지 않고 멈춥니다.

### `squeeze uninstall`

```bash
squeeze uninstall [--dir D] [--purge] [--dry-run]
```

squeeze가 넣은 훅과 `CLAUDE.md` 안내만 지웁니다. 다른 훅은 그대로 둡니다. `--purge`는 `.squeeze/`(로그와 기록)까지 지웁니다. `.gitignore`의 `.squeeze/` 줄은 남깁니다.

### `squeeze run`

```bash
squeeze run -- <명령>
```

셸(`bash`, 없으면 `sh`, Windows에서 bash가 없으면 PowerShell)로 명령을 실행하고 stdout과 stderr를 순서대로 합칩니다. 출력이 길면 압축해서 보여주고 전체는 `.squeeze/raw/`에 저장합니다. **종료 코드는 원래 명령의 것을 그대로** 돌려줍니다.

### `squeeze compress`

```bash
<명령> 2>&1 | squeeze compress
```

파이프로 들어온 텍스트를 압축합니다. 다른 터미널 도구나 스크립트에서 쓸 때 편합니다.

### `squeeze clip`

```bash
squeeze clip [--print]
```

클립보드 내용을 압축본으로 바꿉니다. 긴 로그를 복사 → `squeeze clip` → claude.ai에 붙여넣기. `--print`는 클립보드를 바꾸지 않고 결과만 출력합니다. 짧은 내용은 건드리지 않습니다.

클립보드 도구: macOS `pbpaste/pbcopy`, Windows PowerShell, Linux `wl-clipboard` → `xclip` → `xsel` 순으로 찾습니다.

### `squeeze report`

```bash
squeeze report [--since 7d] [--global] [--json] [--top N]
```

컨텍스트에 들어갈 뻔한 양, 실제로 들어간 양, 아낀 양, 가장 많이 아낀 명령을 보여줍니다. `--since`는 `30m`, `24h`, `7d` 형식. `--global`은 `squeeze clip` 기록(`~/.squeeze/`)을 봅니다.

토큰은 바이트 ÷ 4로 추정한 값입니다. Read 가드의 절약량은 "파일 전체를 읽었다면"을 기준으로 한 상한값입니다.

### `squeeze bench`

```bash
squeeze bench <폴더> [--markdown]
```

폴더 안의 `*.log`를 압축기에 통과시켜 줄 수, 추정 토큰, 절약률, 에러·경고 보존 개수를 보여줍니다.

### `squeeze hook pre-bash` / `pre-read`

Claude Code가 호출하는 훅 진입점입니다. 직접 쓸 일은 없습니다. 표준 입력으로 훅 JSON을 받고, 거부할 때만 JSON을 출력합니다.

## 설정 파일 `.squeeze/config.json`

```json
{
  "compress": {
    "max_lines": 80,
    "max_bytes": 8000,
    "head_lines": 8,
    "tail_lines": 20,
    "context": 2,
    "budget": 120
  },
  "guard_bash": true,
  "guard_read": true,
  "read_max_lines": 400,
  "extra_noisy": [],
  "quiet": [],
  "keep_raw": 50
}
```

| 키 | 뜻 |
|---|---|
| `compress.max_lines`, `max_bytes` | 둘 다 이하이면 압축하지 않음 |
| `compress.head_lines`, `tail_lines` | 처음/끝에서 항상 남길 줄 수 |
| `compress.context` | 에러·경고 앞뒤로 남길 줄 수 |
| `compress.budget` | 남길 줄 수의 대략적인 상한 |
| `guard_bash`, `guard_read` | 각 가드 켜기/끄기 |
| `read_max_lines` | 이보다 긴 파일은 구간 읽기로 유도 |
| `extra_noisy` | 출력이 많다고 볼 명령 앞부분 (예: `"./deploy.sh"`) |
| `quiet` | 가드가 절대 건드리지 않을 명령 앞부분 (예: `"make docs"`) |
| `keep_raw` | `.squeeze/raw/`에 남길 로그 개수 |
