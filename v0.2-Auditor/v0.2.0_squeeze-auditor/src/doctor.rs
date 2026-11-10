//! `squeeze doctor`: what is attached to every message before you type a
//! word (memory files, skill and agent descriptions, MCP tool definitions),
//! with tips to make it smaller.

use crate::json::Value;
use crate::ledger::tokens;
use crate::mcp;
use crate::paths;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Item {
    /// memory | import | skill | agent | mcp
    pub kind: String,
    pub name: String,
    pub path: String,
    pub tokens: usize,
    /// For MCP servers: id, enabled, measured.
    pub id: String,
    pub enabled: bool,
    pub measured: bool,
    pub note: String,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub target: String,
    pub items: Vec<Item>,
    pub total: usize,
    pub tips: Vec<String>,
}

fn item(kind: &str, name: &str, path: &Path, tokens: usize) -> Item {
    Item {
        kind: kind.into(),
        name: name.into(),
        path: paths::tilde(path),
        tokens,
        id: String::new(),
        enabled: true,
        measured: true,
        note: String::new(),
    }
}

/// `@path` imports in a memory file (one level, like Claude Code).
fn imports(file: &Path, text: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut in_code = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }
        for word in line.split_whitespace() {
            let Some(rest) = word.strip_prefix('@') else {
                continue;
            };
            let rest = rest.trim_end_matches([',', '.', ')', ';']);
            if rest.is_empty() || rest.contains('@') || !(rest.contains('/') || rest.contains('.'))
            {
                continue;
            }
            let p = if let Some(h) = rest.strip_prefix("~/") {
                paths::home().join(h)
            } else if Path::new(rest).is_absolute() {
                PathBuf::from(rest)
            } else {
                file.parent().unwrap_or(Path::new(".")).join(rest)
            };
            if p.is_file() && !out.contains(&p) {
                out.push(p);
            }
        }
    }
    out
}

fn add_memory(items: &mut Vec<Item>, tips: &mut Vec<String>, p: &Path) {
    let Ok(text) = std::fs::read_to_string(p) else {
        return;
    };
    let t = tokens(text.len());
    items.push(item(
        "memory",
        &p.file_name().unwrap_or_default().to_string_lossy(),
        p,
        t,
    ));
    if t > 2000 {
        tips.push(format!(
            "{} is about {} tokens and is sent with every message. Move rarely needed parts into separate files and mention them by path so they are read only when needed.",
            paths::tilde(p),
            t
        ));
    }
    let mut seen = std::collections::HashSet::new();
    let dups = text
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 30)
        .filter(|l| !seen.insert(*l))
        .count();
    if dups > 0 {
        tips.push(format!(
            "{} has {} repeated line(s). Removing duplicates is a free saving.",
            paths::tilde(p),
            dups
        ));
    }
    for imp in imports(p, &text) {
        if let Ok(it) = std::fs::read_to_string(&imp) {
            items.push(item(
                "import",
                &format!("@{}", imp.file_name().unwrap_or_default().to_string_lossy()),
                &imp,
                tokens(it.len()),
            ));
        }
    }
}

/// Only the frontmatter `name` and `description` of skills and agents are
/// loaded up front; the body is read when used.
fn frontmatter_tokens(p: &Path) -> Option<(String, usize)> {
    let text = std::fs::read_to_string(p).ok()?;
    let body = text.strip_prefix("---")?;
    let end = body.find("\n---")?;
    let fm = &body[..end];
    let mut name = String::new();
    let mut size = 0;
    for l in fm.lines() {
        if let Some(v) = l.strip_prefix("name:") {
            name = v.trim().trim_matches('"').to_string();
            size += v.len();
        } else if let Some(v) = l.strip_prefix("description:") {
            size += v.len();
        }
    }
    Some((name, tokens(size.max(1)) + 10))
}

fn add_dir_entries(items: &mut Vec<Item>, kind: &str, dir: &Path) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let file = if p.is_dir() {
            p.join("SKILL.md")
        } else {
            p.clone()
        };
        if file.extension().is_some_and(|e| e == "md") {
            if let Some((name, t)) = frontmatter_tokens(&file) {
                let n = if name.is_empty() {
                    file.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                } else {
                    name
                };
                items.push(item(kind, &n, &file, t));
            }
        }
    }
}

fn add_mcp(
    items: &mut Vec<Item>,
    tips: &mut Vec<String>,
    servers: Vec<mcp::Server>,
    last_used: &std::collections::HashMap<String, u64>,
) {
    let now = crate::ledger::now();
    for s in servers {
        let m = mcp::cached_measure(&s.id());
        let measured = m.as_ref().is_some_and(|m| m.error.is_empty());
        let t = m.as_ref().map(|m| m.tokens).unwrap_or(0);
        let used = last_used
            .get(&crate::transcripts::normalize_server(&s.name))
            .copied();
        let note = match (s.scope == mcp::Scope::Desktop, used) {
            (true, _) => String::new(),
            (false, Some(u)) => format!("last used {} day(s) ago", now.saturating_sub(u) / 86_400),
            (false, None) => "no use found in this project".into(),
        };
        if s.enabled && measured && t > 1500 && s.scope != mcp::Scope::Desktop && used.is_none() {
            tips.push(format!(
                "MCP server {:?} adds about {} tokens to every message and has not been used in this project. Turn it off here or with /mcp.",
                s.name, t
            ));
        }
        items.push(Item {
            kind: "mcp".into(),
            name: s.name.clone(),
            path: if s.command.is_empty() {
                s.url.clone()
            } else {
                format!("{} {}", s.command, s.args.join(" "))
            },
            tokens: if s.enabled { t } else { 0 },
            id: s.id(),
            enabled: s.enabled,
            measured,
            note: if !measured && s.enabled {
                m.map(|m| m.error)
                    .filter(|e| !e.is_empty())
                    .unwrap_or_else(|| "not measured yet".into())
            } else {
                note
            },
        });
    }
}

fn finish(mut r: Report) -> Report {
    r.total = r.items.iter().filter(|i| i.enabled).map(|i| i.tokens).sum();
    if r.items
        .iter()
        .any(|i| i.kind == "mcp" && i.enabled && !i.measured)
    {
        r.tips.push("Some MCP servers are not measured yet. Measure them in squeeze-Auditor (Fixed cost tab) to see what they cost.".into());
    }
    r
}

/// Fixed cost of a Claude Code session in `root`.
pub fn claude_code(root: &Path) -> Report {
    let mut r = Report {
        target: format!("Claude Code · {}", paths::tilde(root)),
        ..Default::default()
    };
    let claude = paths::claude_dir();
    add_memory(&mut r.items, &mut r.tips, &claude.join("CLAUDE.md"));
    // CLAUDE.md files from the project up to (not including) home
    let home = std::fs::canonicalize(paths::home()).unwrap_or_else(|_| paths::home());
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut d = Some(root);
    while let Some(p) = d {
        if p == home {
            break;
        }
        dirs.push(p.to_path_buf());
        d = p.parent();
    }
    dirs.reverse();
    for dir in &dirs {
        for f in ["CLAUDE.md", "CLAUDE.local.md", ".claude/CLAUDE.md"] {
            let p = dir.join(f);
            if p.is_file() {
                add_memory(&mut r.items, &mut r.tips, &p);
            }
        }
    }
    add_dir_entries(&mut r.items, "skill", &claude.join("skills"));
    add_dir_entries(&mut r.items, "skill", &root.join(".claude/skills"));
    add_dir_entries(&mut r.items, "agent", &claude.join("agents"));
    add_dir_entries(&mut r.items, "agent", &root.join(".claude/agents"));
    let used = crate::transcripts::mcp_last_used(root);
    add_mcp(
        &mut r.items,
        &mut r.tips,
        mcp::claude_code_servers(root),
        &used,
    );
    // squeeze hooks installed twice would run twice
    let global = std::fs::read_to_string(paths::claude_settings()).unwrap_or_default();
    let local = std::fs::read_to_string(root.join(".claude/settings.json")).unwrap_or_default();
    if global.contains("squeeze")
        && global.contains(" hook ")
        && local.contains("squeeze")
        && local.contains(" hook ")
    {
        r.tips.push("squeeze hooks are installed both for all projects and in this project, so they run twice. Run `squeeze uninstall` in this project.".into());
    }
    finish(r)
}

/// Fixed cost of the Claude desktop app (MCP servers).
pub fn desktop() -> Report {
    let mut r = Report {
        target: "Claude desktop app".into(),
        ..Default::default()
    };
    if paths::desktop_config().is_none() {
        r.tips
            .push("The Claude desktop app is not available on this OS.".into());
        return r;
    }
    add_mcp(
        &mut r.items,
        &mut r.tips,
        mcp::desktop_servers(),
        &Default::default(),
    );
    if r.items.iter().filter(|i| i.enabled).count() > 3 {
        r.tips.push("Every connected MCP server's tools ride along on every desktop chat message. Keep only the ones you use often turned on.".into());
    }
    finish(r)
}

pub fn to_json(r: &Report) -> Value {
    let mut v = Value::obj();
    v.set("target", Value::str(r.target.clone()));
    v.set("total_tokens", Value::num(r.total));
    v.set(
        "items",
        Value::Arr(
            r.items
                .iter()
                .map(|i| {
                    let mut x = Value::obj();
                    x.set("kind", Value::str(i.kind.clone()));
                    x.set("name", Value::str(i.name.clone()));
                    x.set("path", Value::str(i.path.clone()));
                    x.set("tokens", Value::num(i.tokens));
                    x.set("id", Value::str(i.id.clone()));
                    x.set("enabled", Value::Bool(i.enabled));
                    x.set("measured", Value::Bool(i.measured));
                    x.set("note", Value::str(i.note.clone()));
                    x
                })
                .collect(),
        ),
    );
    v.set(
        "tips",
        Value::Arr(r.tips.iter().map(|t| Value::str(t.clone())).collect()),
    );
    v
}

pub fn print(r: &Report) {
    println!("{}", r.target);
    println!("Sent with every message: about {} tokens\n", r.total);
    for i in &r.items {
        let state = if !i.enabled {
            " (off)".to_string()
        } else if !i.measured || !i.note.is_empty() {
            format!(" ({})", i.note)
        } else {
            String::new()
        };
        println!("  {:>7}  {:<7} {}{}", i.tokens, i.kind, i.name, state);
    }
    if !r.tips.is_empty() {
        println!("\nTips:");
        for t in &r.tips {
            println!("  - {}", t);
        }
    }
    println!("\nToken counts are estimates (bytes / 4).");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::tmp_project;

    #[test]
    fn counts_memory_imports_and_tips() {
        let root = tmp_project("doctor");
        std::fs::write(root.join("notes.md"), "x".repeat(4000)).unwrap();
        let repeated = "Always run the full test suite before committing changes here.";
        std::fs::write(
            root.join("CLAUDE.md"),
            format!(
                "# Rules\n{r}\n{r}\nSee @notes.md for details.\n{}",
                "y".repeat(9000),
                r = repeated
            ),
        )
        .unwrap();
        std::fs::create_dir_all(root.join(".claude/skills/deploy")).unwrap();
        std::fs::write(
            root.join(".claude/skills/deploy/SKILL.md"),
            "---\nname: deploy\ndescription: Deploy the app to staging\n---\nlong body ".repeat(3),
        )
        .unwrap();
        let r = claude_code(&root);
        assert!(
            r.items
                .iter()
                .any(|i| i.kind == "import" && i.tokens == 1000),
            "{:?}",
            r.items
        );
        assert!(r
            .items
            .iter()
            .any(|i| i.kind == "skill" && i.name == "deploy"));
        assert!(r.tips.iter().any(|t| t.contains("repeated")));
        assert!(r.tips.iter().any(|t| t.contains("every message")));
        assert!(r.total > 3000);
    }
}
