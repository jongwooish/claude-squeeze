//! Records what squeeze kept out of the context (one JSON line per event)
//! and summarizes it for `squeeze report`.

use crate::compress::human;
use crate::json::{self, Value};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const FILE: &str = "ledger.jsonl";

#[derive(Debug, Clone, Default)]
pub struct Event {
    pub ts: u64,
    /// run | compress | clip | guard-bash | guard-read
    pub kind: String,
    /// The command or file path.
    pub subject: String,
    pub in_bytes: usize,
    pub out_bytes: usize,
    pub in_lines: usize,
    pub out_lines: usize,
    pub exit: Option<i32>,
}

impl Event {
    pub fn saved(&self) -> usize {
        self.in_bytes.saturating_sub(self.out_bytes)
    }
    fn to_json(&self) -> Value {
        let mut v = Value::obj();
        v.set("ts", Value::num(self.ts));
        v.set("kind", Value::str(self.kind.clone()));
        v.set("subject", Value::str(self.subject.clone()));
        v.set("in_bytes", Value::num(self.in_bytes));
        v.set("out_bytes", Value::num(self.out_bytes));
        v.set("in_lines", Value::num(self.in_lines));
        v.set("out_lines", Value::num(self.out_lines));
        if let Some(e) = self.exit {
            v.set("exit", Value::num(e));
        }
        v
    }
    fn from_json(v: &Value) -> Option<Event> {
        let n = |k: &str| v.get(k).and_then(|x| x.as_i64()).unwrap_or(0).max(0) as usize;
        Some(Event {
            ts: v.get("ts")?.as_i64()? as u64,
            kind: v.get("kind")?.as_str()?.to_string(),
            subject: v
                .get("subject")
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .to_string(),
            in_bytes: n("in_bytes"),
            out_bytes: n("out_bytes"),
            in_lines: n("in_lines"),
            out_lines: n("out_lines"),
            exit: v.get("exit").and_then(|x| x.as_i64()).map(|e| e as i32),
        })
    }
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Append one event. Callers on hook paths ignore errors: the ledger must
/// never break a session.
pub fn append(dir: &Path, mut e: Event) -> std::io::Result<()> {
    if e.ts == 0 {
        e.ts = now();
    }
    if e.subject.chars().count() > 200 {
        e.subject = e.subject.chars().take(200).collect::<String>() + "…";
    }
    std::fs::create_dir_all(dir)?;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join(FILE))?;
    writeln!(f, "{}", e.to_json().to_compact())
}

pub fn read(dir: &Path) -> Vec<Event> {
    std::fs::read_to_string(dir.join(FILE))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| json::parse(l).ok())
        .filter_map(|v| Event::from_json(&v))
        .collect()
}

/// Rough token estimate: about 4 bytes per token for English text and code.
pub fn tokens(bytes: usize) -> usize {
    bytes.div_ceil(4)
}

#[derive(Debug, Default)]
pub struct Summary {
    pub events: usize,
    pub in_bytes: usize,
    pub out_bytes: usize,
    pub saved: usize,
    pub by_kind: Vec<(String, usize, usize)>, // kind, count, saved bytes
    pub top: Vec<Event>,
}

pub fn summarize(events: &[Event], since: u64, top: usize) -> Summary {
    let mut s = Summary::default();
    let mut kept: Vec<Event> = Vec::new();
    for e in events.iter().filter(|e| e.ts >= since) {
        s.events += 1;
        s.in_bytes += e.in_bytes;
        s.out_bytes += e.out_bytes;
        s.saved += e.saved();
        match s.by_kind.iter_mut().find(|(k, _, _)| *k == e.kind) {
            Some(slot) => {
                slot.1 += 1;
                slot.2 += e.saved();
            }
            None => s.by_kind.push((e.kind.clone(), 1, e.saved())),
        }
        kept.push(e.clone());
    }
    s.by_kind.sort();
    kept.sort_by_key(|e| std::cmp::Reverse(e.saved()));
    kept.retain(|e| e.saved() > 0);
    kept.truncate(top);
    s.top = kept;
    s
}

pub fn to_json(s: &Summary) -> Value {
    let mut v = Value::obj();
    v.set("events", Value::num(s.events));
    v.set("in_bytes", Value::num(s.in_bytes));
    v.set("out_bytes", Value::num(s.out_bytes));
    v.set("saved_bytes", Value::num(s.saved));
    v.set("saved_tokens_est", Value::num(tokens(s.saved)));
    let mut k = Value::obj();
    for (kind, count, saved) in &s.by_kind {
        let mut x = Value::obj();
        x.set("events", Value::num(count));
        x.set("saved_bytes", Value::num(saved));
        k.set(kind, x);
    }
    v.set("by_kind", k);
    v.set(
        "top",
        Value::Arr(s.top.iter().map(|e| e.to_json()).collect()),
    );
    v
}

fn one_line(s: &str, n: usize) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() > n {
        s.chars().take(n - 1).collect::<String>() + "…"
    } else {
        s
    }
}

pub fn print(s: &Summary) {
    if s.events == 0 {
        println!("No events yet. Use Claude Code in this project with squeeze installed, then run `squeeze report` again.");
        return;
    }
    let pct = if s.in_bytes > 0 {
        100.0 * s.saved as f64 / s.in_bytes as f64
    } else {
        0.0
    };
    println!("Events:                  {}", s.events);
    println!(
        "Would have entered context: {:>9} (~{} tokens)",
        human(s.in_bytes),
        tokens(s.in_bytes)
    );
    println!(
        "Actually entered:           {:>9} (~{} tokens)",
        human(s.out_bytes),
        tokens(s.out_bytes)
    );
    println!(
        "Kept out:                   {:>9} (~{} tokens, {:.1}%)",
        human(s.saved),
        tokens(s.saved),
        pct
    );
    println!("\nBy kind:");
    for (k, c, saved) in &s.by_kind {
        println!("  {:<11} {:>4} events  {:>9}", k, c, human(*saved));
    }
    if !s.top.is_empty() {
        println!("\nBiggest savings:");
        for e in &s.top {
            println!(
                "  {:>9}  {:<10} {}",
                human(e.saved()),
                e.kind,
                one_line(&e.subject, 70)
            );
        }
    }
    println!("\nTokens are estimates (bytes / 4). Text kept out of the context also stays out of every later turn.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_read_summarize() {
        let dir = std::env::temp_dir().join(format!("squeeze-ledger-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        append(
            &dir,
            Event {
                kind: "run".into(),
                subject: "npm test".into(),
                in_bytes: 4000,
                out_bytes: 400,
                ..Default::default()
            },
        )
        .unwrap();
        append(
            &dir,
            Event {
                kind: "guard-bash".into(),
                subject: "npm test".into(),
                ..Default::default()
            },
        )
        .unwrap();
        let ev = read(&dir);
        assert_eq!(ev.len(), 2);
        let s = summarize(&ev, 0, 10);
        assert_eq!(s.saved, 3600);
        assert_eq!(s.top.len(), 1);
        assert_eq!(tokens(3600), 900);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
