# 설치와 첫 연결

← [가이드 목차](index.md)

## 1. 설치

[최신 릴리스](https://github.com/jongwooish/claude-squeeze/releases/latest)에서 내 컴퓨터에 맞는 파일을 받습니다.

| 시스템 | 파일 |
|---|---|
| Windows 10/11 (대부분의 PC) | `squeeze-Auditor-0.2.0-setup-x64.exe` |
| Windows ARM (Surface Pro X 등) | `squeeze-Auditor-0.2.0-setup-arm64.exe` |
| macOS (M1 이후) | `squeeze-Auditor-0.2.0-macos-arm64.pkg` |
| macOS (Intel) | `squeeze-Auditor-0.2.0-macos-x86_64.pkg` |
| Ubuntu / Debian | `squeeze-auditor_0.2.0_amd64.deb` (ARM은 `_arm64.deb`) |

### 경고 창이 뜰 때

아직 코드 서명이 없어서 처음 한 번 막힐 수 있습니다. 경고를 넘기기 전에 릴리스 페이지의 `checksums.txt`(SHA-256)와 파일이 같은지 확인하세요.

- **Windows SmartScreen** ("Windows의 PC 보호"): **추가 정보** → **실행**.
- **Windows 스마트 앱 컨트롤**: 그래도 실행하는 버튼이 없고, 설치한 뒤 `squeeze.exe`도 막습니다. 가상 머신이나 Windows 샌드박스에서 쓰거나, Windows 보안 → 앱 및 브라우저 컨트롤 → 스마트 앱 컨트롤 설정에서 끄세요 (Windows 11 25H2 이상은 나중에 다시 켤 수 있습니다).
- **macOS** ("열 수 없음", "확인되지 않은 개발자"): 시스템 설정 → 개인정보 보호 및 보안 → **그래도 열기**. "손상되었기 때문에 열 수 없습니다"가 뜨면 `xattr -dr com.apple.quarantine /Applications/squeeze-Auditor.app`.
- **Linux**: `.deb`는 `sudo apt install ./파일.deb`로 설치하고, `.tar.gz` 프로그램이 "Permission denied"면 `chmod +x squeeze squeeze-auditor`.

명령어까지 전부 정리한 내용: [시스템이 막을 때](../../README/README.ko.md#시스템이-막을-때).

### Windows 설치 파일이 하는 일

- 관리자 권한 없이 `%LOCALAPPDATA%\Programs\squeeze`에 설치합니다 (v0.1 `install.ps1`과 같은 폴더라 덮어쓰면 업그레이드).
- 시작 메뉴에 **squeeze-Auditor** 아이콘, 원하면 바탕 화면 아이콘.
- 터미널에서 `squeeze`를 쓸 수 있게 사용자 PATH에 추가.
- 마지막 화면의 **지금 Claude Code에 연결** 체크를 그대로 두면 바로 연결됩니다.

## 2. 연결

squeeze-Auditor를 열고 **연결된 환경** 탭(또는 대시보드의 안내)에서:

- **Claude Code 연결**: `~/.claude/settings.json`에 훅 6개를 넣습니다. 모든 프로젝트에 적용되고, 원래 설정과 다른 훅은 그대로 둡니다. 바꾸기 전 원본은 `~/.squeeze/backups/`에 복사합니다. 이미 열려 있는 Claude Code는 다시 시작하세요.
- **데스크톱 앱 연결** (Windows, macOS): `claude_desktop_config.json`에 squeeze 도구를 추가합니다. 그 아래 **읽을 수 있는 폴더**에 로그 폴더를 적고 저장한 뒤, Claude 데스크톱 앱을 완전히 종료했다가 다시 켭니다.

터미널에서는 `squeeze setup`(연결), `squeeze setup --remove`(해제).

## 3. v0.1에서 올라올 때

- 그냥 설치하고 한 번 연결하면 됩니다.
- `squeeze init`을 했던 프로젝트도 계속 동작합니다. 훅이 두 번 실행돼도 같은 요청은 한 번으로 처리하니 문제없고, 깔끔하게 하려면 그 프로젝트에서 `squeeze uninstall`을 실행하세요.
- 기록(`.squeeze/ledger.jsonl`)은 v0.2부터 `~/.squeeze/ledger.jsonl` 한 곳에 모입니다. 예전 프로젝트 기록도 대시보드에 함께 보입니다.

## 4. 제거

- **Windows**: 설정 → 앱 → squeeze-Auditor → 제거. Claude Code와 데스크톱 앱 연결도 함께 해제합니다.
- **macOS / Linux**: 먼저 `squeeze setup --remove`, 그다음 macOS는 응용 프로그램에서 squeeze-Auditor를 휴지통으로(그리고 `/usr/local/bin/squeeze` 삭제), Linux는 `sudo apt remove squeeze-auditor`.
- 기록과 설정까지 지우려면 `~/.squeeze/` 폴더를 삭제하세요.
