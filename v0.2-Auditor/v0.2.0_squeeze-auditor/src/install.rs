//! `squeeze init` / `squeeze uninstall`: hooks in .claude/settings.json,
//! a default config, a .gitignore entry and a short CLAUDE.md note.

use crate::config;
use crate::json::{self, Value};
use std::path::Path;

const START: &str = "<!-- squeeze:start -->";
const END: &str = "<!-- squeeze:end -->";

pub fn snippet() -> String {
    format!(
        "{START}\n## Saving tokens (squeeze)\n\n\
- For commands that print a lot (installs, builds, tests, logs), run them as `squeeze run -- <command>`. You get errors, warnings, and the first and last lines; the full log path is printed at the end. Open that log only if the summary is not enough.\n\
- For large files, read a range with offset and limit instead of the whole file. Do not re-read a file you already read unless it changed.\n\
- If squeeze asks for a handoff note, write it to the given path before stopping.\n{END}\n"
    )
}

pub struct Options<'a> {
    pub dir: &'a Path,
    pub bin: &'a str,
    pub dry_run: bool,
    pub purge: bool,
}

/// (event, matcher, `squeeze hook` subcommand). v0.1 used the first two;
/// v0.2 adds the session hooks.
pub const HOOKS: &[(&str, &str, &str)] = &[
    ("PreToolUse", "Bash", "pre-bash"),
    ("PreToolUse", "Read", "pre-read"),
    ("UserPromptSubmit", "", "prompt"),
    ("Stop", "", "stop"),
    ("SessionStart", "", "session-start"),
    ("PreCompact", "", "pre-compact"),
];

fn entry(matcher: &str, command: &str) -> Value {
    let mut h = Value::obj();
    h.set("type", Value::str("command"));
    h.set("command", Value::str(command));
    let mut e = Value::obj();
    if !matcher.is_empty() {
        e.set("matcher", Value::str(matcher));
    }
    e.set("hooks", Value::Arr(vec![h]));
    e
}

/// The command a hook runs. Paths use forward slashes and are quoted when
/// they contain spaces, which works in bash, sh, cmd and PowerShell.
pub fn hook_cmd(bin: &str, sub: &str) -> String {
    let b = bin.replace('\\', "/");
    if b.contains(' ') {
        format!("\"{}\" hook {}", b, sub)
    } else {
        format!("{} hook {}", b, sub)
    }
}

fn is_ours(e: &Value) -> bool {
    e.get("hooks").and_then(|h| h.as_arr()).is_some_and(|hs| {
        hs.iter().any(|h| {
            h.get("command")
                .and_then(|c| c.as_str())
                .is_some_and(|c| c.contains("squeeze") && c.contains(" hook "))
        })
    })
}

fn read_settings(p: &Path) -> Result<Value, String> {
    match std::fs::read_to_string(p) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Value::obj()),
        Err(e) => Err(e.to_string()),
        Ok(s) if s.trim().is_empty() => Ok(Value::obj()),
        Ok(s) => match json::parse(&s) {
            Ok(v @ Value::Obj(_)) => Ok(v),
            Ok(_) => Err(format!("{} is not a JSON object", p.display())),
            Err(e) => Err(format!(
                "{} is not valid JSON ({}); fix it first, squeeze will not overwrite it",
                p.display(),
                e
            )),
        },
    }
}

/// Remove squeeze entries from every hook event; returns whether anything
/// changed.
fn strip_ours(s: &mut Value) -> bool {
    let Some(hooks) = s.get_mut("hooks") else {
        return false;
    };
    let Value::Obj(events) = hooks else {
        return false;
    };
    let mut changed = false;
    for (_, list) in events.iter_mut() {
        if let Value::Arr(a) = list {
            let before = a.len();
            a.retain(|e| !is_ours(e));
            changed |= a.len() != before;
        }
    }
    events.retain(|(_, list)| !matches!(list, Value::Arr(a) if a.is_empty()));
    if events.is_empty() {
        s.remove("hooks");
    }
    changed
}

/// Add all squeeze hooks to a settings file (replacing old ones).
pub fn add_hooks(settings: &Path, bin: &str, dry: bool) -> Result<(), String> {
    let mut s = read_settings(settings)?;
    strip_ours(&mut s);
    if s.get("hooks").is_none() {
        s.set("hooks", Value::obj());
    }
    let hooks = s.get_mut("hooks").unwrap();
    if !matches!(hooks, Value::Obj(_)) {
        return Err("settings.json \"hooks\" is not an object".into());
    }
    for (event, matcher, sub) in HOOKS {
        if hooks.get(event).is_none() {
            hooks.set(event, Value::Arr(Vec::new()));
        }
        if let Some(Value::Arr(list)) = hooks.get_mut(event) {
            list.push(entry(matcher, &hook_cmd(bin, sub)));
        }
    }
    write_settings(settings, &s, dry)
}

/// Remove squeeze hooks from a settings file. Returns whether any were found.
pub fn remove_hooks(settings: &Path, dry: bool) -> Result<bool, String> {
    let mut s = read_settings(settings)?;
    if strip_ours(&mut s) {
        write_settings(settings, &s, dry)?;
        return Ok(true);
    }
    Ok(false)
}

/// Whether a settings file has squeeze hooks, and whether they are complete.
pub fn hooks_state(settings: &Path) -> (bool, bool) {
    let Ok(s) = read_settings(settings) else {
        return (false, false);
    };
    let text = s.to_compact();
    let any = text.contains("squeeze") && text.contains(" hook ");
    let all = HOOKS
        .iter()
        .all(|(_, _, sub)| text.contains(&format!("hook {}", sub)));
    (any, all)
}

fn write_settings(p: &Path, v: &Value, dry: bool) -> Result<(), String> {
    let text = v.to_pretty() + "\n";
    if dry {
        println!("--- {} (dry run) ---\n{}", p.display(), text);
        return Ok(());
    }
    if p.exists() {
        std::fs::copy(p, p.with_extension("json.squeeze.bak")).map_err(|e| e.to_string())?;
    }
    std::fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(p, text).map_err(|e| e.to_string())
}

pub fn init(o: &Options) -> Result<(), String> {
    let settings = o.dir.join(".claude").join("settings.json");
    add_hooks(&settings, o.bin, o.dry_run)?;
    println!("hooks      .claude/settings.json");
    if !o.dry_run {
        match config::write_default(o.dir) {
            Ok(true) => println!("config     .squeeze/config.json"),
            Ok(false) => println!("config     .squeeze/config.json (kept existing)"),
            Err(e) => return Err(e.to_string()),
        }
    }
    if ensure_line(&o.dir.join(".gitignore"), ".squeeze/", o.dry_run)? {
        println!("gitignore  added .squeeze/ (raw logs can contain secrets)");
    }
    if add_snippet(&o.dir.join("CLAUDE.md"), o.dry_run)? {
        println!("CLAUDE.md  added a short squeeze note");
    }
    Ok(())
}

pub fn uninstall(o: &Options) -> Result<(), String> {
    let settings = o.dir.join(".claude").join("settings.json");
    if remove_hooks(&settings, o.dry_run)? {
        println!("hooks      removed from .claude/settings.json");
    }
    if remove_snippet(&o.dir.join("CLAUDE.md"), o.dry_run)? {
        println!("CLAUDE.md  removed squeeze note");
    }
    if o.purge {
        if o.dry_run {
            println!("would delete .squeeze/");
        } else if std::fs::remove_dir_all(o.dir.join(config::DIR)).is_ok() {
            println!("deleted    .squeeze/");
        }
    } else {
        println!("kept       .squeeze/ (use --purge to delete logs and ledger)");
    }
    Ok(())
}

fn ensure_line(p: &Path, line: &str, dry: bool) -> Result<bool, String> {
    let cur = std::fs::read_to_string(p).unwrap_or_default();
    let bare = line.trim_end_matches('/');
    if cur.lines().any(|l| l.trim() == line || l.trim() == bare) {
        return Ok(false);
    }
    if dry {
        return Ok(true);
    }
    let mut s = cur;
    if !s.is_empty() && !s.ends_with('\n') {
        s.push('\n');
    }
    s.push_str(line);
    s.push('\n');
    std::fs::write(p, s).map_err(|e| e.to_string())?;
    Ok(true)
}

fn add_snippet(p: &Path, dry: bool) -> Result<bool, String> {
    let cur = std::fs::read_to_string(p).unwrap_or_default();
    if cur.contains(START) {
        return Ok(false);
    }
    if dry {
        return Ok(true);
    }
    let mut s = cur;
    if !s.is_empty() {
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s.push('\n');
    }
    s.push_str(&snippet());
    std::fs::write(p, s).map_err(|e| e.to_string())?;
    Ok(true)
}

fn remove_snippet(p: &Path, dry: bool) -> Result<bool, String> {
    let Ok(s) = std::fs::read_to_string(p) else {
        return Ok(false);
    };
    let (Some(i), Some(j)) = (s.find(START), s.find(END)) else {
        return Ok(false);
    };
    if j < i {
        return Ok(false);
    }
    if dry {
        return Ok(true);
    }
    let rest = format!(
        "{}\n{}",
        s[..i].trim_end_matches('\n'),
        s[j + END.len()..].trim_start_matches('\n')
    );
    if rest.trim().is_empty() {
        std::fs::remove_file(p).map_err(|e| e.to_string())?;
    } else {
        std::fs::write(p, rest).map_err(|e| e.to_string())?;
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_uninstall_keep_user_settings() {
        let d = std::env::temp_dir().join(format!("squeeze-install-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join(".claude")).unwrap();
        std::fs::write(d.join(".claude/settings.json"), r#"{"model":"opus","hooks":{"PreToolUse":[{"matcher":"Edit","hooks":[{"type":"command","command":"my-lint"}]}]}}"#).unwrap();
        std::fs::write(d.join("CLAUDE.md"), "# Project rules\n").unwrap();
        let o = Options {
            dir: &d,
            bin: "squeeze",
            dry_run: false,
            purge: false,
        };
        init(&o).unwrap();
        init(&o).unwrap(); // idempotent
        let s = std::fs::read_to_string(d.join(".claude/settings.json")).unwrap();
        assert_eq!(s.matches("squeeze hook pre-bash").count(), 1, "{}", s);
        assert!(s.contains("my-lint") && s.contains("\"model\": \"opus\""));
        let md = std::fs::read_to_string(d.join("CLAUDE.md")).unwrap();
        assert_eq!(md.matches(START).count(), 1);
        assert!(std::fs::read_to_string(d.join(".gitignore"))
            .unwrap()
            .contains(".squeeze/"));
        assert!(d.join(".squeeze/config.json").exists());

        uninstall(&o).unwrap();
        let s = std::fs::read_to_string(d.join(".claude/settings.json")).unwrap();
        assert!(!s.contains("squeeze") && s.contains("my-lint"), "{}", s);
        assert_eq!(
            std::fs::read_to_string(d.join("CLAUDE.md")).unwrap(),
            "# Project rules\n"
        );
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn refuses_broken_settings() {
        let d = std::env::temp_dir().join(format!("squeeze-broken-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join(".claude")).unwrap();
        std::fs::write(d.join(".claude/settings.json"), "{oops").unwrap();
        assert!(init(&Options {
            dir: &d,
            bin: "squeeze",
            dry_run: false,
            purge: false
        })
        .is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
}
