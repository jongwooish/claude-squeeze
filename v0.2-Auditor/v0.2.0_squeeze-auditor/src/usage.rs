//! Numbers for the squeeze-Auditor dashboard: real usage from Claude Code
//! transcripts and what squeeze kept out of the context.

use crate::json::Value;
use crate::ledger::{self, Event};
use crate::paths;
use crate::transcripts::{self, Turn, Usage};
use std::collections::BTreeMap;
use std::path::PathBuf;

const HOUR: u64 = 3600;
const BLOCK: u64 = 5 * HOUR;

fn usage_json(u: &Usage) -> Value {
    let mut v = Value::obj();
    v.set("input", Value::num(u.input));
    v.set("output", Value::num(u.output));
    v.set("cache_create", Value::num(u.cache_create));
    v.set("cache_read", Value::num(u.cache_read));
    v.set("total", Value::num(u.total()));
    v
}

fn sum(turns: &[&Turn]) -> Usage {
    let mut u = Usage::default();
    for t in turns {
        u.add(&t.usage);
    }
    u
}

/// Claude Code's rolling usage windows: a window starts at the hour of the
/// first message after the previous window ended and lasts five hours.
pub fn current_block(turns: &[Turn], now: u64) -> Option<(u64, Usage, usize)> {
    let mut start = 0u64;
    let mut u = Usage::default();
    let mut n = 0;
    for t in turns {
        if start == 0 || t.ts >= start + BLOCK {
            start = t.ts - t.ts % HOUR;
            u = Usage::default();
            n = 0;
        }
        u.add(&t.usage);
        n += 1;
    }
    if start > 0 && now < start + BLOCK {
        Some((start, u, n))
    } else {
        None
    }
}

fn kind_label(k: &str) -> &'static str {
    match k {
        "run" | "compress" | "clip" => "output",
        "guard-bash" => "output",
        "guard-read" => "big-file",
        "dup-read" => "duplicate",
        "budget" => "budget",
        "handoff" => "handoff",
        "mcp-off" => "fixed-cost",
        "desktop-read" => "desktop",
        _ => "other",
    }
}

pub fn overview(days: u64) -> Value {
    let now = ledger::now();
    let since = now.saturating_sub(days * 86_400);
    let files = transcripts::files();
    let turns = transcripts::read_turns(&files, since);
    let all: Vec<&Turn> = turns.iter().collect();

    // hourly buckets (the page groups them into local days)
    let mut hourly: BTreeMap<u64, Usage> = BTreeMap::new();
    for t in &turns {
        hourly.entry(t.ts - t.ts % HOUR).or_default().add(&t.usage);
    }
    let mut by_project: BTreeMap<String, (Usage, u64, std::collections::HashSet<String>)> =
        BTreeMap::new();
    let mut by_model: BTreeMap<String, Usage> = BTreeMap::new();
    let mut sessions: BTreeMap<String, (u64, u64, String)> = BTreeMap::new(); // id -> (last ts, context, cwd)
    for t in &turns {
        let p = by_project.entry(t.cwd.clone()).or_default();
        p.0.add(&t.usage);
        p.1 = p.1.max(t.ts);
        p.2.insert(t.session.clone());
        by_model.entry(t.model.clone()).or_default().add(&t.usage);
        let s = sessions
            .entry(t.session.clone())
            .or_insert((0, 0, t.cwd.clone()));
        if t.ts >= s.0 {
            *s = (t.ts, t.usage.context(), t.cwd.clone());
        }
    }

    // savings from the ledger
    let roots: Vec<PathBuf> = by_project
        .keys()
        .filter(|k| !k.is_empty())
        .map(PathBuf::from)
        .collect();
    let events: Vec<Event> = ledger::read_all(&roots)
        .into_iter()
        .filter(|e| e.ts >= since)
        .collect();
    let mut by_kind: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    let mut saved_hourly: BTreeMap<u64, usize> = BTreeMap::new();
    for e in &events {
        let k = by_kind.entry(kind_label(&e.kind)).or_default();
        k.0 += 1;
        k.1 += ledger::tokens(e.saved());
        *saved_hourly.entry(e.ts - e.ts % HOUR).or_default() += ledger::tokens(e.saved());
    }
    let mut top: Vec<&Event> = events.iter().filter(|e| e.saved() > 0).collect();
    top.sort_by_key(|e| std::cmp::Reverse(e.saved()));
    top.truncate(12);

    let mut v = Value::obj();
    v.set("now", Value::num(now));
    v.set("days", Value::num(days));
    v.set("transcripts_found", Value::Bool(!files.is_empty()));
    v.set("messages", Value::num(turns.len()));
    v.set("usage", usage_json(&sum(&all)));
    v.set(
        "today_since",
        Value::num(now - now % 86_400), // UTC midnight; the page recomputes local days
    );
    v.set(
        "hourly",
        Value::Arr(
            hourly
                .iter()
                .map(|(h, u)| {
                    let mut x = usage_json(u);
                    x.set("t", Value::num(*h));
                    x.set(
                        "saved",
                        Value::num(saved_hourly.get(h).copied().unwrap_or(0)),
                    );
                    x
                })
                .collect(),
        ),
    );
    // saved hours without usage
    v.set(
        "saved_hourly",
        Value::Arr(
            saved_hourly
                .iter()
                .map(|(h, s)| {
                    let mut x = Value::obj();
                    x.set("t", Value::num(*h));
                    x.set("saved", Value::num(*s));
                    x
                })
                .collect(),
        ),
    );
    let mut projects: Vec<_> = by_project.into_iter().collect();
    projects.sort_by_key(|(_, (u, _, _))| std::cmp::Reverse(u.total()));
    v.set(
        "projects",
        Value::Arr(
            projects
                .iter()
                .take(20)
                .map(|(cwd, (u, last, s))| {
                    let mut x = usage_json(u);
                    x.set(
                        "path",
                        Value::str(if cwd.is_empty() {
                            "(unknown)".into()
                        } else {
                            paths::tilde(std::path::Path::new(cwd))
                        }),
                    );
                    x.set("root", Value::str(cwd.clone()));
                    x.set("last", Value::num(*last));
                    x.set("sessions", Value::num(s.len()));
                    x
                })
                .collect(),
        ),
    );
    v.set(
        "models",
        Value::Arr(
            by_model
                .iter()
                .map(|(m, u)| {
                    let mut x = usage_json(u);
                    x.set(
                        "model",
                        Value::str(if m.is_empty() { "(unknown)" } else { m }),
                    );
                    x
                })
                .collect(),
        ),
    );
    let mut recent: Vec<_> = sessions.into_iter().collect();
    recent.sort_by_key(|(_, (ts, _, _))| std::cmp::Reverse(*ts));
    v.set(
        "sessions",
        Value::Arr(
            recent
                .iter()
                .take(8)
                .map(|(id, (ts, ctx, cwd))| {
                    let mut x = Value::obj();
                    x.set("id", Value::str(id.chars().take(8).collect::<String>()));
                    x.set("last", Value::num(*ts));
                    x.set("context", Value::num(*ctx));
                    x.set("path", Value::str(paths::tilde(std::path::Path::new(cwd))));
                    x
                })
                .collect(),
        ),
    );
    // the window uses all recent messages, not just the selected range
    let window_turns = if days >= 1 {
        turns.clone()
    } else {
        transcripts::read_turns(&files, now.saturating_sub(BLOCK * 2))
    };
    match current_block(&window_turns, now) {
        Some((start, u, n)) => {
            let mut b = usage_json(&u);
            b.set("start", Value::num(start));
            b.set("end", Value::num(start + BLOCK));
            b.set("messages", Value::num(n));
            let last_hour: u64 = window_turns
                .iter()
                .filter(|t| t.ts + HOUR >= now)
                .map(|t| t.usage.total())
                .sum();
            b.set("per_minute_last_hour", Value::num(last_hour / 60));
            v.set("block", b);
        }
        None => v.set("block", Value::Null),
    }
    let mut saved = Value::obj();
    let total_saved: usize = by_kind.values().map(|(_, t)| t).sum();
    saved.set("tokens", Value::num(total_saved));
    saved.set("events", Value::num(events.len()));
    let mut kinds = Value::obj();
    for (k, (n, t)) in &by_kind {
        let mut x = Value::obj();
        x.set("events", Value::num(*n));
        x.set("tokens", Value::num(*t));
        kinds.set(k, x);
    }
    saved.set("by_kind", kinds);
    saved.set(
        "top",
        Value::Arr(
            top.iter()
                .map(|e| {
                    let mut x = Value::obj();
                    x.set("ts", Value::num(e.ts));
                    x.set("kind", Value::str(kind_label(&e.kind)));
                    x.set("subject", Value::str(e.subject.clone()));
                    x.set("tokens", Value::num(ledger::tokens(e.saved())));
                    x.set(
                        "project",
                        Value::str(paths::tilde(std::path::Path::new(&e.project))),
                    );
                    x
                })
                .collect(),
        ),
    );
    v.set("saved", saved);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(ts: u64, n: u64) -> Turn {
        Turn {
            ts,
            usage: Usage {
                input: n,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn five_hour_window() {
        let base = 1_793_768_400; // 05:00 UTC
        let turns = vec![
            t(base - 8 * HOUR, 5),
            t(base + 600, 10),
            t(base + 2 * HOUR, 20),
        ];
        let (start, u, n) = current_block(&turns, base + 3 * HOUR).unwrap();
        assert_eq!(start, base);
        assert_eq!(u.input, 30);
        assert_eq!(n, 2);
        assert!(current_block(&turns, base + 6 * HOUR).is_none());
    }
}
