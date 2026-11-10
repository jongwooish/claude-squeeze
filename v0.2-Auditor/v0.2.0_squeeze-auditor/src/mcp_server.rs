//! `squeeze mcp`: a small read-only MCP server for the Claude desktop app.
//! Claude can read log files already squeezed (instead of a pasted wall of
//! text), and save or load handoff notes to move a long chat into a new one.
//! Only folders allowed in squeeze-Auditor can be read. Nothing is written
//! except handoff notes in `~/.squeeze/handoff/desktop/`.

use crate::compress;
use crate::config::{self, Config};
use crate::json::{self, Value};
use crate::ledger::{self, Event};
use crate::paths;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

const PROTOCOL: &str = "2025-06-18";

fn tool(name: &str, desc: &str, props: &[(&str, &str, &str)], required: &[&str]) -> Value {
    let mut p = Value::obj();
    for (k, ty, d) in props {
        let mut x = Value::obj();
        x.set("type", Value::str(*ty));
        x.set("description", Value::str(*d));
        p.set(k, x);
    }
    let mut schema = Value::obj();
    schema.set("type", Value::str("object"));
    schema.set("properties", p);
    schema.set(
        "required",
        Value::Arr(required.iter().map(|r| Value::str(*r)).collect()),
    );
    let mut t = Value::obj();
    t.set("name", Value::str(name));
    t.set("description", Value::str(desc));
    t.set("inputSchema", schema);
    t
}

/// Kept short on purpose: tool descriptions ride along on every message.
pub fn tools() -> Value {
    Value::Arr(vec![
        tool(
            "read_log",
            "Read a local text/log file, squeezed: errors, warnings, head and tail are kept. Use offset/limit for exact lines.",
            &[("path", "string", "File path"), ("offset", "integer", "First line (1-based)"), ("limit", "integer", "Number of lines")],
            &["path"],
        ),
        tool(
            "tail_log",
            "Last lines of a local log file, squeezed.",
            &[("path", "string", "File path"), ("lines", "integer", "How many lines (default 300)")],
            &["path"],
        ),
        tool(
            "search_log",
            "Find lines containing text in a local file, with line numbers.",
            &[("path", "string", "File path"), ("text", "string", "Text to find (case-insensitive)")],
            &["path", "text"],
        ),
        tool(
            "list_logs",
            "List files in an allowed folder.",
            &[("path", "string", "Folder path (optional)")],
            &[],
        ),
        tool(
            "save_handoff",
            "Save a handoff note (goal, decisions, next steps) so a long chat can continue in a new, smaller chat.",
            &[("title", "string", "Short title"), ("content", "string", "The note in Markdown")],
            &["content"],
        ),
        tool(
            "load_handoff",
            "Load the latest saved handoff note to continue earlier work.",
            &[],
            &[],
        ),
    ])
}

fn allowed(cfg: &Config) -> Vec<PathBuf> {
    cfg.desktop
        .allowed_dirs
        .iter()
        .filter_map(|d| std::fs::canonicalize(d).ok())
        .collect()
}

fn check_path(cfg: &Config, p: &str) -> Result<PathBuf, String> {
    let dirs = allowed(cfg);
    if dirs.is_empty() {
        return Err("No folders are allowed yet. Open squeeze-Auditor → Connections → Desktop app and add the folders Claude may read.".into());
    }
    let path = std::fs::canonicalize(p).map_err(|e| format!("{}: {}", p, e))?;
    if dirs.iter().any(|d| path.starts_with(d)) {
        Ok(path)
    } else {
        Err(format!(
            "{} is outside the allowed folders ({}). Add its folder in squeeze-Auditor first.",
            p,
            dirs.iter()
                .map(|d| paths::tilde(d))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

fn read_text(p: &Path) -> Result<String, String> {
    let b = std::fs::read(p).map_err(|e| e.to_string())?;
    if b.iter().take(8000).any(|c| *c == 0) {
        return Err("binary file".into());
    }
    if b.len() > 64 * 1024 * 1024 {
        return Err("file is larger than 64 MB".into());
    }
    Ok(String::from_utf8_lossy(&b).into_owned())
}

fn squeeze_text(raw: &str, subject: &str, cfg: &Config) -> String {
    let r = compress::compress(raw, &cfg.compress);
    if !r.compressed {
        return raw.to_string();
    }
    let _ = ledger::record(
        None,
        Event {
            kind: "desktop-read".into(),
            subject: subject.into(),
            in_bytes: r.in_bytes,
            out_bytes: r.out_bytes,
            in_lines: r.in_lines,
            out_lines: r.out_lines,
            ..Default::default()
        },
    );
    format!("{}{}\n", r.text, compress::footer(&r, None, None))
}

fn arg_str<'a>(a: &'a Value, k: &str) -> Option<&'a str> {
    a.get(k).and_then(|x| x.as_str())
}
fn arg_num(a: &Value, k: &str) -> Option<usize> {
    a.get(k)
        .and_then(|x| x.as_i64())
        .filter(|n| *n > 0)
        .map(|n| n as usize)
}

fn handoff_dir() -> PathBuf {
    paths::squeeze_home().join("handoff").join("desktop")
}

/// Run one tool. Returns (text, is_error).
pub fn call(name: &str, a: &Value, cfg: &Config) -> (String, bool) {
    let r: Result<String, String> = (|| match name {
        "read_log" => {
            let p = check_path(cfg, arg_str(a, "path").ok_or("path is required")?)?;
            let text = read_text(&p)?;
            if arg_num(a, "offset").is_some() || arg_num(a, "limit").is_some() {
                let off = arg_num(a, "offset").unwrap_or(1);
                let lim = arg_num(a, "limit").unwrap_or(400).min(2000);
                return Ok(text
                    .lines()
                    .enumerate()
                    .skip(off - 1)
                    .take(lim)
                    .map(|(i, l)| format!("{:>6}\t{}\n", i + 1, l))
                    .collect());
            }
            Ok(squeeze_text(&text, &paths::tilde(&p), cfg))
        }
        "tail_log" => {
            let p = check_path(cfg, arg_str(a, "path").ok_or("path is required")?)?;
            let text = read_text(&p)?;
            let n = arg_num(a, "lines").unwrap_or(300).min(5000);
            let lines: Vec<&str> = text.lines().collect();
            let tail = lines[lines.len().saturating_sub(n)..].join("\n");
            Ok(squeeze_text(&tail, &paths::tilde(&p), cfg))
        }
        "search_log" => {
            let p = check_path(cfg, arg_str(a, "path").ok_or("path is required")?)?;
            let needle = arg_str(a, "text").ok_or("text is required")?.to_lowercase();
            let text = read_text(&p)?;
            let hits: Vec<String> = text
                .lines()
                .enumerate()
                .filter(|(_, l)| l.to_lowercase().contains(&needle))
                .take(200)
                .map(|(i, l)| format!("{:>6}\t{}", i + 1, l.chars().take(400).collect::<String>()))
                .collect();
            Ok(if hits.is_empty() {
                "No matches.".into()
            } else {
                hits.join("\n")
            })
        }
        "list_logs" => {
            let dirs = allowed(cfg);
            let target = match arg_str(a, "path") {
                Some(p) => vec![check_path(cfg, p)?],
                None => dirs.clone(),
            };
            if target.is_empty() {
                return Err(check_path(cfg, "").unwrap_err());
            }
            let mut out = String::new();
            for d in target {
                out += &format!("{}\n", paths::tilde(&d));
                let mut es: Vec<_> = std::fs::read_dir(&d)
                    .map_err(|e| e.to_string())?
                    .flatten()
                    .collect();
                es.sort_by_key(|e| e.file_name());
                for e in es.into_iter().take(300) {
                    let m = e.metadata().ok();
                    let kind = if m.as_ref().is_some_and(|m| m.is_dir()) {
                        "dir "
                    } else {
                        "file"
                    };
                    out += &format!(
                        "  {} {:>10}  {}\n",
                        kind,
                        m.map(|m| compress::human(m.len() as usize))
                            .unwrap_or_default(),
                        e.file_name().to_string_lossy()
                    );
                }
            }
            Ok(out)
        }
        "save_handoff" => {
            let content = arg_str(a, "content").ok_or("content is required")?;
            let title: String = arg_str(a, "title")
                .unwrap_or("handoff")
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '-' })
                .take(40)
                .collect();
            let p = handoff_dir().join(format!(
                "{}-{}.md",
                crate::runner::stamp(ledger::now()),
                title
            ));
            paths::write_atomic(&p, content).map_err(|e| e.to_string())?;
            Ok(format!(
                "Saved {}. Start a new chat and ask to load the handoff to continue with only this note.",
                paths::tilde(&p)
            ))
        }
        "load_handoff" => {
            let mut notes: Vec<PathBuf> = std::fs::read_dir(handoff_dir())
                .map_err(|_| "No handoff notes saved yet.".to_string())?
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "md"))
                .collect();
            notes.sort();
            let p = notes.pop().ok_or("No handoff notes saved yet.")?;
            let text = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
            let _ = ledger::record(
                None,
                Event {
                    kind: "handoff".into(),
                    subject: paths::tilde(&p),
                    out_bytes: text.len(),
                    ..Default::default()
                },
            );
            Ok(format!("Handoff note {}:\n\n{}", paths::tilde(&p), text))
        }
        other => Err(format!("unknown tool {}", other)),
    })();
    match r {
        Ok(t) => (t, false),
        Err(e) => (e, true),
    }
}

fn reply(w: &mut dyn Write, id: &Value, result: Value) {
    let mut v = Value::obj();
    v.set("jsonrpc", Value::str("2.0"));
    v.set("id", id.clone());
    v.set("result", result);
    let _ = writeln!(w, "{}", v.to_compact());
    let _ = w.flush();
}

fn error(w: &mut dyn Write, id: &Value, code: i64, msg: &str) {
    let mut e = Value::obj();
    e.set("code", Value::num(code));
    e.set("message", Value::str(msg));
    let mut v = Value::obj();
    v.set("jsonrpc", Value::str("2.0"));
    v.set("id", id.clone());
    v.set("error", e);
    let _ = writeln!(w, "{}", v.to_compact());
    let _ = w.flush();
}

/// Handle one JSON-RPC message.
pub fn handle(line: &str, w: &mut dyn Write) {
    let Ok(msg) = json::parse(line) else { return };
    let Some(id) = msg.get("id").cloned() else {
        return; // notification
    };
    let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(Value::obj());
    match method {
        "initialize" => {
            let pv = params
                .get("protocolVersion")
                .and_then(|p| p.as_str())
                .unwrap_or(PROTOCOL)
                .to_string();
            let mut caps = Value::obj();
            caps.set("tools", Value::obj());
            let mut info = Value::obj();
            info.set("name", Value::str("squeeze"));
            info.set("version", Value::str(env!("CARGO_PKG_VERSION")));
            let mut r = Value::obj();
            r.set("protocolVersion", Value::str(pv));
            r.set("capabilities", caps);
            r.set("serverInfo", info);
            r.set(
                "instructions",
                Value::str("Use read_log/tail_log for log files instead of asking the user to paste them. When a chat gets long, offer save_handoff and continue in a new chat with load_handoff."),
            );
            reply(w, &id, r);
        }
        "ping" => reply(w, &id, Value::obj()),
        "tools/list" => {
            let mut r = Value::obj();
            r.set("tools", tools());
            reply(w, &id, r);
        }
        "tools/call" => {
            let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(Value::obj());
            let cfg = config::load_global();
            let (text, is_err) = call(name, &args, &cfg);
            let mut c = Value::obj();
            c.set("type", Value::str("text"));
            c.set("text", Value::str(text));
            let mut r = Value::obj();
            r.set("content", Value::Arr(vec![c]));
            if is_err {
                r.set("isError", Value::Bool(true));
            }
            reply(w, &id, r);
        }
        _ => error(w, &id, -32601, "method not found"),
    }
}

pub fn serve() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines().map_while(Result::ok) {
        if !line.trim().is_empty() {
            handle(&line, &mut stdout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::tmp_project;

    fn rpc(w: &mut Vec<u8>, line: &str) -> Value {
        w.clear();
        handle(line, w);
        json::parse(String::from_utf8_lossy(w).trim()).unwrap()
    }

    #[test]
    fn protocol_and_tools() {
        let mut w = Vec::new();
        let v = rpc(
            &mut w,
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}"#,
        );
        assert_eq!(
            v.get("result")
                .unwrap()
                .get("protocolVersion")
                .unwrap()
                .as_str(),
            Some("2025-03-26")
        );
        let v = rpc(&mut w, r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#);
        assert_eq!(
            v.get("result")
                .unwrap()
                .get("tools")
                .unwrap()
                .as_arr()
                .unwrap()
                .len(),
            6
        );
        w.clear();
        handle(
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            &mut w,
        );
        assert!(w.is_empty());
    }

    #[test]
    fn reads_only_allowed_folders_and_squeezes() {
        let d = tmp_project("mcpsrv");
        let logs = d.join("logs");
        std::fs::create_dir_all(&logs).unwrap();
        let mut big = String::new();
        for i in 0..500 {
            big += &format!("INFO request {} ok\n", i);
        }
        big += "ERROR database connection refused\n";
        std::fs::write(logs.join("app.log"), &big).unwrap();
        let mut cfg = Config::default();
        let a = json::parse(&format!(
            r#"{{"path":{}}}"#,
            Value::str(logs.join("app.log").to_string_lossy()).to_compact()
        ))
        .unwrap();
        let (t, err) = call("read_log", &a, &cfg);
        assert!(err && t.contains("No folders are allowed"), "{}", t);
        cfg.desktop.allowed_dirs = vec![logs.to_string_lossy().into_owned()];
        let (t, err) = call("read_log", &a, &cfg);
        assert!(!err, "{}", t);
        assert!(t.contains("ERROR database connection refused") && t.contains("[squeeze]"));
        assert!(t.len() < big.len() / 3);
        let outside = json::parse(&format!(
            r#"{{"path":{}}}"#,
            Value::str(d.join("x").to_string_lossy()).to_compact()
        ))
        .unwrap();
        std::fs::write(d.join("x"), "secret").unwrap();
        assert!(call("read_log", &outside, &cfg).1);
        let s = json::parse(&format!(
            r#"{{"path":{},"text":"refused"}}"#,
            Value::str(logs.join("app.log").to_string_lossy()).to_compact()
        ))
        .unwrap();
        assert!(call("search_log", &s, &cfg).0.contains("501"));
    }

    #[test]
    fn handoff_save_and_load() {
        crate::testutil::init();
        let cfg = Config::default();
        let a =
            json::parse(r##"{"title":"v0.2 work","content":"# Handoff\nnext: release"}"##).unwrap();
        let (t, err) = call("save_handoff", &a, &cfg);
        assert!(!err, "{}", t);
        let (t, err) = call("load_handoff", &Value::obj(), &cfg);
        assert!(!err && t.contains("next: release"), "{}", t);
    }
}
