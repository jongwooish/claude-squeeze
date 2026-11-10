//! MCP servers configured for Claude Code and the Claude desktop app: find
//! them, measure what their tool definitions cost on every message, and turn
//! idle ones off.
//!
//! Claude Code: servers come from `~/.claude.json` (user and per-project
//! scope) and `<project>/.mcp.json`. Turning one off uses Claude Code's own
//! per-project list `projects[<path>].disabledMcpServers` (the same switch as
//! `/mcp` → disable). Desktop app: the entry is moved out of
//! `claude_desktop_config.json` into `~/.squeeze/desktop-disabled.json` and
//! moved back to turn it on again.

use crate::config::Config;
use crate::json::{self, Value};
use crate::ledger::{self, Event};
use crate::paths;
use crate::transcripts;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    User,
    Local,
    Project,
    Desktop,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::User => "user",
            Scope::Local => "local",
            Scope::Project => "project",
            Scope::Desktop => "desktop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub name: String,
    pub scope: Scope,
    /// "stdio" | "http" | "sse"
    pub transport: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub url: String,
    pub enabled: bool,
    /// The raw config entry (to move it when disabling a desktop server).
    pub raw: Value,
}

impl Server {
    pub fn id(&self) -> String {
        format!("{}:{}", self.scope.as_str(), self.name)
    }
}

fn read_json(p: &Path) -> Option<Value> {
    json::parse(&std::fs::read_to_string(p).ok()?).ok()
}

fn parse_servers(obj: Option<&Value>, scope: Scope, disabled: &[String]) -> Vec<Server> {
    let Some(Value::Obj(kv)) = obj else {
        return Vec::new();
    };
    kv.iter()
        .map(|(name, v)| {
            let s = |k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
            let url = s("url");
            let mut transport = s("type");
            if transport.is_empty() {
                transport = if url.is_empty() {
                    "stdio".into()
                } else {
                    "http".into()
                };
            }
            Server {
                name: name.clone(),
                scope: scope.clone(),
                transport,
                command: s("command"),
                args: v
                    .get("args")
                    .and_then(|a| a.as_arr())
                    .map(|a| {
                        a.iter()
                            .filter_map(|x| x.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default(),
                env: match v.get("env") {
                    Some(Value::Obj(e)) => e
                        .iter()
                        .filter_map(|(k, x)| Some((k.clone(), x.as_str()?.to_string())))
                        .collect(),
                    _ => Vec::new(),
                },
                url,
                enabled: !disabled.iter().any(|d| d == name),
                raw: v.clone(),
            }
        })
        .collect()
}

fn project_key(state: &Value, root: &Path) -> Option<String> {
    let Some(Value::Obj(projects)) = state.get("projects") else {
        return None;
    };
    let want = root.to_string_lossy().replace('\\', "/");
    projects
        .iter()
        .map(|(k, _)| k)
        .find(|k| k.replace('\\', "/").trim_end_matches('/') == want.trim_end_matches('/'))
        .cloned()
}

fn disabled_list(state: &Value, key: Option<&str>) -> Vec<String> {
    key.and_then(|k| {
        state
            .get("projects")?
            .get(k)?
            .get("disabledMcpServers")?
            .as_arr()
            .cloned()
    })
    .map(|a| {
        a.iter()
            .filter_map(|x| x.as_str().map(str::to_string))
            .collect()
    })
    .unwrap_or_default()
}

/// Claude Code servers visible in a project.
pub fn claude_code_servers(root: &Path) -> Vec<Server> {
    let state = read_json(&paths::claude_state()).unwrap_or(Value::obj());
    let key = project_key(&state, root);
    let disabled = disabled_list(&state, key.as_deref());
    let mut v = parse_servers(state.get("mcpServers"), Scope::User, &disabled);
    if let Some(k) = &key {
        v.extend(parse_servers(
            state
                .get("projects")
                .and_then(|p| p.get(k))
                .and_then(|p| p.get("mcpServers")),
            Scope::Local,
            &disabled,
        ));
    }
    if let Some(pj) = read_json(&root.join(".mcp.json")) {
        v.extend(parse_servers(
            pj.get("mcpServers"),
            Scope::Project,
            &disabled,
        ));
    }
    v
}

fn desktop_stash() -> PathBuf {
    paths::squeeze_home().join("desktop-disabled.json")
}

/// Desktop app servers, including ones squeeze turned off.
pub fn desktop_servers() -> Vec<Server> {
    let Some(cfg) = paths::desktop_config() else {
        return Vec::new();
    };
    let mut v = read_json(&cfg)
        .map(|c| parse_servers(c.get("mcpServers"), Scope::Desktop, &[]))
        .unwrap_or_default();
    if let Some(stash) = read_json(&desktop_stash()) {
        let mut off = parse_servers(stash.get("mcpServers"), Scope::Desktop, &[]);
        for s in &mut off {
            s.enabled = false;
        }
        v.extend(off);
    }
    v
}

fn set_obj_path<'a>(v: &'a mut Value, keys: &[&str]) -> &'a mut Value {
    let mut cur = v;
    for k in keys {
        if cur.get(k).is_none() || !matches!(cur.get(k), Some(Value::Obj(_))) {
            cur.set(k, Value::obj());
        }
        cur = cur.get_mut(k).unwrap();
    }
    cur
}

/// Turn a Claude Code server on or off for a project (next session).
pub fn set_claude_code_enabled(root: &Path, name: &str, enabled: bool) -> Result<(), String> {
    let p = paths::claude_state();
    let mut state =
        read_json(&p).ok_or("~/.claude.json was not found; start Claude Code once first")?;
    let key = project_key(&state, root).unwrap_or_else(|| root.to_string_lossy().into_owned());
    paths::backup(&p);
    let proj = set_obj_path(&mut state, &["projects", &key]);
    let mut list: Vec<String> = proj
        .get("disabledMcpServers")
        .and_then(|a| a.as_arr())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    list.retain(|n| n != name);
    if !enabled {
        list.push(name.to_string());
    }
    proj.set(
        "disabledMcpServers",
        Value::Arr(list.into_iter().map(Value::str).collect()),
    );
    paths::write_atomic(&p, &(state.to_pretty() + "\n")).map_err(|e| e.to_string())
}

/// Turn a desktop app server on or off (restart the app to apply).
pub fn set_desktop_enabled(name: &str, enabled: bool) -> Result<(), String> {
    let cfg_path =
        paths::desktop_config().ok_or("the Claude desktop app is not available on this OS")?;
    let mut cfg = read_json(&cfg_path).unwrap_or(Value::obj());
    let mut stash = read_json(&desktop_stash()).unwrap_or(Value::obj());
    let (from, to) = if enabled {
        (&mut stash, &mut cfg)
    } else {
        (&mut cfg, &mut stash)
    };
    let entry = from
        .get("mcpServers")
        .and_then(|m| m.get(name))
        .cloned()
        .ok_or_else(|| format!("server {:?} not found", name))?;
    if let Some(m) = from.get_mut("mcpServers") {
        m.remove(name);
    }
    set_obj_path(to, &["mcpServers"]).set(name, entry);
    paths::backup(&cfg_path);
    paths::write_atomic(&cfg_path, &(cfg.to_pretty() + "\n")).map_err(|e| e.to_string())?;
    paths::write_atomic(&desktop_stash(), &(stash.to_pretty() + "\n")).map_err(|e| e.to_string())
}

// ---------- measuring tool definitions ----------

#[derive(Debug, Clone, Default)]
pub struct Measure {
    pub tools: usize,
    pub bytes: usize,
    pub tokens: usize,
    pub error: String,
    pub at: u64,
}

fn measure_cache() -> PathBuf {
    paths::squeeze_home().join("mcp-measure.json")
}

pub fn cached_measure(id: &str) -> Option<Measure> {
    let v = read_json(&measure_cache())?;
    let m = v.get(id)?;
    let n = |k: &str| m.get(k).and_then(|x| x.as_i64()).unwrap_or(0).max(0) as usize;
    Some(Measure {
        tools: n("tools"),
        bytes: n("bytes"),
        tokens: n("tokens"),
        error: m.get("error").and_then(|x| x.as_str()).unwrap_or("").into(),
        at: n("at") as u64,
    })
}

fn save_measure(id: &str, m: &Measure) {
    let mut v = read_json(&measure_cache()).unwrap_or(Value::obj());
    let mut x = Value::obj();
    x.set("tools", Value::num(m.tools));
    x.set("bytes", Value::num(m.bytes));
    x.set("tokens", Value::num(m.tokens));
    x.set("error", Value::str(m.error.clone()));
    x.set("at", Value::num(m.at));
    v.set(id, x);
    let _ = paths::write_atomic(&measure_cache(), &v.to_pretty());
}

fn rpc(id: u64, method: &str, params: Value) -> String {
    let mut v = Value::obj();
    v.set("jsonrpc", Value::str("2.0"));
    v.set("id", Value::num(id));
    v.set("method", Value::str(method));
    v.set("params", params);
    v.to_compact() + "\n"
}

/// Start a stdio server, ask for its tool list, and stop it. Runs the
/// server's own configured command, so callers only do this on request.
pub fn measure(s: &Server) -> Measure {
    let mut m = Measure {
        at: ledger::now(),
        ..Default::default()
    };
    if s.transport != "stdio" || s.command.is_empty() {
        m.error = "only local (stdio) servers can be measured".into();
        return m;
    }
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&s.command);
        c
    } else {
        Command::new(&s.command)
    };
    crate::paths::no_window(&mut cmd)
        .args(&s.args)
        .envs(s.env.iter().cloned())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            m.error = format!("could not start: {}", e);
            return m;
        }
    };
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let wait_for = |id: u64| -> Option<Value> {
        let deadline = std::time::Instant::now() + Duration::from_secs(20);
        while let Some(left) = deadline.checked_duration_since(std::time::Instant::now()) {
            let line = rx.recv_timeout(left).ok()?;
            if let Ok(v) = json::parse(&line) {
                if v.get("id").and_then(|x| x.as_i64()) == Some(id as i64) {
                    return Some(v);
                }
            }
        }
        None
    };
    let mut init = Value::obj();
    init.set("protocolVersion", Value::str("2025-06-18"));
    init.set("capabilities", Value::obj());
    let mut ci = Value::obj();
    ci.set("name", Value::str("squeeze-auditor"));
    ci.set("version", Value::str(env!("CARGO_PKG_VERSION")));
    init.set("clientInfo", ci);
    let _ = stdin.write_all(rpc(1, "initialize", init).as_bytes());
    let ok = wait_for(1).is_some();
    if !ok {
        m.error = "no answer to initialize within 20 s".into();
    } else {
        let _ =
            stdin.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n");
        let mut tools: Vec<Value> = Vec::new();
        let mut cursor: Option<String> = None;
        for page in 0..20u64 {
            let mut p = Value::obj();
            if let Some(c) = &cursor {
                p.set("cursor", Value::str(c.clone()));
            }
            let _ = stdin.write_all(rpc(2 + page, "tools/list", p).as_bytes());
            let Some(resp) = wait_for(2 + page) else {
                m.error = "no answer to tools/list within 20 s".into();
                break;
            };
            let r = resp.get("result");
            if let Some(a) = r.and_then(|r| r.get("tools")).and_then(|t| t.as_arr()) {
                tools.extend(a.iter().cloned());
            }
            cursor = r
                .and_then(|r| r.get("nextCursor"))
                .and_then(|c| c.as_str())
                .map(str::to_string);
            if cursor.is_none() {
                break;
            }
        }
        m.tools = tools.len();
        m.bytes = Value::Arr(tools).to_compact().len();
        m.tokens = ledger::tokens(m.bytes);
    }
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();
    save_measure(&s.id(), &m);
    m
}

// ---------- auto prune (SessionStart) ----------

fn seen_file() -> PathBuf {
    paths::squeeze_home().join("mcp-seen.json")
}

/// Turn off Claude Code servers this project has not used for
/// `unused_days`. A server is only judged after squeeze has known about it
/// for that long, so newly added servers are never turned off right away.
pub fn auto_prune(root: &Path, cfg: &Config) -> Option<String> {
    let servers: Vec<Server> = claude_code_servers(root)
        .into_iter()
        .filter(|s| s.enabled)
        .collect();
    if servers.is_empty() {
        return None;
    }
    let now = ledger::now();
    let limit = cfg.mcp_prune.unused_days * 86_400;
    let mut seen = read_json(&seen_file()).unwrap_or(Value::obj());
    let last = transcripts::mcp_last_used(root);
    let mut off = Vec::new();
    for s in &servers {
        let key = format!("{}|{}", root.to_string_lossy(), s.name);
        let first = match seen.get(&key).and_then(|x| x.as_i64()) {
            Some(t) => t as u64,
            None => {
                seen.set(&key, Value::num(now));
                now
            }
        };
        let used = last
            .get(&transcripts::normalize_server(&s.name))
            .copied()
            .unwrap_or(0);
        if now.saturating_sub(first.max(used)) > limit
            && set_claude_code_enabled(root, &s.name, false).is_ok()
        {
            let tokens = cached_measure(&s.id()).map(|m| m.tokens).unwrap_or(0);
            let _ = ledger::record(
                Some(root),
                Event {
                    kind: "mcp-off".into(),
                    subject: s.name.clone(),
                    in_bytes: tokens * 4,
                    ..Default::default()
                },
            );
            off.push(s.name.clone());
        }
    }
    let _ = paths::write_atomic(&seen_file(), &seen.to_pretty());
    if off.is_empty() {
        return None;
    }
    Some(format!(
        "squeeze: turned off MCP server(s) unused in this project for {} days: {} (from the next session). Turn back on in squeeze-Auditor or with /mcp.",
        cfg.mcp_prune.unused_days,
        off.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::tmp_project;

    #[test]
    fn discovers_and_toggles_claude_code_servers() {
        let root = tmp_project("mcp");
        let key = root.to_string_lossy().into_owned();
        let state = format!(
            r#"{{"mcpServers":{{"github":{{"command":"npx","args":["-y","gh-mcp"]}}}},"projects":{{{}:{{"mcpServers":{{"db":{{"type":"http","url":"http://x"}}}}}}}}}}"#,
            Value::str(key.clone()).to_compact()
        );
        std::fs::write(paths::claude_state(), state).unwrap();
        std::fs::write(
            root.join(".mcp.json"),
            r#"{"mcpServers":{"docs":{"command":"docs-mcp"}}}"#,
        )
        .unwrap();
        let v = claude_code_servers(&root);
        let names: Vec<_> = v
            .iter()
            .map(|s| (s.name.as_str(), s.scope.as_str(), s.enabled))
            .collect();
        assert_eq!(
            names,
            vec![
                ("github", "user", true),
                ("db", "local", true),
                ("docs", "project", true)
            ]
        );
        set_claude_code_enabled(&root, "github", false).unwrap();
        assert!(!claude_code_servers(&root)[0].enabled);
        set_claude_code_enabled(&root, "github", true).unwrap();
        assert!(claude_code_servers(&root)[0].enabled);
        // other keys in ~/.claude.json survive
        assert!(std::fs::read_to_string(paths::claude_state())
            .unwrap()
            .contains("gh-mcp"));
    }

    #[test]
    fn desktop_toggle_moves_entry_and_back() {
        let cfg = paths::desktop_config().unwrap();
        std::fs::create_dir_all(cfg.parent().unwrap()).unwrap();
        std::fs::write(
            &cfg,
            r#"{"mcpServers":{"fs":{"command":"fs-mcp"}},"other":1}"#,
        )
        .unwrap();
        set_desktop_enabled("fs", false).unwrap();
        let text = std::fs::read_to_string(&cfg).unwrap();
        assert!(
            !text.contains("fs-mcp") && text.contains("\"other\""),
            "{}",
            text
        );
        assert!(desktop_servers()
            .iter()
            .any(|s| s.name == "fs" && !s.enabled));
        set_desktop_enabled("fs", true).unwrap();
        assert!(std::fs::read_to_string(&cfg).unwrap().contains("fs-mcp"));
    }

    #[test]
    fn measures_a_real_stdio_server() {
        // a tiny MCP server in sh: answers initialize and tools/list
        let d = tmp_project("measure");
        let script = d.join("fake-mcp.sh");
        std::fs::write(
            &script,
            "#!/bin/sh\nwhile read line; do\n  case \"$line\" in\n    *initialize\\\"*) echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"protocolVersion\":\"2025-06-18\",\"capabilities\":{}}}' ;;\n    *tools/list*) echo '{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"tools\":[{\"name\":\"a\",\"description\":\"does a\",\"inputSchema\":{\"type\":\"object\"}},{\"name\":\"b\",\"description\":\"does b\",\"inputSchema\":{\"type\":\"object\"}}]}}' ;;\n  esac\ndone\n",
        )
        .unwrap();
        if cfg!(windows) {
            return;
        }
        let s = Server {
            name: "fake".into(),
            scope: Scope::User,
            transport: "stdio".into(),
            command: "sh".into(),
            args: vec![script.to_string_lossy().into_owned()],
            env: vec![],
            url: String::new(),
            enabled: true,
            raw: Value::obj(),
        };
        let m = measure(&s);
        assert_eq!(m.error, "");
        assert_eq!(m.tools, 2);
        assert!(m.tokens > 10);
        assert_eq!(cached_measure("user:fake").unwrap().tools, 2);
    }
}
