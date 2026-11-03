//! Loads `.squeeze/config.json` and finds the project root.

use crate::compress::Options;
use crate::json::{self, Value};
use std::path::{Path, PathBuf};

pub const DIR: &str = ".squeeze";

#[derive(Debug, Clone)]
pub struct Config {
    pub compress: Options,
    /// Redirect noisy shell commands to `squeeze run`.
    pub guard_bash: bool,
    /// Redirect whole-file reads of large files to a ranged read.
    pub guard_read: bool,
    pub read_max_lines: usize,
    /// Extra command prefixes the guard treats as noisy.
    pub extra_noisy: Vec<String>,
    /// Command prefixes the guard never touches.
    pub quiet: Vec<String>,
    /// How many raw logs to keep in `.squeeze/raw`.
    pub keep_raw: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            compress: Options::default(),
            guard_bash: true,
            guard_read: true,
            read_max_lines: 400,
            extra_noisy: Vec::new(),
            quiet: Vec::new(),
            keep_raw: 50,
        }
    }
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
        let strs = |v: &[String]| Value::Arr(v.iter().map(|s| Value::str(s.clone())).collect());
        let mut v = Value::obj();
        v.set("compress", c);
        v.set("guard_bash", Value::Bool(self.guard_bash));
        v.set("guard_read", Value::Bool(self.guard_read));
        v.set("read_max_lines", Value::num(self.read_max_lines));
        v.set("extra_noisy", strs(&self.extra_noisy));
        v.set("quiet", strs(&self.quiet));
        v.set("keep_raw", Value::num(self.keep_raw));
        v
    }

    fn merge(&mut self, v: &Value) {
        let pos = |x: Option<&Value>| {
            x.and_then(|x| x.as_i64())
                .filter(|n| *n > 0)
                .map(|n| n as usize)
        };
        if let Some(c) = v.get("compress") {
            let o = &mut self.compress;
            if let Some(n) = pos(c.get("max_lines")) {
                o.max_lines = n
            }
            if let Some(n) = pos(c.get("max_bytes")) {
                o.max_bytes = n
            }
            if let Some(n) = pos(c.get("head_lines")) {
                o.head_lines = n
            }
            if let Some(n) = pos(c.get("tail_lines")) {
                o.tail_lines = n
            }
            if let Some(n) = pos(c.get("context")) {
                o.context = n
            }
            if let Some(n) = pos(c.get("budget")) {
                o.budget = n
            }
        }
        if let Some(b) = v.get("guard_bash").and_then(|x| x.as_bool()) {
            self.guard_bash = b
        }
        if let Some(b) = v.get("guard_read").and_then(|x| x.as_bool()) {
            self.guard_read = b
        }
        if let Some(n) = pos(v.get("read_max_lines")) {
            self.read_max_lines = n
        }
        if let Some(n) = pos(v.get("keep_raw")) {
            self.keep_raw = n
        }
        let strs = |x: Option<&Value>| -> Vec<String> {
            x.and_then(|x| x.as_arr())
                .map(|a| {
                    a.iter()
                        .filter_map(|s| s.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        };
        self.extra_noisy.extend(strs(v.get("extra_noisy")));
        self.quiet.extend(strs(v.get("quiet")));
    }
}

/// Walk up from `dir` looking for `.squeeze`, then `.git`. Falls back to `dir`.
pub fn root(dir: &Path) -> PathBuf {
    let abs = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    for marker in [DIR, ".git"] {
        let mut d: Option<&Path> = Some(&abs);
        while let Some(p) = d {
            if p.join(marker).is_dir() {
                return p.to_path_buf();
            }
            d = p.parent();
        }
    }
    abs
}

/// Read `<root>/.squeeze/config.json`, filling gaps with defaults.
pub fn load(root: &Path) -> Config {
    let mut c = Config::default();
    if let Ok(s) = std::fs::read_to_string(root.join(DIR).join("config.json")) {
        if let Ok(v) = json::parse(&s) {
            c.merge(&v);
        }
    }
    c
}

/// Write the default config. Returns false if one already exists.
pub fn write_default(root: &Path) -> std::io::Result<bool> {
    let p = root.join(DIR).join("config.json");
    if p.exists() {
        return Ok(false);
    }
    std::fs::create_dir_all(p.parent().unwrap())?;
    std::fs::write(&p, Config::default().to_json().to_pretty() + "\n")?;
    Ok(true)
}

/// User-level state directory for commands that run outside a project
/// (`squeeze clip`): `~/.squeeze`.
pub fn global_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(DIR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_overrides_only_given_keys() {
        let mut c = Config::default();
        c.merge(
            &json::parse(
                r#"{"guard_bash":false,"compress":{"max_lines":10},"quiet":["make docs"]}"#,
            )
            .unwrap(),
        );
        assert!(!c.guard_bash && c.guard_read);
        assert_eq!(c.compress.max_lines, 10);
        assert_eq!(c.compress.budget, 120);
        assert_eq!(c.quiet, vec!["make docs"]);
    }
}
