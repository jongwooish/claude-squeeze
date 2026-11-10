//! Connect squeeze to Claude Code (hooks in `~/.claude/settings.json`, all
//! projects) and to the Claude desktop app (an MCP server entry in
//! `claude_desktop_config.json`). Both can be undone.

use crate::install;
use crate::json::{self, Value};
use crate::paths;
use std::path::PathBuf;

/// The installed `squeeze` program next to this one (squeeze-auditor runs
/// the same code but hooks must call the console program).
pub fn squeeze_exe() -> PathBuf {
    let me = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("squeeze"));
    let name = if cfg!(windows) {
        "squeeze.exe"
    } else {
        "squeeze"
    };
    match me.parent() {
        Some(dir) if dir.join(name).is_file() => dir.join(name),
        _ => me,
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub claude_code_found: bool,
    pub claude_code_hooks: bool,
    pub claude_code_complete: bool,
    pub desktop_available: bool,
    pub desktop_found: bool,
    pub desktop_connected: bool,
    pub exe: String,
}

pub const DESKTOP_NAME: &str = "squeeze";

pub fn state() -> State {
    let (hooks, complete) = install::hooks_state(&paths::claude_settings());
    let desk = paths::desktop_config();
    let connected = desk
        .as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| json::parse(&t).ok())
        .is_some_and(|v| {
            v.get("mcpServers")
                .and_then(|m| m.get(DESKTOP_NAME))
                .is_some()
        });
    State {
        claude_code_found: paths::claude_dir().is_dir() || paths::claude_state().is_file(),
        claude_code_hooks: hooks,
        claude_code_complete: complete,
        desktop_available: desk.is_some(),
        desktop_found: desk
            .as_ref()
            .is_some_and(|p| p.is_file() || p.parent().is_some_and(|d| d.is_dir())),
        desktop_connected: connected,
        exe: squeeze_exe().to_string_lossy().into_owned(),
    }
}

pub fn connect_claude_code() -> Result<(), String> {
    let p = paths::claude_settings();
    paths::backup(&p);
    install::add_hooks(&p, &squeeze_exe().to_string_lossy(), false)
}

pub fn disconnect_claude_code() -> Result<bool, String> {
    let p = paths::claude_settings();
    paths::backup(&p);
    install::remove_hooks(&p, false)
}

fn read_desktop() -> Result<(PathBuf, Value), String> {
    let p = paths::desktop_config().ok_or("the Claude desktop app is not available on this OS")?;
    let v = match std::fs::read_to_string(&p) {
        Ok(t) if !t.trim().is_empty() => json::parse(&t)
            .map_err(|e| format!("{} is not valid JSON ({}); fix it first", p.display(), e))?,
        _ => Value::obj(),
    };
    Ok((p, v))
}

pub fn connect_desktop() -> Result<(), String> {
    let (p, mut v) = read_desktop()?;
    let mut entry = Value::obj();
    entry.set("command", Value::str(squeeze_exe().to_string_lossy()));
    entry.set("args", Value::Arr(vec![Value::str("mcp")]));
    if v.get("mcpServers").is_none() {
        v.set("mcpServers", Value::obj());
    }
    v.get_mut("mcpServers").unwrap().set(DESKTOP_NAME, entry);
    paths::backup(&p);
    paths::write_atomic(&p, &(v.to_pretty() + "\n")).map_err(|e| e.to_string())
}

pub fn disconnect_desktop() -> Result<bool, String> {
    let (p, mut v) = read_desktop()?;
    let Some(m) = v.get_mut("mcpServers") else {
        return Ok(false);
    };
    if m.get(DESKTOP_NAME).is_none() {
        return Ok(false);
    }
    m.remove(DESKTOP_NAME);
    paths::backup(&p);
    paths::write_atomic(&p, &(v.to_pretty() + "\n")).map_err(|e| e.to_string())?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_and_disconnect_everything() {
        crate::testutil::init();
        let settings = paths::claude_settings();
        std::fs::create_dir_all(settings.parent().unwrap()).unwrap();
        std::fs::write(&settings, r#"{"theme":"dark","hooks":{"Stop":[{"hooks":[{"type":"command","command":"notify-me"}]}]}}"#).unwrap();
        connect_claude_code().unwrap();
        let s = state();
        assert!(s.claude_code_hooks && s.claude_code_complete);
        let text = std::fs::read_to_string(&settings).unwrap();
        assert!(
            text.contains("notify-me") && text.contains("hook session-start"),
            "{}",
            text
        );
        assert!(disconnect_claude_code().unwrap());
        let text = std::fs::read_to_string(&settings).unwrap();
        assert!(
            text.contains("notify-me") && !text.contains("hook "),
            "{}",
            text
        );

        connect_desktop().unwrap();
        assert!(state().desktop_connected);
        assert!(disconnect_desktop().unwrap());
        assert!(!state().desktop_connected);
    }
}
