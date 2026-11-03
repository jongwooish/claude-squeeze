# Benchmark: fixture replay

← [v0.1 Sentinel](../README.md)

## What is measured

`squeeze bench <dir>` feeds each `*.log` file through the same compressor that `squeeze run` uses, with default settings, and reports:

- **Lines and tokens before → after.** "After" includes the footer line that Claude would see. Tokens are estimated as bytes ÷ 4.
- **Saved**: 1 − after/before, by bytes.
- **Distinct errors/warnings kept**: every line that squeeze classifies as an error or warning is normalized (digits → 0, hex ids → #). A message counts as kept if at least one line of that shape appears in the output. Repeats are allowed to collapse into `… [same message N more times] …`.

The unit tests run the same check and fail if any distinct message is lost.

## Results (v0.1.0)

```
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures --markdown
```

| Log | Lines before → after | Tokens before → after (est.) | Saved | Distinct errors/warnings kept |
|---|---:|---:|---:|---:|
| `dpkg-list.log` | 1049 → 63 | 35,237 → 2,106 | **94.0%** | 8 / 8 |
| `go-test-fail.log` | 488 → 16 | 4,247 → 202 | **95.2%** | 4 / 4 |
| `journal-web01-synthetic.log` | 1400 → 37 | 31,161 → 997 | **96.8%** | 5 / 5 |
| `make-gcc-warnings.log` | 370 → 44 | 7,046 → 850 | **87.9%** | 7 / 7 |
| `npm-install-short.log` | 16 → 16 | 287 → 287 | **0.0%** | 12 / 12 |
| `pytest-fail.log` | 217 → 39 | 4,040 → 516 | **87.2%** | 11 / 11 |
| `tsc-errors.log` | 124 → 9 | 1,310 → 172 | **86.9%** | 3 / 3 |
| **Total** | | 83,325 → 5,128 | **93.8%** | 50 / 50 |

## The fixtures

| File | Source |
|---|---|
| `go-test-fail.log` | Real `go test -v` output: 241 subtests, one fails. |
| `pytest-fail.log` | Real `pytest -v` output: 183 tests, two fail with tracebacks. |
| `make-gcc-warnings.log` | Real `make -k` output: 30 C files with gcc `-Wall -Wextra` warnings and one compile error. |
| `tsc-errors.log` | Real `tsc` output with `listFiles`: two type errors. |
| `dpkg-list.log` | Real `dpkg -l` output from a Debian-based system. |
| `npm-install-short.log` | Real, short `npm install` failure (network error). Shows that short output is left alone. |
| `journal-web01-synthetic.log` | **Synthetic** server journal (nginx, app, cron, sshd) with an upstream timeout, DB pool errors and an OOM kill. Generated because real server logs contain private data. |

Paths that pointed to the build machine were replaced with `~/project/`.

## Why savings compound

Claude Code sends the conversation so far with every request. Text that enters the context is paid for again on each later turn (mostly as cached input, which is cheaper but not free, and it still fills the context window).

Example: a 1,400-line journal is about 31,000 tokens. Left in context for 20 more turns, that is about 620,000 input tokens. Squeezed to about 1,000 tokens, the same 20 turns cost about 20,000.

This is arithmetic on the fixture numbers, not a measurement of a live session.

## What this benchmark does not show

- **No live Claude Code A/B run yet.** The real effect depends on how often your sessions hit noisy commands. A live benchmark (same tasks, squeeze on and off, cost from Claude Code's own usage output) is planned.
- **Token counts are estimates.** Real tokenizers differ by language and content; bytes ÷ 4 is a common rough rule for English text and code.
- **"Kept" is about messages, not meaning.** squeeze keeps lines that look like errors or warnings. Output where the important part has no such words (for example, a wrong number in a table) can lose detail. The full log is always saved, and its path is in the footer.

Your real numbers: run `squeeze report` in a project after a few sessions.
