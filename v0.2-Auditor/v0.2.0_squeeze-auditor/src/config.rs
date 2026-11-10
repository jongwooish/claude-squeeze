//! Settings: built-in defaults, then `~/.squeeze/config.json` (global, edited
//! by squeeze-Auditor), then `<project>/.squeeze/config.json` (overrides).

use crate::compress::Options;
use crate::json::{self, Value};
use crate::paths;
use std::path::{Path, PathBuf};

pub const DIR: &str = ".squeeze";

/// Profiles offered in squeeze-Auditor.
pub const PROFILES: &[&str] = &["strong", "balanced", "light", "custom"];

#[derive(Debug, Clone, PartialEq)]
pub struct Budget {
    pub enabled: bool,
    /// Pause before sending a prompt once the session context is this big.
    pub tokens: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Handoff {
    pub enabled: bool,
    /// "note": ask Claude to write a handoff note, then load it after /clear.
    /// "remind": only remind the user to /compact or /clear.
    pub mode: String,
    pub tokens: u64,
    /// Handoff notes older than this are not loaded into a new session.
    pub max_age_hours: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prune {
    /// Turn off MCP servers that a project has not used for `unused_days`.
    pub enabled: bool,
    pub unused_days: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Desktop {
    /// Folders the desktop-app tools may read. Empty means nothing.
    pub allowed_dirs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub profile: String,
    pub compress: Options,
    pub guard_bash: bool,
    pub guard_read: bool,
    pub read_max_lines: usize,
    pub extra_noisy: Vec<String>,
    pub quiet: Vec<String>,
    pub keep_raw: usize,
    /// Block a full re-read of a file that has not changed this session.
    pub dup_read: bool,
    pub budget: Budget,
    pub handoff: Handoff,
    pub mcp_prune: Prune,
    pub desktop: Desktop,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            profile: "balanced".into(),
            compress: Options::default(),
            guard_bash: true,
            guard_read: true,
            read_max_lines: 400,
            extra_noisy: Vec::new(),
            quiet: Vec::new(),
            keep_raw: 50,
            dup_read: true,
            budget: Budget {
                enabled: true,
                tokens: 150_000,
            },
            handoff: Handoff {
                enabled: true,
                mode: "note".into(),
                tokens: 120_000,
                max_age_hours: 48,
            },
            mcp_prune: Prune {
                enabled: false,
                unused_days: 14,
            },
            desktop: Desktop {
                allowed_dirs: Vec::new(),
            },
        }
    }
}

/// The values a profile sets. "custom" keeps the current values.
pub fn preset(name: &str, current: &Config) -> Config {
    let mut c = Config {
        extra_noisy: current.extra_noisy.clone(),
        quiet: current.quiet.clone(),
        desktop: current.desktop.clone(),
        mcp_prune: current.mcp_prune.clone(),
        ..Default::default()
    };
    match name {
        "strong" => {
            c.compress = Options {
                max_lines: 40,
                max_bytes: 4000,
                head_lines: 5,
                tail_lines: 12,
                context: 1,
                budget: 70,
            };
            c.read_max_lines = 250;
            c.budget.tokens = 100_000;
            c.handoff.tokens = 80_000;
        }
        "light" => {
            c.compress = Options {
                max_lines: 150,
                max_bytes: 16000,
                head_lines: 12,
                tail_lines: 30,
                context: 3,
                budget: 200,
            };
            c.read_max_lines = 800;
            c.dup_read = true;
            c.budget.tokens = 200_000;
            c.handoff.tokens = 160_000;
        }
        "custom" => {
            return Config {
                profile: "custom".into(),
                ..current.clone()
            }
        }
        _ => {}
    }
    c.profile = if PROFILES.contains(&name) {
        name.to_string()
    } else {
        "balanced".into()
    };
    c
}

fn strs(v: &[String]) -> Value {
    Value::Arr(v.iter().map(|s| Value::str(s.clone())).collect())
}

impl Config {
    pub fn to_json(&self) -> Value {
        let o = &self.compress;
        let mut c = Value::obj();
        c.set("max_lines", Value::num(o.max_lines));
        c.set("max_bytes", Value::num(o.max_bytes));
        c.set("head_lines", Value::num(o.head_lines));
        c.set("tail_lines", Value::num(o.tail_lines));
        c.set("context", Value::num(o.context));
        c.set("budget", Value::num(o.budget));
        let mut b = Value::obj();
        b.set("enabled", Value::Bool(self.budget.enabled));
        b.set("tokens", Value::num(self.budget.tokens));
        let mut h = Value::obj();
        h.set("enabled", Value::Bool(self.handoff.enabled));
        h.set("mode", Value::str(self.handoff.mode.clone()));
        h.set("tokens", Value::num(self.handoff.tokens));
        h.set("max_age_hours", Value::num(self.handoff.max_age_hours));
        let mut p = Value::obj();
        p.set("enabled", Value::Bool(self.mcp_prune.enabled));
        p.set("unused_days", Value::num(self.mcp_prune.unused_days));
        let mut d = Value::obj();
        d.set("allowed_dirs", strs(&self.desktop.allowed_dirs));
        let mut v = Value::obj();
        v.set("profile", Value::str(self.profile.clone()));
        v.set("compress", c);
        v.set("guard_bash", Value::Bool(self.guard_bash));
        v.set("guard_read", Value::Bool(self.guard_read));
        v.set("read_max_lines", Value::num(self.read_max_lines));
        v.set("extra_noisy", strs(&self.extra_noisy));
        v.set("quiet", strs(&self.quiet));
        v.set("keep_raw", Value::num(self.keep_raw));
        v.set("dup_read", Value::Bool(self.dup_read));
        v.set("budget", b);
        v.set("handoff", h);
        v.set("mcp_prune", p);
        v.set("desktop", d);
        v
    }

    /// Apply the keys present in `v` on top of the current values.
    pub fn merge(&mut self, v: &Value) {
        let pos = |x: Option<&Value>| {
            x.and_then(|x| x.as_i64())
                .filter(|n| *n > 0)
                .map(|n| n as u64)
        };
        let b = |x: Option<&Value>| x.and_then(|x| x.as_bool());
        let list = |x: Option<&Value>| -> Option<Vec<String>> {
            x.and_then(|x| x.as_arr()).map(|a| {
                a.iter()
                    .filter_map(|s| s.as_str().map(str::to_string))
                    .filter(|s| !s.trim().is_empty())
                    .collect()
            })
        };
        if let Some(p) = v.get("profile").and_then(|p| p.as_str()) {
            if PROFILES.contains(&p) {
                self.profile = p.to_string();
            }
        }
        if let Some(c) = v.get("compress") {
            let o = &mut self.compress;
            if let Some(n) = pos(c.get("max_lines")) {
                o.max_lines = n as usize
            }
            if let Some(n) = pos(c.get("max_bytes")) {
                o.max_bytes = n as usize
            }
            if let Some(n) = pos(c.get("head_lines")) {
                o.head_lines = n as usize
            }
            if let Some(n) = pos(c.get("tail_lines")) {
                o.tail_lines = n as usize
            }
            if let Some(n) = pos(c.get("context")) {
                o.context = n as usize
            }
            if let Some(n) = pos(c.get("budget")) {
                o.budget = n as usize
            }
        }
        if let Some(x) = b(v.get("guard_bash")) {
            self.guard_bash = x
        }
        if let Some(x) = b(v.get("guard_read")) {
            self.guard_read = x
        }
        if let Some(n) = pos(v.get("read_max_lines")) {
            self.read_max_lines = n as usize
        }
        if let Some(n) = pos(v.get("keep_raw")) {
            self.keep_raw = n as usize
        }
        if let Some(l) = list(v.get("extra_noisy")) {
            self.extra_noisy = l
        }
        if let Some(l) = list(v.get("quiet")) {
            self.quiet = l
        }
        if let Some(x) = b(v.get("dup_read")) {
            self.dup_read = x
        }
        if let Some(s) = v.get("budget") {
            if let Some(x) = b(s.get("enabled")) {
                self.budget.enabled = x
            }
            if let Some(n) = pos(s.get("tokens")) {
                self.budget.tokens = n
            }
        }
        if let Some(s) = v.get("handoff") {
            if let Some(x) = b(s.get("enabled")) {
                self.handoff.enabled = x
            }
            if let Some(m) = s.get("mode").and_then(|m| m.as_str()) {
                if m == "note" || m == "remind" {
                    self.handoff.mode = m.to_string()
                }
            }
            if let Some(n) = pos(s.get("tokens")) {
                self.handoff.tokens = n
            }
            if let Some(n) = pos(s.get("max_age_hours")) {
                self.handoff.max_age_hours = n
            }
        }
        if let Some(s) = v.get("mcp_prune") {
            if let Some(x) = b(s.get("enabled")) {
                self.mcp_prune.enabled = x
            }
            if let Some(n) = pos(s.get("unused_days")) {
                self.mcp_prune.unused_days = n
            }
        }
        if let Some(s) = v.get("desktop") {
            if let Some(l) = list(s.get("allowed_dirs")) {
                self.desktop.allowed_dirs = l
            }
        }
    }
}

/// Walk up from `dir` looking for `.squeeze`, then `.git`. Falls back to `dir`.
pub fn root(dir: &Path) -> PathBuf {
    let abs = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let home = std::fs::canonicalize(paths::home()).unwrap_or_else(|_| paths::home());
    for marker in [DIR, ".git"] {
        let mut d: Option<&Path> = Some(&abs);
        while let Some(p) = d {
            // ~/.squeeze is the global dir, not a project marker
            if p.join(marker).is_dir() && !(marker == DIR && p == home) {
                return p.to_path_buf();
            }
            d = p.parent();
        }
    }
    abs
}

pub fn global_path() -> PathBuf {
    paths::squeeze_home().join("config.json")
}

fn read_json(p: &Path) -> Option<Value> {
    json::parse(&std::fs::read_to_string(p).ok()?).ok()
}

/// Global settings only.
pub fn load_global() -> Config {
    let mut c = Config::default();
    if let Some(v) = read_json(&global_path()) {
        c.merge(&v);
    }
    c
}

/// Global settings with the project's overrides on top.
pub fn load(root: &Path) -> Config {
    let mut c = load_global();
    if let Some(v) = read_json(&root.join(DIR).join("config.json")) {
        c.merge(&v);
    }
    c
}

pub fn save_global(c: &Config) -> std::io::Result<()> {
    paths::write_atomic(&global_path(), &(c.to_json().to_pretty() + "\n"))
}

/// Create an empty project override file. Returns false if one exists.
pub fn write_default(root: &Path) -> std::io::Result<bool> {
    let p = root.join(DIR).join("config.json");
    if p.exists() {
        return Ok(false);
    }
    std::fs::create_dir_all(p.parent().unwrap())?;
    std::fs::write(&p, "{}\n")?;
    Ok(true)
}

/// Where `squeeze clip` and other non-project commands keep state.
pub fn global_dir() -> PathBuf {
    paths::squeeze_home()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_overrides_only_given_keys() {
        let mut c = Config::default();
        c.merge(
            &json::parse(r#"{"guard_bash":false,"compress":{"max_lines":10},"quiet":["make docs"],"budget":{"tokens":90000},"handoff":{"mode":"remind"}}"#)
                .unwrap(),
        );
        assert!(!c.guard_bash && c.guard_read);
        assert_eq!(c.compress.max_lines, 10);
        assert_eq!(c.compress.budget, 120);
        assert_eq!(c.quiet, vec!["make docs"]);
        assert_eq!(c.budget.tokens, 90000);
        assert!(c.budget.enabled);
        assert_eq!(c.handoff.mode, "remind");
    }

    #[test]
    fn round_trip_and_presets() {
        let c = preset("strong", &Config::default());
        assert_eq!(c.profile, "strong");
        assert!(c.compress.max_lines < Config::default().compress.max_lines);
        let mut back = Config::default();
        back.merge(&c.to_json());
        assert_eq!(back, c);
        let mut cur = Config {
            quiet: vec!["make docs".into()],
            ..Default::default()
        };
        cur.compress.max_lines = 33;
        let custom = preset("custom", &cur);
        assert_eq!(custom.compress.max_lines, 33);
        assert_eq!(preset("light", &cur).quiet, vec!["make docs"]);
    }
}
