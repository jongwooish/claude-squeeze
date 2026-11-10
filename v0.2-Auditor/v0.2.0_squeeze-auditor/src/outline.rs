//! Builds a short table of contents for a large text file so a model can
//! read the range it needs instead of the whole file.

use std::path::Path;

/// Line count, size, and whether the file looks like text.
pub fn stat(path: &Path) -> std::io::Result<(usize, u64, bool)> {
    let bytes = std::fs::read(path)?;
    let size = bytes.len() as u64;
    if bytes.iter().take(8000).any(|b| *b == 0) {
        return Ok((0, size, false));
    }
    let mut lines = bytes.iter().filter(|b| **b == b'\n').count();
    if !bytes.is_empty() && *bytes.last().unwrap() != b'\n' {
        lines += 1;
    }
    Ok((lines, size, true))
}

fn is_heading(ext: &str, line: &str) -> bool {
    let t = line.trim_start();
    let indent = line.len() - t.len();
    let starts = |ps: &[&str]| ps.iter().any(|p| t.starts_with(p));
    match ext {
        "md" | "markdown" => starts(&["# ", "## ", "### "]) && indent == 0,
        "go" => indent == 0 && starts(&["func ", "type "]),
        "rs" => {
            indent <= 4
                && starts(&[
                    "fn ",
                    "pub fn ",
                    "pub(crate) fn ",
                    "struct ",
                    "pub struct ",
                    "enum ",
                    "pub enum ",
                    "trait ",
                    "pub trait ",
                    "impl ",
                    "impl<",
                    "mod ",
                    "pub mod ",
                ])
        }
        "py" => indent <= 4 && starts(&["def ", "async def ", "class "]),
        "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" => {
            indent == 0
                && starts(&[
                    "function ",
                    "async function ",
                    "class ",
                    "export ",
                    "interface ",
                    "type ",
                    "enum ",
                    "const ",
                ])
                && !t.ends_with(';')
                || indent == 0 && t.starts_with("const ") && t.contains("=>")
        }
        "java" | "kt" | "cs" => {
            indent <= 4
                && starts(&[
                    "public ",
                    "private ",
                    "protected ",
                    "class ",
                    "fun ",
                    "internal ",
                ])
                && (t.ends_with('{') || t.ends_with(')'))
        }
        "c" | "h" | "cpp" | "hpp" | "cc" => {
            indent == 0
                && !t.is_empty()
                && !starts(&["#", "//", "/*", "*", "}", "{"])
                && t.contains('(')
                && !t.ends_with(';')
        }
        "sh" | "bash" => {
            indent == 0 && (t.starts_with("function ") || t.contains("() {") || t.ends_with("()"))
        }
        "yaml" | "yml" => {
            indent == 0 && !t.starts_with('#') && !t.starts_with('-') && t.contains(':')
        }
        "toml" | "ini" | "cfg" => indent == 0 && t.starts_with('['),
        "conf" => {
            (indent == 0 && t.starts_with('['))
                || (indent <= 4 && t.ends_with('{') && !t.starts_with('#'))
        }
        "tf" => {
            indent == 0
                && starts(&[
                    "resource ",
                    "module ",
                    "variable ",
                    "output ",
                    "data ",
                    "provider ",
                ])
        }
        "sql" => {
            indent == 0 && {
                let l = t.to_lowercase();
                l.starts_with("create ") || l.starts_with("alter ")
            }
        }
        _ => false,
    }
}

/// Up to `max` (line number, text) entries. Unknown file types fall back
/// to evenly spaced non-empty lines.
pub fn build(path: &Path, max: usize) -> std::io::Result<Vec<(usize, String)>> {
    let text = String::from_utf8_lossy(&std::fs::read(path)?).into_owned();
    let lines: Vec<&str> = text.lines().collect();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let mut out: Vec<(usize, String)> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| is_heading(&ext, l))
        .map(|(i, l)| (i + 1, trim(l)))
        .collect();
    if out.is_empty() {
        let step = lines.len() / max.max(1) + 1;
        out = (0..lines.len())
            .step_by(step)
            .filter(|i| !lines[*i].trim().is_empty())
            .map(|i| (i + 1, trim(lines[i])))
            .collect();
    }
    if out.len() > max {
        let n = out.len();
        out = (0..max).map(|k| out[k * n / max].clone()).collect();
    }
    Ok(out)
}

pub fn format(es: &[(usize, String)]) -> String {
    es.iter()
        .map(|(n, t)| format!("  L{:<6} {}\n", n, t))
        .collect()
}

fn trim(s: &str) -> String {
    let s = s.trim_end_matches([' ', '\t', '\r', '{']);
    if s.chars().count() > 100 {
        s.chars().take(99).collect::<String>() + "…"
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outline_of_rust_and_markdown() {
        assert!(is_heading("rs", "pub fn main() {"));
        assert!(is_heading("md", "## Install"));
        assert!(!is_heading("md", "text"));
        assert!(is_heading("conf", "server {"));
        assert!(is_heading("yaml", "services:"));
    }
}
