//! Claude Code PreToolUse hooks.
//!
//! squeeze never approves anything for the user. A hook either stays silent
//! (Claude Code's normal permission flow continues) or denies the call with a
//! short reason that tells Claude what to run instead. Any error fails open:
//! no output, exit 0.

use crate::config::{self, Config};
use crate::json::{self, Value};
use crate::ledger::{self, Event};
use crate::outline;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn deny(w: &mut dyn Write, reason: &str) {
    let mut h = Value::obj();
    h.set("hookEventName", Value::str("PreToolUse"));
    h.set("permissionDecision", Value::str("deny"));
    h.set("permissionDecisionReason", Value::str(reason));
    let mut o = Value::obj();
    o.set("hookSpecificOutput", h);
    let _ = writeln!(w, "{}", o.to_compact());
}

/// Split a command line into simple commands at ; && || | and newlines.
fn segments(cmd: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for part in cmd.split(['\n', ';', '|', '&']) {
        let toks: Vec<String> = part
            .split_whitespace()
            .map(|t| {
                t.trim_matches(|c| c == '(' || c == ')' || c == '{' || c == '}')
                    .to_string()
            })
            .filter(|t| !t.is_empty())
            .collect();
        if !toks.is_empty() {
            out.push(toks);
        }
    }
    out
}

/// Drop env assignments and wrappers (sudo, time, npx) to reach the program.
fn program(toks: &[String]) -> (String, Vec<String>) {
    let mut i = 0;
    while i < toks.len() {
        let t = &toks[i];
        if t.contains('=') && !t.starts_with('-')
            || matches!(
                t.as_str(),
                "sudo" | "time" | "nice" | "env" | "npx" | "bunx" | "exec"
            )
        {
            i += 1;
            continue;
        }
        if (t == "python" || t == "python3") && toks.get(i + 1).map(|s| s.as_str()) == Some("-m") {
            i += 2;
            continue;
        }
        break;
    }
    match toks.get(i) {
        None => (String::new(), Vec::new()),
        Some(p) => {
            let name = p.rsplit('/').next().unwrap_or(p).to_string();
            (name, toks[i + 1..].to_vec())
        }
    }
}

fn noisy_simple(prog: &str, args: &[String]) -> bool {
    let a0 = args.first().map(|s| s.as_str()).unwrap_or("");
    let a1 = args.get(1).map(|s| s.as_str()).unwrap_or("");
    match prog {
        "npm" | "pnpm" | "yarn" | "bun" => {
            matches!(
                a0,
                "i" | "install" | "ci" | "add" | "update" | "build" | "test" | "t"
            ) || (a0 == "run" && matches!(a1, "build" | "test"))
                || (prog == "yarn" && a0.is_empty())
        }
        "pip" | "pip3" => a0 == "install",
        "uv" => (a0 == "pip" && a1 == "install") || a0 == "sync",
        "poetry" => a0 == "install",
        "go" => matches!(a0, "test" | "build" | "vet") || (a0 == "mod" && a1 == "download"),
        "cargo" => matches!(a0, "build" | "test" | "check" | "clippy"),
        "mvn" | "gradle" | "gradlew" | "make" | "cmake" | "ninja" => true,
        "docker" => {
            matches!(a0, "build" | "logs")
                || (a0 == "compose" && matches!(a1, "up" | "build" | "logs"))
        }
        "docker-compose" => matches!(a0, "up" | "build" | "logs"),
        "apt" | "apt-get" => {
            matches!(
                a0,
                "install" | "update" | "upgrade" | "full-upgrade" | "dist-upgrade"
            ) || args
                .iter()
                .any(|x| matches!(x.as_str(), "install" | "update" | "upgrade"))
        }
        "dnf" | "yum" => args
            .iter()
            .any(|x| matches!(x.as_str(), "install" | "update" | "upgrade")),
        "brew" => matches!(a0, "install" | "upgrade"),
        "journalctl" | "dmesg" => true,
        "pytest" | "jest" | "vitest" | "mocha" | "tsc" | "eslint" | "ruff" | "mypy" => true,
        "kubectl" => matches!(a0, "logs" | "describe") || (a0 == "get" && a1 == "events"),
        "ansible-playbook" => true,
        "terraform" => matches!(a0, "plan" | "apply" | "init"),
        "git" => a0 == "log",
        _ => false,
    }
}

/// Output is already bounded (pipe to head/tail/grep, -n 50, redirect...).
fn bounded(cmd: &str, segs: &[Vec<String>]) -> bool {
    let compact = cmd.replace(' ', "");
    for p in [
        "|head", "|tail", "|grep", "|wc", "|rg", "|less", "|more", "|jq", "|sort", "|uniq",
    ] {
        if compact.contains(p) {
            return true;
        }
    }
    if cmd.contains(" > ")
        || cmd.contains(" >> ")
        || cmd.contains(">/dev/null")
        || cmd.contains("> /dev/null")
    {
        return true;
    }
    segs.iter().flatten().any(|t| {
        t == "-q"
            || t == "--quiet"
            || t == "--version"
            || t == "--help"
            || t == "-h"
            || t.starts_with("--max-count")
            || t.starts_with("-n") && t.len() > 2 && t[2..].chars().all(|c| c.is_ascii_digit())
            || t.starts_with('-') && t.len() > 1 && t[1..].chars().all(|c| c.is_ascii_digit())
    }) || segs.iter().any(|s| {
        s.windows(2).any(|w| {
            (w[0] == "-n" || w[0] == "--lines") && w[1].chars().all(|c| c.is_ascii_digit())
        })
    })
}

/// Should the guard redirect this command to `squeeze run`?
pub fn is_noisy(cmd: &str, cfg: &Config) -> bool {
    let c = cmd.trim();
    if c.is_empty() || c.contains("squeeze ") || c.starts_with("SQUEEZE_SKIP=1") {
        return false;
    }
    if cfg
        .quiet
        .iter()
        .any(|q| !q.is_empty() && c.starts_with(q.as_str()))
    {
        return false;
    }
    if cfg
        .extra_noisy
        .iter()
        .any(|q| !q.is_empty() && c.starts_with(q.as_str()))
    {
        return true;
    }
    let segs = segments(c);
    if bounded(c, &segs) {
        return false;
    }
    segs.iter().any(|s| {
        let (p, a) = program(s);
        noisy_simple(&p, &a)
    })
}

fn read_input(r: &mut dyn Read) -> Option<Value> {
    let mut s = String::new();
    r.read_to_string(&mut s).ok()?;
    json::parse(&s).ok()
}

fn cwd_of(v: &Value) -> PathBuf {
    v.get("cwd")
        .and_then(|c| c.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// POSIX single-quote a string.
pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

pub fn pre_bash(r: &mut dyn Read, w: &mut dyn Write) {
    let Some(v) = read_input(r) else { return };
    if v.get("tool_name").and_then(|t| t.as_str()) != Some("Bash") {
        return;
    }
    let Some(cmd) = v
        .get("tool_input")
        .and_then(|t| t.get("command"))
        .and_then(|c| c.as_str())
    else {
        return;
    };
    let root = config::root(&cwd_of(&v));
    let cfg = config::load(&root);
    if !cfg.guard_bash || !is_noisy(cmd, &cfg) {
        return;
    }
    deny(
        w,
        &format!(
            "squeeze: this command usually prints long output. Run it again as:\n  squeeze run -- {}\nYou get the errors, warnings, first and last lines, and the path of the full log. If you really need the raw output, prefix the command with SQUEEZE_SKIP=1.",
            shell_quote(cmd)
        ),
    );
    let _ = ledger::append(
        &root.join(config::DIR),
        Event {
            kind: "guard-bash".into(),
            subject: cmd.into(),
            ..Default::default()
        },
    );
}

pub fn pre_read(r: &mut dyn Read, w: &mut dyn Write) {
    let Some(v) = read_input(r) else { return };
    if v.get("tool_name").and_then(|t| t.as_str()) != Some("Read") {
        return;
    }
    let Some(ti) = v.get("tool_input") else {
        return;
    };
    if ti.get("offset").is_some() || ti.get("limit").is_some() {
        return;
    }
    let Some(fp) = ti.get("file_path").and_then(|p| p.as_str()) else {
        return;
    };
    let cwd = cwd_of(&v);
    let root = config::root(&cwd);
    let cfg = config::load(&root);
    if !cfg.guard_read {
        return;
    }
    let path = if Path::new(fp).is_absolute() {
        PathBuf::from(fp)
    } else {
        cwd.join(fp)
    };
    let Ok((lines, size, text)) = outline::stat(&path) else {
        return;
    };
    if !text || lines <= cfg.read_max_lines {
        return;
    }
    let Ok(es) = outline::build(&path, 40) else {
        return;
    };
    let reason = format!(
        "squeeze: {} has {} lines. Read only the part you need with offset and limit (for example offset=1, limit=200). Outline (line numbers):\n{}To read the whole file anyway, call Read again with limit={}.",
        fp,
        lines,
        outline::format(&es),
        lines
    );
    deny(w, &reason);
    let _ = ledger::append(
        &root.join(config::DIR),
        Event {
            kind: "guard-read".into(),
            subject: fp.into(),
            in_bytes: size as usize,
            out_bytes: reason.len(),
            in_lines: lines,
            out_lines: es.len(),
            ..Default::default()
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noisy_detection() {
        let cfg = Config::default();
        for c in [
            "npm install",
            "go test ./...",
            "sudo apt-get install -y nginx",
            "journalctl -u nginx",
            "python -m pytest -v",
            "make",
            "git log",
            "docker compose up --build",
            "cd web && npm ci",
            "npx tsc -p .",
        ] {
            assert!(is_noisy(c, &cfg), "expected noisy: {}", c);
        }
        for c in [
            "ls -la",
            "git status",
            "npm install 2>&1 | tail -20",
            "journalctl -u nginx -n 50",
            "squeeze run -- npm test",
            "SQUEEZE_SKIP=1 npm test",
            "go version",
            "npm test > out.txt",
            "git log --oneline -5",
            "cargo --version",
        ] {
            assert!(!is_noisy(c, &cfg), "expected quiet: {}", c);
        }
        let mut cfg = Config::default();
        cfg.quiet.push("make docs".into());
        cfg.extra_noisy.push("./deploy.sh".into());
        assert!(!is_noisy("make docs", &cfg));
        assert!(is_noisy("./deploy.sh staging", &cfg));
    }

    fn payload(tool: &str, input: &str, cwd: &Path) -> String {
        format!(
            r#"{{"hook_event_name":"PreToolUse","cwd":{},"tool_name":"{}","tool_input":{}}}"#,
            Value::str(cwd.to_string_lossy()).to_compact(),
            tool,
            input
        )
    }

    fn decision(out: &[u8]) -> Option<(String, String)> {
        let s = String::from_utf8_lossy(out);
        if s.trim().is_empty() {
            return None;
        }
        let v = json::parse(s.trim()).unwrap();
        let h = v.get("hookSpecificOutput").unwrap();
        Some((
            h.get("permissionDecision")?.as_str()?.into(),
            h.get("permissionDecisionReason")?.as_str()?.into(),
        ))
    }

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("squeeze-hook-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join(config::DIR)).unwrap();
        d
    }

    #[test]
    fn pre_bash_denies_noisy_never_allows() {
        let d = tmp("bash");
        let mut out = Vec::new();
        pre_bash(
            &mut payload("Bash", r#"{"command":"npm install"}"#, &d).as_bytes(),
            &mut out,
        );
        let (dec, reason) = decision(&out).unwrap();
        assert_eq!(dec, "deny");
        assert!(
            reason.contains("squeeze run -- 'npm install'"),
            "{}",
            reason
        );
        let mut out = Vec::new();
        pre_bash(
            &mut payload("Bash", r#"{"command":"ls"}"#, &d).as_bytes(),
            &mut out,
        );
        assert!(decision(&out).is_none());
        // garbage input fails open
        let mut out = Vec::new();
        pre_bash(&mut "not json".as_bytes(), &mut out);
        assert!(out.is_empty());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn pre_bash_respects_config() {
        let d = tmp("cfg");
        std::fs::write(d.join(".squeeze/config.json"), r#"{"guard_bash": false}"#).unwrap();
        let mut out = Vec::new();
        pre_bash(
            &mut payload("Bash", r#"{"command":"npm install"}"#, &d).as_bytes(),
            &mut out,
        );
        assert!(out.is_empty());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn pre_read_large_file() {
        let d = tmp("read");
        let p = d.join("big.rs");
        let mut s = String::new();
        for i in 0..900 {
            if i % 100 == 0 {
                s += &format!("fn f{}() {{\n", i);
            } else {
                s += "    x += 1;\n";
            }
        }
        std::fs::write(&p, s).unwrap();
        let fp = Value::str(p.to_string_lossy()).to_compact();
        let mut out = Vec::new();
        pre_read(
            &mut payload("Read", &format!(r#"{{"file_path":{}}}"#, fp), &d).as_bytes(),
            &mut out,
        );
        let (dec, reason) = decision(&out).unwrap();
        assert_eq!(dec, "deny");
        assert!(
            reason.contains("fn f500()") && reason.contains("limit=900"),
            "{}",
            reason
        );
        let mut out = Vec::new();
        pre_read(
            &mut payload(
                "Read",
                &format!(r#"{{"file_path":{},"offset":1,"limit":50}}"#, fp),
                &d,
            )
            .as_bytes(),
            &mut out,
        );
        assert!(out.is_empty());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn quoting() {
        assert_eq!(shell_quote("echo 'hi'"), r"'echo '\''hi'\'''");
    }
}
