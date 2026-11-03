//! squeeze — keep long command output and large file reads out of Claude
//! Code's context. See README.md.

mod clip;
mod compress;
mod config;
mod hook;
mod install;
mod json;
mod ledger;
mod outline;
mod runner;

use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "squeeze — keep long command output out of Claude's context

Usage:
  squeeze init [--dir D] [--bin PATH] [--dry-run]    Install hooks into a project
  squeeze uninstall [--dir D] [--purge] [--dry-run]  Remove hooks (and logs with --purge)
  squeeze run -- <command>                           Run a command, print a short summary
  <command> 2>&1 | squeeze compress                  Compress piped output
  squeeze clip [--print]                             Compress the clipboard (for claude.ai)
  squeeze report [--since 7d] [--global] [--json]    Show what was kept out of context
  squeeze bench <dir> [--markdown]                   Replay saved logs and measure savings
  squeeze hook pre-bash|pre-read                     Claude Code hook entry points
  squeeze version                                    Print the version
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else {
        print!("{}", USAGE);
        return;
    };
    let rest = &args[1..];
    let res = match cmd.as_str() {
        "run" => exit(cmd_run(rest)),
        "compress" => exit(cmd_compress()),
        "hook" => {
            cmd_hook(rest);
            Ok(())
        }
        "init" => cmd_init(rest),
        "uninstall" => cmd_uninstall(rest),
        "clip" => clip::clip(flag(rest, "--print"), &config::load(&config::root(&cwd()))),
        "report" => cmd_report(rest),
        "bench" => cmd_bench(rest),
        "version" | "--version" | "-V" => {
            println!("squeeze {}", VERSION);
            Ok(())
        }
        "help" | "--help" | "-h" => {
            print!("{}", USAGE);
            Ok(())
        }
        other => {
            eprintln!("squeeze: unknown command {:?}\n\n{}", other, USAGE);
            exit(2)
        }
    };
    if let Err(e) = res {
        eprintln!("squeeze: {}", e);
        exit(1);
    }
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn opt<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn cmd_run(args: &[String]) -> i32 {
    let args = if args.first().map(|s| s.as_str()) == Some("--") {
        &args[1..]
    } else {
        args
    };
    if args.is_empty() {
        eprintln!("usage: squeeze run -- <command>");
        return 2;
    }
    let command = args.join(" ");
    let dir = cwd();
    runner::run(&command, &dir, &config::load(&config::root(&dir)))
}

fn cmd_compress() -> i32 {
    let mut buf = Vec::new();
    if let Err(e) = std::io::stdin().read_to_end(&mut buf) {
        eprintln!("squeeze: {}", e);
        return 1;
    }
    let raw = String::from_utf8_lossy(&buf);
    let dir = cwd();
    runner::emit(
        &raw,
        "(stdin)",
        &dir,
        "compress",
        None,
        &config::load(&config::root(&dir)),
        &mut std::io::stdout(),
    );
    0
}

fn cmd_hook(args: &[String]) {
    // a hook must never break a session: swallow panics, stay silent
    let which = args.first().cloned().unwrap_or_default();
    let _ = std::panic::catch_unwind(|| {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        match which.as_str() {
            "pre-bash" => hook::pre_bash(&mut stdin, &mut stdout),
            "pre-read" => hook::pre_read(&mut stdin, &mut stdout),
            _ => {}
        }
    });
}

fn project_dir(args: &[String]) -> Result<PathBuf, String> {
    let d = opt(args, "--dir").unwrap_or(".");
    std::fs::canonicalize(d).map_err(|e| format!("{}: {}", d, e))
}

fn cmd_init(args: &[String]) -> Result<(), String> {
    let dir = project_dir(args)?;
    let dry = flag(args, "--dry-run");
    install::init(&install::Options {
        dir: &dir,
        bin: opt(args, "--bin").unwrap_or("squeeze"),
        dry_run: dry,
        purge: false,
    })?;
    if !dry {
        println!("\nDone. Start (or restart) Claude Code in this project. Run `squeeze report` later to see the savings.");
    }
    Ok(())
}

fn cmd_uninstall(args: &[String]) -> Result<(), String> {
    let dir = project_dir(args)?;
    install::uninstall(&install::Options {
        dir: &dir,
        bin: "squeeze",
        dry_run: flag(args, "--dry-run"),
        purge: flag(args, "--purge"),
    })
}

fn parse_since(s: &str) -> Result<u64, String> {
    let (num, unit) = s.split_at(s.trim_end_matches(|c: char| c.is_ascii_alphabetic()).len());
    let n: u64 = num
        .parse()
        .map_err(|_| format!("bad --since {:?} (use e.g. 24h, 7d)", s))?;
    let secs = match unit {
        "d" => n * 86_400,
        "h" => n * 3600,
        "m" => n * 60,
        _ => return Err(format!("bad --since {:?} (use e.g. 24h, 7d)", s)),
    };
    Ok(ledger::now().saturating_sub(secs))
}

fn cmd_report(args: &[String]) -> Result<(), String> {
    let since = match opt(args, "--since") {
        Some(s) => parse_since(s)?,
        None => 0,
    };
    let dir = if flag(args, "--global") {
        config::global_dir()
    } else {
        config::root(&cwd()).join(config::DIR)
    };
    let top = opt(args, "--top")
        .and_then(|t| t.parse().ok())
        .unwrap_or(10);
    let s = ledger::summarize(&ledger::read(&dir), since, top);
    if flag(args, "--json") {
        println!("{}", ledger::to_json(&s).to_pretty());
    } else {
        ledger::print(&s);
    }
    Ok(())
}

/// Replay saved logs through the compressor; measure size and check that
/// every distinct error/warning message survives.
fn cmd_bench(args: &[String]) -> Result<(), String> {
    let md = flag(args, "--markdown");
    let dir = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .ok_or("usage: squeeze bench <dir> [--markdown]")?;
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {}", dir, e))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "log"))
        .collect();
    files.sort();
    let opts = compress::Options::default();
    let mut total = (0usize, 0usize, 0usize, 0usize);
    let pct = |a: usize, b: usize| {
        if b == 0 {
            0.0
        } else {
            100.0 - 100.0 * a as f64 / b as f64
        }
    };
    if md {
        println!("| Log | Lines before → after | Tokens before → after (est.) | Saved | Distinct errors/warnings kept |");
        println!("|---|---:|---:|---:|---:|");
    }
    for f in &files {
        let raw = std::fs::read_to_string(f).map_err(|e| e.to_string())?;
        let r = compress::compress(&raw, &opts);
        let out = if r.compressed {
            format!(
                "{}{}\n",
                r.text,
                compress::footer(&r, Some(".squeeze/raw/20261103-043000-abc123.log"), Some(1))
            )
        } else {
            raw.clone()
        };
        let mut seen: HashMap<String, bool> = HashMap::new();
        for l in compress::clean(&raw)
            .lines()
            .filter(|l| compress::is_important(l) && !l.trim().is_empty())
        {
            let hit = out.contains(l);
            let v = seen.entry(compress::shape(l)).or_insert(false);
            *v = *v || hit;
        }
        let kept = seen.values().filter(|v| **v).count();
        total = (
            total.0 + raw.len(),
            total.1 + out.len(),
            total.2 + seen.len(),
            total.3 + kept,
        );
        let name = Path::new(f).file_name().unwrap().to_string_lossy();
        if md {
            println!(
                "| `{}` | {} → {} | {} → {} | **{:.1}%** | {} / {} |",
                name,
                r.in_lines,
                r.out_lines,
                comma(ledger::tokens(raw.len())),
                comma(ledger::tokens(out.len())),
                pct(out.len(), raw.len()),
                kept,
                seen.len()
            );
        } else {
            println!(
                "{:<30} {:>5} → {:>4} lines  {:>6} → {:>5} tokens(est)  saved {:>5.1}%  messages kept {}/{}",
                name, r.in_lines, r.out_lines, ledger::tokens(raw.len()), ledger::tokens(out.len()), pct(out.len(), raw.len()), kept, seen.len()
            );
        }
    }
    if md {
        println!(
            "| **Total** | | {} → {} | **{:.1}%** | {} / {} |",
            comma(ledger::tokens(total.0)),
            comma(ledger::tokens(total.1)),
            pct(total.1, total.0),
            total.3,
            total.2
        );
    } else {
        println!(
            "\nTOTAL {} → {} tokens(est)  saved {:.1}%  messages kept {}/{}",
            ledger::tokens(total.0),
            ledger::tokens(total.1),
            pct(total.1, total.0),
            total.3,
            total.2
        );
    }
    Ok(())
}

fn comma(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}
