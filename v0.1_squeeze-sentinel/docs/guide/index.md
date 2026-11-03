# squeeze 사용자 가이드 (v0.1 Sentinel)

← [v0.1 Sentinel README](../../README/README.ko.md)

- [동작 원리](how-it-works.md): 훅이 언제 끼어드는지, 압축 규칙, 왜 "거부 + 힌트" 방식인지
- [CLI](cli.md): 모든 명령, 옵션, 설정 파일, 환경 변수
- [벤치마크](../bench.md) (영어): 측정 방법과 결과, 한계

## 3줄 요약

1. `squeeze init`을 프로젝트에서 한 번 실행하면 Claude Code가 긴 출력을 내는 명령을 `squeeze run`으로 다시 실행하게 됩니다.
2. Claude는 에러·경고·처음과 끝만 받고, 전체 로그는 `.squeeze/raw/`에 남습니다.
3. `squeeze report`로 얼마나 아꼈는지 확인합니다.
