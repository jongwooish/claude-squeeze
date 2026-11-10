//! squeeze-Auditor: a control panel served on 127.0.0.1 only and opened as
//! an app window (Edge/Chrome `--app`, else the default browser). Every API
//! call needs a per-launch secret token, and the Host header must be the
//! loopback address, so other web pages cannot talk to it.

use crate::config::{self, Config};
use crate::json::{self, Value};
use crate::{doctor, mcp, paths, setup, usage};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const PAGE: &str = include_str!("../ui/index.html");

struct Req {
    method: String,
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: String,
}

impl Req {
    fn header(&self, k: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(h, _)| h.eq_ignore_ascii_case(k))
            .map(|(_, v)| v.as_str())
    }
    fn q(&self, k: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(a, _)| a == k)
            .map(|(_, v)| v.as_str())
    }
    fn json(&self) -> Value {
        json::parse(&self.body).unwrap_or(Value::obj())
    }
}

fn unescape(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < b.len() => {
                if let Ok(x) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                    out.push(x);
                    i += 2;
                } else {
                    out.push(b'%');
                }
            }
            c => out.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn read_req(stream: &mut TcpStream) -> Option<Req> {
    let mut r = BufReader::new(stream.try_clone().ok()?);
    let mut line = String::new();
    r.read_line(&mut line).ok()?;
    let mut parts = line.split_whitespace();
    let method = parts.next()?.to_string();
    let target = parts.next()?.to_string();
    let mut headers = Vec::new();
    loop {
        let mut h = String::new();
        if r.read_line(&mut h).ok()? == 0 {
            break;
        }
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let len: usize = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0)
        .min(4 * 1024 * 1024);
    let mut body = vec![0; len];
    r.read_exact(&mut body).ok()?;
    let (path, qs) = target.split_once('?').unwrap_or((&target, ""));
    let query = qs
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let (k, v) = p.split_once('=').unwrap_or((p, ""));
            (unescape(k), unescape(v))
        })
        .collect();
    Some(Req {
        method,
        path: path.to_string(),
        query,
        headers,
        body: String::from_utf8_lossy(&body).into_owned(),
    })
}

fn respond(stream: &mut TcpStream, status: &str, ctype: &str, body: &str) {
    let _ = write!(
        stream,
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: default-src 'self'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'\r\nConnection: close\r\n\r\n{}",
        status,
        ctype,
        body.len(),
        body
    );
}

fn ok(v: Value) -> (String, String) {
    ("200 OK".into(), v.to_compact())
}
fn bad(msg: &str) -> (String, String) {
    let mut v = Value::obj();
    v.set("error", Value::str(msg));
    ("400 Bad Request".into(), v.to_compact())
}

/// Projects seen in Claude Code transcripts (newest first).
fn projects() -> Vec<PathBuf> {
    let mut seen = Vec::new();
    for f in transcripts_recent() {
        if let Ok(text) = std::fs::read_to_string(&f) {
            if let Some((_, t)) = text.lines().rev().find_map(crate::transcripts::parse_line) {
                let p = PathBuf::from(&t.cwd);
                if !t.cwd.is_empty() && !seen.contains(&p) {
                    seen.push(p);
                }
            }
        }
        if seen.len() >= 30 {
            break;
        }
    }
    seen
}

fn transcripts_recent() -> Vec<PathBuf> {
    crate::transcripts::files().into_iter().take(200).collect()
}

fn root_of(r: &Req, body: &Value) -> PathBuf {
    let raw = body
        .get("root")
        .and_then(|x| x.as_str())
        .map(str::to_string)
        .or_else(|| r.q("root").map(str::to_string))
        .unwrap_or_default();
    if raw.is_empty() {
        projects().into_iter().next().unwrap_or_else(paths::home)
    } else {
        PathBuf::from(raw)
    }
}

fn find_server(root: &Path, id: &str) -> Option<mcp::Server> {
    mcp::claude_code_servers(root)
        .into_iter()
        .chain(mcp::desktop_servers())
        .find(|s| s.id() == id)
}

fn handoffs() -> Vec<(PathBuf, String)> {
    let mut dirs: Vec<(PathBuf, String)> = projects()
        .into_iter()
        .map(|p| (p.join(config::DIR).join("handoff"), paths::tilde(&p)))
        .collect();
    dirs.push((
        paths::squeeze_home().join("handoff").join("desktop"),
        "Claude desktop app".into(),
    ));
    let mut out = Vec::new();
    for (d, label) in dirs {
        if let Ok(rd) = std::fs::read_dir(&d) {
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().is_some_and(|x| x == "md") {
                    out.push((p, label.clone()));
                }
            }
        }
    }
    out.sort_by(|a, b| b.0.file_name().cmp(&a.0.file_name()));
    out
}

fn env_json() -> Value {
    let s = setup::state();
    let cfg = config::load_global();
    let mut v = Value::obj();
    v.set("claude_code_found", Value::Bool(s.claude_code_found));
    v.set("claude_code_hooks", Value::Bool(s.claude_code_hooks));
    v.set("claude_code_complete", Value::Bool(s.claude_code_complete));
    v.set("desktop_available", Value::Bool(s.desktop_available));
    v.set("desktop_found", Value::Bool(s.desktop_found));
    v.set("desktop_connected", Value::Bool(s.desktop_connected));
    v.set("exe", Value::str(paths::tilde(Path::new(&s.exe))));
    v.set(
        "allowed_dirs",
        Value::Arr(
            cfg.desktop
                .allowed_dirs
                .iter()
                .map(|d| Value::str(d.clone()))
                .collect(),
        ),
    );
    v.set(
        "settings_path",
        Value::str(paths::tilde(&paths::claude_settings())),
    );
    v.set(
        "desktop_config",
        Value::str(
            paths::desktop_config()
                .map(|p| paths::tilde(&p))
                .unwrap_or_default(),
        ),
    );
    v.set("version", Value::str(env!("CARGO_PKG_VERSION")));
    v
}

fn settings_json() -> Value {
    let c = config::load_global();
    let mut v = Value::obj();
    v.set("config", c.to_json());
    v.set(
        "profiles",
        Value::Arr(
            config::PROFILES
                .iter()
                .map(|p| {
                    let mut x = Value::obj();
                    x.set("name", Value::str(*p));
                    x.set("config", config::preset(p, &c).to_json());
                    x
                })
                .collect(),
        ),
    );
    v.set("path", Value::str(paths::tilde(&config::global_path())));
    v
}

fn route(r: &Req, quit: &Arc<AtomicU64>, beat: &Arc<AtomicU64>) -> (String, String) {
    let body = r.json();
    match (r.method.as_str(), r.path.as_str()) {
        ("GET", "/api/overview") => {
            let days = r
                .q("days")
                .and_then(|d| d.parse().ok())
                .unwrap_or(30u64)
                .clamp(1, 365);
            ok(usage::overview(days))
        }
        ("GET", "/api/settings") => ok(settings_json()),
        ("POST", "/api/settings") => {
            let mut c = config::load_global();
            if let Some(cfg) = body.get("config") {
                c.merge(cfg);
            }
            if body.get("custom").and_then(|x| x.as_bool()) == Some(true) {
                c.profile = "custom".into();
            }
            match config::save_global(&c) {
                Ok(()) => ok(settings_json()),
                Err(e) => bad(&e.to_string()),
            }
        }
        ("POST", "/api/profile") => {
            let name = body
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("balanced");
            let cur = config::load_global();
            let c = config::preset(name, &cur);
            match config::save_global(&c) {
                Ok(()) => ok(settings_json()),
                Err(e) => bad(&e.to_string()),
            }
        }
        ("GET", "/api/projects") => ok(Value::Arr(
            projects()
                .iter()
                .map(|p| {
                    let mut x = Value::obj();
                    x.set("root", Value::str(p.to_string_lossy()));
                    x.set("path", Value::str(paths::tilde(p)));
                    x
                })
                .collect(),
        )),
        ("GET", "/api/doctor") => {
            if r.q("target") == Some("desktop") {
                ok(doctor::to_json(&doctor::desktop()))
            } else {
                ok(doctor::to_json(&doctor::claude_code(&root_of(r, &body))))
            }
        }
        ("POST", "/api/mcp/measure") => {
            let root = root_of(r, &body);
            let id = body.get("id").and_then(|x| x.as_str()).unwrap_or("");
            let Some(s) = find_server(&root, id) else {
                return bad("server not found");
            };
            let m = mcp::measure(&s);
            let mut v = Value::obj();
            v.set("tools", Value::num(m.tools));
            v.set("tokens", Value::num(m.tokens));
            v.set("error", Value::str(m.error));
            ok(v)
        }
        ("POST", "/api/mcp/toggle") => {
            let root = root_of(r, &body);
            let id = body.get("id").and_then(|x| x.as_str()).unwrap_or("");
            let on = body
                .get("enabled")
                .and_then(|x| x.as_bool())
                .unwrap_or(true);
            let Some(s) = find_server(&root, id) else {
                return bad("server not found");
            };
            let res = if s.scope == mcp::Scope::Desktop {
                mcp::set_desktop_enabled(&s.name, on)
            } else {
                mcp::set_claude_code_enabled(&root, &s.name, on)
            };
            match res {
                Ok(()) => {
                    if !on {
                        let tokens = mcp::cached_measure(&s.id()).map(|m| m.tokens).unwrap_or(0);
                        let _ = crate::ledger::record(
                            Some(&root),
                            crate::ledger::Event {
                                kind: "mcp-off".into(),
                                subject: s.name.clone(),
                                in_bytes: tokens * 4,
                                ..Default::default()
                            },
                        );
                    }
                    ok(Value::Bool(true))
                }
                Err(e) => bad(&e),
            }
        }
        ("GET", "/api/env") => ok(env_json()),
        ("POST", "/api/env") => {
            let target = body.get("target").and_then(|x| x.as_str()).unwrap_or("");
            let connect = body
                .get("connect")
                .and_then(|x| x.as_bool())
                .unwrap_or(true);
            let res = match (target, connect) {
                ("claude_code", true) => setup::connect_claude_code(),
                ("claude_code", false) => setup::disconnect_claude_code().map(|_| ()),
                ("desktop", true) => setup::connect_desktop(),
                ("desktop", false) => setup::disconnect_desktop().map(|_| ()),
                _ => Err("unknown target".into()),
            };
            match res {
                Ok(()) => ok(env_json()),
                Err(e) => bad(&e),
            }
        }
        ("POST", "/api/allowed") => {
            let dirs: Vec<String> = body
                .get("dirs")
                .and_then(|d| d.as_arr())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
                        .filter(|s| !s.is_empty())
                        .collect()
                })
                .unwrap_or_default();
            if let Some(missing) = dirs.iter().find(|d| !Path::new(d).is_dir()) {
                return bad(&format!("not a folder: {}", missing));
            }
            let mut c: Config = config::load_global();
            c.desktop.allowed_dirs = dirs;
            match config::save_global(&c) {
                Ok(()) => ok(env_json()),
                Err(e) => bad(&e.to_string()),
            }
        }
        ("GET", "/api/handoffs") => ok(Value::Arr(
            handoffs()
                .iter()
                .take(50)
                .map(|(p, label)| {
                    let mut x = Value::obj();
                    x.set("path", Value::str(p.to_string_lossy()));
                    x.set(
                        "name",
                        Value::str(p.file_name().unwrap_or_default().to_string_lossy()),
                    );
                    x.set("where", Value::str(label.clone()));
                    x.set(
                        "bytes",
                        Value::num(std::fs::metadata(p).map(|m| m.len()).unwrap_or(0)),
                    );
                    x
                })
                .collect(),
        )),
        ("GET", "/api/handoff") => {
            let want = r.q("path").unwrap_or("");
            match handoffs()
                .into_iter()
                .find(|(p, _)| p.to_string_lossy() == want)
            {
                Some((p, _)) => {
                    let mut x = Value::obj();
                    x.set(
                        "text",
                        Value::str(std::fs::read_to_string(p).unwrap_or_default()),
                    );
                    ok(x)
                }
                None => bad("not found"),
            }
        }
        ("POST", "/api/heartbeat") => {
            beat.store(crate::ledger::now(), Ordering::SeqCst);
            ok(Value::Bool(true))
        }
        ("POST", "/api/quit") => {
            quit.store(1, Ordering::SeqCst);
            ok(Value::Bool(true))
        }
        _ => ("404 Not Found".into(), "{\"error\":\"not found\"}".into()),
    }
}

fn handle(mut s: TcpStream, port: u16, token: &str, quit: Arc<AtomicU64>, beat: Arc<AtomicU64>) {
    let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(10)));
    let Some(r) = read_req(&mut s) else { return };
    let host_ok = r
        .header("host")
        .is_some_and(|h| h == format!("127.0.0.1:{}", port) || h == format!("localhost:{}", port));
    if !host_ok {
        return respond(&mut s, "403 Forbidden", "text/plain", "forbidden");
    }
    if r.method == "GET" && r.path == "/" {
        if r.q("t") != Some(token) {
            return respond(
                &mut s,
                "403 Forbidden",
                "text/plain",
                "Open squeeze-Auditor from its shortcut.",
            );
        }
        let page = PAGE
            .replace("__SQUEEZE_TOKEN__", token)
            .replace("__SQUEEZE_VERSION__", env!("CARGO_PKG_VERSION"));
        return respond(&mut s, "200 OK", "text/html; charset=utf-8", &page);
    }
    if r.method == "GET" && r.path == "/favicon.ico" {
        return respond(&mut s, "204 No Content", "text/plain", "");
    }
    if !r.path.starts_with("/api/") || r.header("x-squeeze-token") != Some(token) {
        return respond(
            &mut s,
            "403 Forbidden",
            "application/json",
            "{\"error\":\"forbidden\"}",
        );
    }
    if r.method == "POST"
        && !r
            .header("content-type")
            .unwrap_or("")
            .starts_with("application/json")
    {
        return respond(
            &mut s,
            "415 Unsupported Media Type",
            "application/json",
            "{\"error\":\"json only\"}",
        );
    }
    let (status, body) = route(&r, &quit, &beat);
    respond(&mut s, &status, "application/json; charset=utf-8", &body);
}

fn lock_file() -> PathBuf {
    paths::squeeze_home().join("auditor.lock")
}

/// If an Auditor is already running, return its URL.
fn running() -> Option<String> {
    let text = std::fs::read_to_string(lock_file()).ok()?;
    let (port, token) = text.trim().split_once(' ')?;
    let port: u16 = port.parse().ok()?;
    let mut s = TcpStream::connect_timeout(
        &format!("127.0.0.1:{}", port).parse().ok()?,
        std::time::Duration::from_millis(500),
    )
    .ok()?;
    let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(2)));
    write!(s, "GET /api/env HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nX-Squeeze-Token: {}\r\nConnection: close\r\n\r\n", port, token).ok()?;
    let mut buf = String::new();
    s.read_to_string(&mut buf).ok()?;
    buf.starts_with("HTTP/1.1 200")
        .then(|| format!("http://127.0.0.1:{}/?t={}", port, token))
}

/// Open `url` as an app window (no tabs or address bar) when Edge or Chrome
/// is available, else in the default browser.
pub fn open_window(url: &str) {
    use std::process::Command;
    let app = format!("--app={}", url);
    let tried = if cfg!(windows) {
        // Edge ships with Windows 10/11. Start browsers by path so a missing
        // one never pops up an error dialog.
        let env = |k: &str| std::env::var_os(k).map(PathBuf::from);
        let mut candidates: Vec<PathBuf> = Vec::new();
        for base in ["ProgramFiles(x86)", "ProgramFiles", "LOCALAPPDATA"] {
            if let Some(b) = env(base) {
                candidates.push(b.join(r"Microsoft\Edge\Application\msedge.exe"));
            }
        }
        for base in ["ProgramFiles", "ProgramFiles(x86)", "LOCALAPPDATA"] {
            if let Some(b) = env(base) {
                candidates.push(b.join(r"Google\Chrome\Application\chrome.exe"));
            }
        }
        candidates
            .iter()
            .filter(|p| p.is_file())
            .any(|p| Command::new(p).arg(&app).spawn().is_ok())
            || paths::no_window(Command::new("rundll32").args(["url.dll,FileProtocolHandler", url]))
                .spawn()
                .is_ok()
    } else if cfg!(target_os = "macos") {
        ["Google Chrome", "Microsoft Edge", "Brave Browser"]
            .iter()
            .any(|b| {
                Path::new(&format!("/Applications/{}.app", b)).exists()
                    && Command::new("open")
                        .args(["-na", b, "--args", &app])
                        .status()
                        .is_ok_and(|s| s.success())
            })
            || Command::new("open")
                .arg(url)
                .status()
                .is_ok_and(|s| s.success())
    } else {
        [
            "google-chrome",
            "chromium",
            "chromium-browser",
            "microsoft-edge",
            "brave-browser",
        ]
        .iter()
        .any(|b| Command::new(b).arg(&app).spawn().is_ok())
            || Command::new("xdg-open").arg(url).spawn().is_ok()
    };
    if !tried {
        eprintln!(
            "squeeze-Auditor: open this address in your browser: {}",
            url
        );
    }
}

pub struct Options {
    pub port: u16,
    pub open: bool,
    /// Exit when the window has not checked in for this long (0 = never).
    pub idle_exit_secs: u64,
}

pub fn run(o: Options) -> Result<(), String> {
    if o.port == 0 {
        if let Some(url) = running() {
            if o.open {
                open_window(&url);
            }
            println!("squeeze-Auditor is already running: {}", url);
            return Ok(());
        }
    }
    let listener = TcpListener::bind(("127.0.0.1", o.port))
        .map_err(|e| format!("cannot listen on 127.0.0.1: {}", e))?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let token = paths::random_token();
    let _ = paths::write_atomic(&lock_file(), &format!("{} {}\n", port, token));
    let url = format!("http://127.0.0.1:{}/?t={}", port, token);
    println!(
        "squeeze-Auditor {} is running at {}",
        env!("CARGO_PKG_VERSION"),
        url
    );
    println!("Close the window (or press Ctrl+C here) to stop it.");
    if o.open {
        open_window(&url);
    }
    let quit = Arc::new(AtomicU64::new(0));
    let beat = Arc::new(AtomicU64::new(crate::ledger::now()));
    {
        let (quit, beat) = (quit.clone(), beat.clone());
        let idle = o.idle_exit_secs;
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let stale =
                idle > 0 && crate::ledger::now().saturating_sub(beat.load(Ordering::SeqCst)) > idle;
            if quit.load(Ordering::SeqCst) == 1 || stale {
                let _ = std::fs::remove_file(lock_file());
                std::process::exit(0);
            }
        });
    }
    for s in listener.incoming().flatten() {
        let (t, q, b) = (token.clone(), quit.clone(), beat.clone());
        std::thread::spawn(move || handle(s, port, &t, q, b));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescape_query() {
        assert_eq!(unescape("a%20b+c%2Fd"), "a b c/d");
        assert_eq!(unescape("100%"), "100%");
    }
}
