//! Reads Claude Code's own session logs (`~/.claude/projects/**/*.jsonl`) for
//! real token numbers. The format is not documented and may change, so every
//! field is optional and bad lines are skipped.

use crate::json::{self, Value};
use crate::paths;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Usage {
    pub input: u64,
    pub output: u64,
    pub cache_create: u64,
    pub cache_read: u64,
}

impl Usage {
    /// Tokens the model read for this request (= the context size).
    pub fn context(&self) -> u64 {
        self.input + self.cache_create + self.cache_read
    }
    pub fn total(&self) -> u64 {
        self.context() + self.output
    }
    pub fn add(&mut self, o: &Usage) {
        self.input += o.input;
        self.output += o.output;
        self.cache_create += o.cache_create;
        self.cache_read += o.cache_read;
    }
}

/// One assistant message with usage.
#[derive(Debug, Clone, Default)]
pub struct Turn {
    pub ts: u64,
    pub session: String,
    pub cwd: String,
    pub model: String,
    pub usage: Usage,
    /// MCP servers whose tools this message called.
    pub mcp_servers: Vec<String>,
}

fn num(v: Option<&Value>) -> u64 {
    v.and_then(|x| x.as_i64()).unwrap_or(0).max(0) as u64
}

/// "2026-11-04T05:12:01.123Z" → unix seconds (UTC).
pub fn parse_ts(s: &str) -> u64 {
    let b = s.as_bytes();
    if b.len() < 19 {
        return 0;
    }
    let n = |r: std::ops::Range<usize>| s.get(r).and_then(|x| x.parse::<i64>().ok());
    let (Some(y), Some(mo), Some(d), Some(h), Some(mi), Some(se)) =
        (n(0..4), n(5..7), n(8..10), n(11..13), n(14..16), n(17..19))
    else {
        return 0;
    };
    // days from civil (Howard Hinnant)
    let y2 = if mo <= 2 { y - 1 } else { y };
    let era = y2.div_euclid(400);
    let yoe = y2 - era * 400;
    let mp = (mo + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    (days * 86_400 + h * 3600 + mi * 60 + se).max(0) as u64
}

/// MCP tool names look like `mcp__<server>__<tool>`.
pub fn mcp_server_of(tool: &str) -> Option<&str> {
    let rest = tool.strip_prefix("mcp__")?;
    let end = rest.find("__")?;
    Some(&rest[..end])
}

/// Claude Code's normalization of server names inside tool names.
pub fn normalize_server(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Parse one transcript line. Returns the message id and the turn.
pub fn parse_line(line: &str) -> Option<(String, Turn)> {
    if !line.contains("\"assistant\"") || !line.contains("\"usage\"") {
        return None;
    }
    let v = json::parse(line).ok()?;
    if v.get("type")?.as_str()? != "assistant" {
        return None;
    }
    let m = v.get("message")?;
    let u = m.get("usage")?;
    let usage = Usage {
        input: num(u.get("input_tokens")),
        output: num(u.get("output_tokens")),
        cache_create: num(u.get("cache_creation_input_tokens")),
        cache_read: num(u.get("cache_read_input_tokens")),
    };
    if usage.total() == 0 {
        return None;
    }
    let id = m
        .get("id")
        .and_then(|x| x.as_str())
        .map(str::to_string)
        .or_else(|| v.get("uuid").and_then(|x| x.as_str()).map(str::to_string))?;
    let req = v.get("requestId").and_then(|x| x.as_str()).unwrap_or("");
    let mut mcp = Vec::new();
    if let Some(Value::Arr(content)) = m.get("content") {
        for c in content {
            if c.get("type").and_then(|t| t.as_str()) == Some("tool_use") {
                if let Some(s) = c
                    .get("name")
                    .and_then(|n| n.as_str())
                    .and_then(mcp_server_of)
                {
                    if !mcp.iter().any(|x: &String| x == s) {
                        mcp.push(s.to_string());
                    }
                }
            }
        }
    }
    let s = |k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    Some((
        format!("{}:{}", id, req),
        Turn {
            ts: parse_ts(&s("timestamp")),
            session: s("sessionId"),
            cwd: s("cwd"),
            model: m
                .get("model")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            usage,
            mcp_servers: mcp,
        },
    ))
}

/// All transcript files, newest first.
pub fn files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for d in paths::transcript_dirs() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        for p in rd.flatten().map(|e| e.path()).filter(|p| p.is_dir()) {
            let Ok(inner) = std::fs::read_dir(&p) else {
                continue;
            };
            for f in inner.flatten().map(|e| e.path()) {
                if f.extension().is_some_and(|e| e == "jsonl") {
                    out.push(f);
                }
            }
        }
    }
    out.sort_by_key(|f| {
        std::cmp::Reverse(
            std::fs::metadata(f)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0),
        )
    });
    out
}

/// Read turns from the given files, keeping one record per message.
pub fn read_turns(files: &[PathBuf], since: u64) -> Vec<Turn> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for f in files {
        if since > 0 {
            let modified = std::fs::metadata(f)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(u64::MAX);
            if modified < since {
                continue;
            }
        }
        let Ok(text) = std::fs::read_to_string(f) else {
            continue;
        };
        for line in text.lines() {
            if let Some((key, t)) = parse_line(line) {
                if t.ts >= since && seen.insert(key) {
                    out.push(t);
                }
            }
        }
    }
    out.sort_by_key(|t| t.ts);
    out
}

/// Context size of the newest assistant message in a transcript, reading
/// only the end of the file.
pub fn context_tokens(transcript: &Path) -> Option<u64> {
    let mut f = std::fs::File::open(transcript).ok()?;
    let len = f.metadata().ok()?.len();
    let mut start = len.saturating_sub(512 * 1024);
    loop {
        f.seek(SeekFrom::Start(start)).ok()?;
        let mut buf = Vec::new();
        f.by_ref().take(len - start).read_to_end(&mut buf).ok()?;
        let text = String::from_utf8_lossy(&buf);
        let mut lines: Vec<&str> = text.lines().collect();
        if start > 0 && !lines.is_empty() {
            lines.remove(0); // probably cut in half
        }
        for l in lines.iter().rev() {
            if let Some((_, t)) = parse_line(l) {
                return Some(t.usage.context());
            }
        }
        if start == 0 {
            return None;
        }
        start = start.saturating_sub(4 * 1024 * 1024);
    }
}

/// Claude Code's project folder name for a working directory.
pub fn encode_project(cwd: &str) -> String {
    cwd.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

/// Transcript files that belong to a project directory.
pub fn project_files(root: &Path) -> Vec<PathBuf> {
    let enc = encode_project(&root.to_string_lossy());
    let mut out = Vec::new();
    for d in paths::transcript_dirs() {
        let p = d.join(&enc);
        if let Ok(rd) = std::fs::read_dir(&p) {
            out.extend(
                rd.flatten()
                    .map(|e| e.path())
                    .filter(|f| f.extension().is_some_and(|e| e == "jsonl")),
            );
        }
    }
    out
}

/// Last time each MCP server (normalized name) was used in a project.
pub fn mcp_last_used(root: &Path) -> HashMap<String, u64> {
    let mut m = HashMap::new();
    for t in read_turns(&project_files(root), 0) {
        for s in &t.mcp_servers {
            let e = m.entry(s.clone()).or_insert(0);
            *e = (*e).max(t.ts);
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = r#"{"type":"assistant","sessionId":"s1","cwd":"/p","timestamp":"2026-11-04T05:00:00.000Z","requestId":"r1","message":{"id":"m1","model":"claude-x","content":[{"type":"tool_use","name":"mcp__github__search","input":{}},{"type":"text","text":"hi"}],"usage":{"input_tokens":10,"output_tokens":50,"cache_creation_input_tokens":100,"cache_read_input_tokens":20000}}}"#;

    #[test]
    fn parses_usage_and_mcp() {
        let (k, t) = parse_line(LINE).unwrap();
        assert_eq!(k, "m1:r1");
        assert_eq!(t.usage.context(), 20110);
        assert_eq!(t.usage.total(), 20160);
        assert_eq!(t.mcp_servers, vec!["github"]);
        assert_eq!(t.ts, 1_793_768_400);
        assert!(parse_line(r#"{"type":"user","message":{}}"#).is_none());
    }

    #[test]
    fn context_from_tail_and_dedupe() {
        let d = std::env::temp_dir().join(format!("squeeze-tr-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&d);
        let f = d.join("t.jsonl");
        let mut s = String::new();
        for i in 0..3000 {
            s += &format!(
                "{{\"type\":\"user\",\"message\":{{\"content\":\"line {}\"}}}}\n",
                i
            );
        }
        s += LINE;
        s.push('\n');
        s += LINE; // duplicate line of the same message
        s.push('\n');
        std::fs::write(&f, &s).unwrap();
        assert_eq!(context_tokens(&f), Some(20110));
        assert_eq!(read_turns(std::slice::from_ref(&f), 0).len(), 1);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn names() {
        assert_eq!(mcp_server_of("mcp__my-server__do_it"), Some("my-server"));
        assert_eq!(mcp_server_of("Bash"), None);
        assert_eq!(normalize_server("my server.v2"), "my_server_v2");
        assert_eq!(encode_project("/home/me/my.app"), "-home-me-my-app");
        assert_eq!(encode_project("C:\\Users\\me"), "C--Users-me");
    }
}
