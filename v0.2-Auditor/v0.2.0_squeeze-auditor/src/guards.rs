//! v0.2 blockers. Unlike v0.1 (which looks at one output's size), these
//! remember what happened in the session:
//!
//! - duplicate read: an unchanged file read in full again is blocked
//! - budget: a prompt is paused once the session context passes a budget
//! - handoff: a long session writes a handoff note, loaded after /clear
//! - fixed cost: idle MCP servers are turned off (see `mcp::auto_prune`)
//!
//! Every hook fails open: bad input or any error means no output.

use crate::config::{self, Config};
use crate::json::{self, Value};
use crate::ledger::{self, Event};
use crate::paths;
use crate::transcripts;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// ---------- session state ----------

#[derive(Debug, Default, Clone)]
pub struct FileSeen {
    pub mtime: u64,
    pub size: u64,
    pub hash: u64,
    pub at: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Session {
    pub reads: Vec<(String, FileSeen)>,
    /// Context size at the last budget pause (0 = never paused).
    pub budget_warned: u64,
    /// "" | "requested" | "announced" | "reminded"
    pub handoff: String,
    pub handoff_note: String,
    pub handoff_tokens: u64,
}

fn state_dir() -> PathBuf {
    paths::squeeze_home().join("sessions")
}

fn safe_id(id: &str) -> String {
    let s: String = id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(80)
        .collect();
    if s.is_empty() {
        "unknown".into()
    } else {
        s
    }
}

impl Session {
    pub fn load(id: &str) -> Session {
        let p = state_dir().join(format!("{}.json", safe_id(id)));
        let Some(v) = std::fs::read_to_string(p)
            .ok()
            .and_then(|s| json::parse(&s).ok())
        else {
            return Session::default();
        };
        let n = |x: Option<&Value>| x.and_then(|x| x.as_i64()).unwrap_or(0).max(0) as u64;
        let mut s = Session {
            budget_warned: n(v.get("budget_warned")),
            handoff: v
                .get("handoff")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .into(),
            handoff_note: v
                .get("handoff_note")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .into(),
            handoff_tokens: n(v.get("handoff_tokens")),
            ..Default::default()
        };
        if let Some(Value::Obj(kv)) = v.get("reads") {
            for (k, r) in kv {
                s.reads.push((
                    k.clone(),
                    FileSeen {
                        mtime: n(r.get("m")),
                        size: n(r.get("s")),
                        hash: r
                            .get("h")
                            .and_then(|x| x.as_str())
                            .and_then(|h| u64::from_str_radix(h, 16).ok())
                            .unwrap_or(0),
                        at: n(r.get("t")),
                    },
                ));
            }
        }
        s
    }

    pub fn save(&self, id: &str) {
        let mut reads = Value::obj();
        for (k, r) in &self.reads {
            let mut x = Value::obj();
            x.set("m", Value::num(r.mtime));
            x.set("s", Value::num(r.size));
            x.set("h", Value::str(format!("{:016x}", r.hash)));
            x.set("t", Value::num(r.at));
            reads.set(k, x);
        }
        let mut v = Value::obj();
        v.set("reads", reads);
        v.set("budget_warned", Value::num(self.budget_warned));
        v.set("handoff", Value::str(self.handoff.clone()));
        v.set("handoff_note", Value::str(self.handoff_note.clone()));
        v.set("handoff_tokens", Value::num(self.handoff_tokens));
        let _ = paths::write_atomic(
            &state_dir().join(format!("{}.json", safe_id(id))),
            &v.to_compact(),
        );
    }
}

/// Remove session state files older than a week.
fn prune_states() {
    let Ok(rd) = std::fs::read_dir(state_dir()) else {
        return;
    };
    let week = std::time::Duration::from_secs(7 * 86_400);
    for e in rd.flatten() {
        let old = e
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.elapsed().ok())
            .is_some_and(|age| age > week);
        if old {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

// ---------- shared helpers ----------

pub struct Input {
    pub v: Value,
    pub session: String,
    pub cwd: PathBuf,
    pub transcript: Option<PathBuf>,
}

pub fn read_input(r: &mut dyn Read) -> Option<Input> {
    let mut s = String::new();
    r.read_to_string(&mut s).ok()?;
    let v = json::parse(&s).ok()?;
    let g = |k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let cwd = g("cwd");
    let tp = g("transcript_path");
    Some(Input {
        session: g("session_id"),
        cwd: if cwd.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(cwd)
        },
        transcript: if tp.is_empty() {
            None
        } else {
            Some(PathBuf::from(tp))
        },
        v,
    })
}

fn out(w: &mut dyn Write, v: Value) {
    let _ = writeln!(w, "{}", v.to_compact());
}

fn comma(n: u64) -> String {
    let s = n.to_string();
    let mut o = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            o.push(',');
        }
        o.push(c);
    }
    o
}

fn file_stat(p: &Path) -> Option<(u64, u64)> {
    let m = std::fs::metadata(p).ok()?;
    let mtime = m
        .modified()
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some((mtime, m.len()))
}

// ---------- 1. duplicate read ----------

/// Decide on a full read of `path`. Returns a deny reason when the file was
/// already read in full this session and has not changed since. Otherwise
/// records the read and returns None.
pub fn check_dup_read(session: &str, path: &Path) -> Option<String> {
    let key = std::fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned();
    let (mtime, size) = file_stat(path)?;
    if size > 50 * 1024 * 1024 {
        return None;
    }
    let mut s = Session::load(session);
    // cheap check first; hash only when mtime and size match
    if let Some((_, seen)) = s.reads.iter().find(|(k, _)| *k == key) {
        if seen.mtime == mtime && seen.size == size {
            let hash = paths::fnv64(&std::fs::read(path).ok()?);
            // a second copy of the hook (for example v0.1 project hooks next
            // to the global ones) sees the same tool call within seconds
            if ledger::now().saturating_sub(seen.at) <= 3 {
                return None;
            }
            if hash == seen.hash {
                let mins = ledger::now().saturating_sub(seen.at) / 60;
                return Some(format!(
                    "squeeze: you already read {} in full {} in this session, and it has not changed since. \
Use what you read earlier. If you need one part again, call Read with offset and limit. \
If the earlier content is no longer in your context, read it again with limit={}.",
                    path.display(),
                    if mins == 0 { "a moment ago".to_string() } else { format!("{} min ago", mins) },
                    std::fs::read_to_string(path).map(|t| t.lines().count().max(1)).unwrap_or(2000)
                ));
            }
        }
    }
    let hash = paths::fnv64(&std::fs::read(path).ok()?);
    s.reads.retain(|(k, _)| *k != key);
    s.reads.push((
        key,
        FileSeen {
            mtime,
            size,
            hash,
            at: ledger::now(),
        },
    ));
    if s.reads.len() > 500 {
        s.reads.remove(0);
    }
    s.save(session);
    None
}

#[cfg(test)]
fn age_reads(session: &str, secs: u64) {
    let mut s = Session::load(session);
    for (_, r) in s.reads.iter_mut() {
        r.at = r.at.saturating_sub(secs);
    }
    s.save(session);
}

/// Forget reads after the context was compacted: Claude no longer has them.
pub fn reset_reads(session: &str) {
    let mut s = Session::load(session);
    if !s.reads.is_empty() {
        s.reads.clear();
        s.save(session);
    }
}

// ---------- 2. budget (UserPromptSubmit) ----------

pub fn prompt(r: &mut dyn Read, w: &mut dyn Write) {
    let Some(inp) = read_input(r) else { return };
    let prompt = inp.v.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
    // never get in the way of /compact, /clear or any other slash command
    if prompt.trim_start().starts_with('/') || prompt.contains("SQUEEZE_SKIP") {
        return;
    }
    let root = config::root(&inp.cwd);
    let cfg = config::load(&root);
    if !cfg.budget.enabled {
        return;
    }
    let Some(ctx) = inp
        .transcript
        .as_deref()
        .and_then(transcripts::context_tokens)
    else {
        return;
    };
    let mut s = Session::load(&inp.session);
    if ctx < cfg.budget.tokens
        || (s.budget_warned > 0 && ctx < s.budget_warned + cfg.budget.tokens / 2)
    {
        return;
    }
    s.budget_warned = ctx;
    s.save(&inp.session);
    let note = if !s.handoff_note.is_empty() && Path::new(&s.handoff_note).is_file() {
        format!(
            "A handoff note is ready ({}). Run /clear to continue from it in a fresh session.",
            paths::tilde(Path::new(&s.handoff_note))
        )
    } else {
        "Run /compact, or /clear to start fresh.".to_string()
    };
    let mut v = Value::obj();
    v.set("decision", Value::str("block"));
    v.set(
        "reason",
        Value::str(format!(
            "squeeze budget: this session is about {} tokens, and every message re-reads all of it. {} \
To send this message anyway, send it again (press ↑ to bring it back).",
            comma(ctx),
            note
        )),
    );
    out(w, v);
    let _ = ledger::record(
        Some(&root),
        Event {
            kind: "budget".into(),
            subject: format!(
                "session {} at {} tokens",
                inp.session.chars().take(8).collect::<String>(),
                ctx
            ),
            ..Default::default()
        },
    );
}

// ---------- 3. handoff (Stop + SessionStart) ----------

fn handoff_dir(root: &Path) -> PathBuf {
    root.join(config::DIR).join("handoff")
}

pub fn stop(r: &mut dyn Read, w: &mut dyn Write) {
    let Some(inp) = read_input(r) else { return };
    let root = config::root(&inp.cwd);
    let cfg = config::load(&root);
    if !cfg.handoff.enabled {
        return;
    }
    let active = inp
        .v
        .get("stop_hook_active")
        .and_then(|x| x.as_bool())
        .unwrap_or(false);
    let mut s = Session::load(&inp.session);

    // The note was requested in an earlier stop: tell the user when it exists.
    if s.handoff == "requested" {
        if Path::new(&s.handoff_note).is_file() {
            s.handoff = "announced".into();
            s.save(&inp.session);
            let note_tokens = ledger::tokens(
                std::fs::metadata(&s.handoff_note)
                    .map(|m| m.len() as usize)
                    .unwrap_or(0),
            ) as u64;
            let mut v = Value::obj();
            v.set(
                "systemMessage",
                Value::str(format!(
                    "squeeze: handoff note saved ({}). Run /clear to continue from it: about {} tokens per message instead of {}.",
                    paths::tilde(Path::new(&s.handoff_note)),
                    comma(note_tokens.max(1)),
                    comma(s.handoff_tokens)
                )),
            );
            out(w, v);
        }
        return;
    }
    if active || !s.handoff.is_empty() {
        return;
    }
    let Some(ctx) = inp
        .transcript
        .as_deref()
        .and_then(transcripts::context_tokens)
    else {
        return;
    };
    if ctx < cfg.handoff.tokens {
        return;
    }
    if cfg.handoff.mode == "remind" {
        s.handoff = "reminded".into();
        s.save(&inp.session);
        let mut v = Value::obj();
        v.set(
            "systemMessage",
            Value::str(format!(
                "squeeze: this session is about {} tokens and every message re-reads it. Consider /compact, or /clear for a fresh start.",
                comma(ctx)
            )),
        );
        out(w, v);
        return;
    }
    let note = handoff_dir(&root).join(format!(
        "{}-{}.md",
        crate::runner::stamp(ledger::now()),
        safe_id(&inp.session).chars().take(8).collect::<String>()
    ));
    s.handoff = "requested".into();
    s.handoff_note = note.to_string_lossy().into_owned();
    s.handoff_tokens = ctx;
    s.save(&inp.session);
    let mut v = Value::obj();
    v.set("decision", Value::str("block"));
    v.set(
        "reason",
        Value::str(format!(
            "squeeze handoff: this session is about {} tokens, and every new message re-reads all of it. \
Before stopping, write a handoff note so the work can continue in a fresh, small session. \
Write it to this exact file with the Write tool: {}\n\
Use these sections, short and concrete, in the language the user writes in:\n\
# Handoff\n## Goal\n## Done so far\n## Decisions and constraints\n## Files touched\n## Next steps\n## Open questions\n\
Keep it under 600 words. Do not paste code or logs; point to files instead. \
After writing it, reply with one line telling the user to run /clear to continue from the note.",
            comma(ctx),
            note.display()
        )),
    );
    out(w, v);
}

fn consumed_file() -> PathBuf {
    paths::squeeze_home().join("handoff-consumed.txt")
}

fn newest_note(root: &Path, max_age_hours: u64) -> Option<PathBuf> {
    let consumed = std::fs::read_to_string(consumed_file()).unwrap_or_default();
    let max_age = std::time::Duration::from_secs(max_age_hours * 3600);
    let mut notes: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(handoff_dir(root))
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .filter(|p| !consumed.lines().any(|l| l == p.to_string_lossy()))
        .filter_map(|p| Some((std::fs::metadata(&p).ok()?.modified().ok()?, p)))
        .filter(|(t, _)| t.elapsed().map(|a| a <= max_age).unwrap_or(false))
        .collect();
    notes.sort();
    notes.pop().map(|(_, p)| p)
}

pub fn session_start(r: &mut dyn Read, w: &mut dyn Write) {
    let Some(inp) = read_input(r) else { return };
    let source = inp
        .v
        .get("source")
        .and_then(|x| x.as_str())
        .unwrap_or("startup")
        .to_string();
    prune_states();
    if source == "compact" {
        reset_reads(&inp.session);
        return;
    }
    let root = config::root(&inp.cwd);
    let cfg = config::load(&root);
    let mut messages: Vec<String> = Vec::new();
    let mut context: Option<String> = None;

    if cfg.handoff.enabled && (source == "startup" || source == "clear") {
        if let Some(note) = newest_note(&root, cfg.handoff.max_age_hours) {
            if let Ok(text) = std::fs::read_to_string(&note) {
                if !text.trim().is_empty() {
                    let mut c = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(consumed_file())
                        .ok();
                    if let Some(f) = c.as_mut() {
                        let _ = writeln!(f, "{}", note.to_string_lossy());
                    }
                    context = Some(format!(
                        "squeeze loaded a handoff note from a previous session in this project ({}). \
Continue from it. Ask the user before redoing work it lists as done.\n\n{}",
                        paths::tilde(&note),
                        text.trim()
                    ));
                    messages.push(format!(
                        "squeeze: continuing from handoff note {}",
                        paths::tilde(&note)
                    ));
                    let _ = ledger::record(
                        Some(&root),
                        Event {
                            kind: "handoff".into(),
                            subject: paths::tilde(&note),
                            in_bytes: (handoff_tokens_for(&note) * 4) as usize,
                            out_bytes: text.len(),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
    if cfg.mcp_prune.enabled {
        if let Some(msg) = crate::mcp::auto_prune(&root, &cfg) {
            messages.push(msg);
        }
    }
    if context.is_none() && messages.is_empty() {
        return;
    }
    let mut v = Value::obj();
    if let Some(c) = context {
        let mut h = Value::obj();
        h.set("hookEventName", Value::str("SessionStart"));
        h.set("additionalContext", Value::str(c));
        v.set("hookSpecificOutput", h);
    }
    if !messages.is_empty() {
        v.set("systemMessage", Value::str(messages.join("\n")));
    }
    out(w, v);
}

/// The session size when the note was requested, from saved session states.
fn handoff_tokens_for(note: &Path) -> u64 {
    let key = note.to_string_lossy();
    let Ok(rd) = std::fs::read_dir(state_dir()) else {
        return 0;
    };
    for e in rd.flatten() {
        let id = e
            .path()
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let s = Session::load(&id);
        if s.handoff_note == key {
            return s.handoff_tokens;
        }
    }
    0
}

pub fn pre_compact(r: &mut dyn Read, _w: &mut dyn Write) {
    if let Some(inp) = read_input(r) {
        reset_reads(&inp.session);
    }
}

/// Used by `hook::pre_read` to decide on duplicate reads.
pub fn dup_enabled(cfg: &Config) -> bool {
    cfg.dup_read
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{payload, tmp_project};

    fn transcript(dir: &Path, ctx: u64) -> PathBuf {
        let f = dir.join("t.jsonl");
        std::fs::write(
            &f,
            format!(
                r#"{{"type":"assistant","timestamp":"2026-11-04T05:00:00Z","message":{{"id":"m","usage":{{"input_tokens":{},"output_tokens":5,"cache_read_input_tokens":0,"cache_creation_input_tokens":0}}}}}}"#,
                ctx
            ) + "\n",
        )
        .unwrap();
        f
    }

    #[test]
    fn dup_read_blocks_only_unchanged_full_reads() {
        let d = tmp_project("dup");
        let f = d.join("a.txt");
        std::fs::write(&f, "one\ntwo\n").unwrap();
        let sid = format!("dup-{}", std::process::id());
        assert!(check_dup_read(&sid, &f).is_none(), "first read passes");
        assert!(
            check_dup_read(&sid, &f).is_none(),
            "a second hook copy for the same call passes"
        );
        age_reads(&sid, 10);
        let r = check_dup_read(&sid, &f).expect("second read is blocked");
        assert!(r.contains("has not changed"), "{}", r);
        std::fs::write(&f, "one\ntwo\nthree\n").unwrap();
        assert!(check_dup_read(&sid, &f).is_none(), "changed file passes");
        reset_reads(&sid);
        assert!(
            check_dup_read(&sid, &f).is_none(),
            "after compaction it passes"
        );
    }

    #[test]
    fn budget_pauses_once_then_allows() {
        let d = tmp_project("budget");
        std::fs::write(
            d.join(".squeeze/config.json"),
            r#"{"budget":{"tokens":1000}}"#,
        )
        .unwrap();
        let t = transcript(&d, 5000);
        let sid = format!("bud-{}", std::process::id());
        let extra = format!(
            r#""prompt":"hello","transcript_path":{}"#,
            Value::str(t.to_string_lossy()).to_compact()
        );
        let mut o = Vec::new();
        prompt(
            &mut payload("UserPromptSubmit", &sid, &d, &extra).as_bytes(),
            &mut o,
        );
        let s = String::from_utf8(o).unwrap();
        assert!(
            s.contains("\"decision\":\"block\"") && s.contains("5,000"),
            "{}",
            s
        );
        let mut o = Vec::new();
        prompt(
            &mut payload("UserPromptSubmit", &sid, &d, &extra).as_bytes(),
            &mut o,
        );
        assert!(o.is_empty(), "second send passes");
        let slash = format!(
            r#""prompt":"/compact","transcript_path":{}"#,
            Value::str(t.to_string_lossy()).to_compact()
        );
        let mut o = Vec::new();
        prompt(
            &mut payload("UserPromptSubmit", "other", &d, &slash).as_bytes(),
            &mut o,
        );
        assert!(o.is_empty(), "slash commands always pass");
    }

    #[test]
    fn handoff_note_flow() {
        let d = tmp_project("handoff");
        std::fs::write(
            d.join(".squeeze/config.json"),
            r#"{"handoff":{"tokens":1000}}"#,
        )
        .unwrap();
        let t = transcript(&d, 9000);
        let sid = format!("ho-{}", std::process::id());
        let extra = format!(
            r#""stop_hook_active":false,"transcript_path":{}"#,
            Value::str(t.to_string_lossy()).to_compact()
        );
        let mut o = Vec::new();
        stop(&mut payload("Stop", &sid, &d, &extra).as_bytes(), &mut o);
        let s = String::from_utf8(o).unwrap();
        assert!(
            s.contains("\"decision\":\"block\"") && s.contains("# Handoff"),
            "{}",
            s
        );
        let note = Session::load(&sid).handoff_note;
        // Claude writes the note, then stops again
        std::fs::create_dir_all(Path::new(&note).parent().unwrap()).unwrap();
        std::fs::write(&note, "# Handoff\n## Goal\nship v0.2\n").unwrap();
        let extra2 = extra.replace("false", "true");
        let mut o = Vec::new();
        stop(&mut payload("Stop", &sid, &d, &extra2).as_bytes(), &mut o);
        let s = String::from_utf8(o).unwrap();
        assert!(s.contains("systemMessage") && s.contains("/clear"), "{}", s);
        // a new session after /clear gets the note as context, once
        let mut o = Vec::new();
        session_start(
            &mut payload("SessionStart", "new", &d, r#""source":"clear""#).as_bytes(),
            &mut o,
        );
        let s = String::from_utf8(o).unwrap();
        assert!(
            s.contains("additionalContext") && s.contains("ship v0.2"),
            "{}",
            s
        );
        let mut o = Vec::new();
        session_start(
            &mut payload("SessionStart", "new2", &d, r#""source":"clear""#).as_bytes(),
            &mut o,
        );
        assert!(
            !String::from_utf8(o).unwrap().contains("ship v0.2"),
            "note is used once"
        );
    }

    #[test]
    fn small_sessions_are_left_alone() {
        let d = tmp_project("small");
        let t = transcript(&d, 500);
        let extra = format!(
            r#""stop_hook_active":false,"prompt":"hi","transcript_path":{}"#,
            Value::str(t.to_string_lossy()).to_compact()
        );
        for f in [stop as fn(&mut dyn Read, &mut dyn Write), prompt] {
            let mut o = Vec::new();
            f(&mut payload("X", "small", &d, &extra).as_bytes(), &mut o);
            assert!(o.is_empty());
        }
        let mut o = Vec::new();
        prompt(&mut "garbage".as_bytes(), &mut o);
        assert!(o.is_empty());
    }
}
