//! Runs a shell command, captures its combined output, compresses it when
//! it is long, and keeps the full log on disk.

use crate::compress::{compress, footer};
use crate::config::{self, Config};
use crate::ledger::{self, Event};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Run `command` in `dir`, print the (possibly compressed) output, and
/// return the command's exit code.
pub fn run(command: &str, dir: &Path, cfg: &Config) -> i32 {
    let (raw, exit) = capture(command, dir);
    emit(
        &raw,
        command,
        dir,
        "run",
        Some(exit),
        cfg,
        &mut std::io::stdout(),
    )
}

/// The shell used to run commands.
#[derive(Debug, Clone, PartialEq)]
pub enum Shell {
    /// A POSIX shell (bash or sh) at this path.
    Posix(PathBuf),
    /// Windows PowerShell, when no POSIX shell exists.
    PowerShell,
}

/// Pick the shell. Order: `SQUEEZE_SHELL`, then bash (on Windows: Git Bash,
/// never the `System32\bash.exe` WSL launcher), then sh, then PowerShell.
pub fn find_shell() -> Shell {
    if let Some(s) = std::env::var_os("SQUEEZE_SHELL").filter(|s| !s.is_empty()) {
        return Shell::Posix(PathBuf::from(s));
    }
    if cfg!(windows) {
        // Claude Code on Windows records its Git Bash here when it is not on PATH.
        if let Some(p) = std::env::var_os("CLAUDE_CODE_GIT_BASH_PATH").map(PathBuf::from) {
            if p.is_file() {
                return Shell::Posix(p);
            }
        }
        if let Some(p) = which("bash.exe").filter(|p| !is_wsl_launcher(p)) {
            return Shell::Posix(p);
        }
        for base in ["ProgramFiles", "ProgramW6432", "LOCALAPPDATA"] {
            if let Some(b) = std::env::var_os(base) {
                for sub in ["Git\\bin\\bash.exe", "Programs\\Git\\bin\\bash.exe"] {
                    let p = PathBuf::from(&b).join(sub);
                    if p.is_file() {
                        return Shell::Posix(p);
                    }
                }
            }
        }
        return Shell::PowerShell;
    }
    which("bash")
        .or_else(|| which("sh"))
        .map(Shell::Posix)
        .unwrap_or_else(|| Shell::Posix(PathBuf::from("/bin/sh")))
}

/// `C:\Windows\System32\bash.exe` and the WindowsApps stub start WSL,
/// not a shell for this machine.
fn is_wsl_launcher(p: &Path) -> bool {
    let s = p.to_string_lossy().to_lowercase().replace('/', "\\");
    s.contains("\\windows\\system32\\") || s.contains("\\windowsapps\\")
}

/// Execute through the shell with stderr merged into stdout in order.
pub fn capture(command: &str, dir: &Path) -> (String, i32) {
    let mut cmd = match find_shell() {
        Shell::Posix(sh) => {
            let mut c = Command::new(sh);
            // the group redirect keeps stdout and stderr in arrival order
            c.arg("-c").arg(format!("{{ {}\n}} 2>&1", command));
            c
        }
        Shell::PowerShell => {
            let mut c = Command::new("powershell");
            c.args([
                "-NoProfile",
                "-Command",
                &format!("& {{ {} }} 2>&1", command),
            ]);
            c
        }
    };
    cmd.current_dir(dir)
        .stdin(Stdio::inherit())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    match cmd.output() {
        Ok(o) => {
            let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
            s.push_str(&String::from_utf8_lossy(&o.stderr));
            (s, o.status.code().unwrap_or(1))
        }
        Err(e) => (format!("squeeze: could not start shell: {}\n", e), 127),
    }
}

/// First match for `prog` on PATH, skipping WSL launchers.
fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(prog))
        .find(|p| p.is_file() && !is_wsl_launcher(p))
}

/// Compress `raw`, save the full log when compressed, write the result to
/// `w`, record a ledger event, and return `exit` (0 when None).
pub fn emit(
    raw: &str,
    subject: &str,
    dir: &Path,
    kind: &str,
    exit: Option<i32>,
    cfg: &Config,
    w: &mut dyn Write,
) -> i32 {
    let root = config::root(dir);
    let state = root.join(config::DIR);
    emit_to(raw, subject, dir, &state, Some(&root), kind, exit, cfg, w)
}

#[allow(clippy::too_many_arguments)]
pub fn emit_to(
    raw: &str,
    subject: &str,
    dir: &Path,
    state: &Path,
    project: Option<&Path>,
    kind: &str,
    exit: Option<i32>,
    cfg: &Config,
    w: &mut dyn Write,
) -> i32 {
    let code = exit.unwrap_or(0);
    let res = compress(raw, &cfg.compress);
    if !res.compressed {
        let _ = w.write_all(raw.as_bytes());
        return code;
    }
    let raw_path = save_raw(state, raw, cfg.keep_raw)
        .ok()
        .map(|p| display_path(dir, &p));
    let foot = footer(&res, raw_path.as_deref(), exit);
    let _ = w.write_all(res.text.as_bytes());
    let _ = writeln!(w, "{}", foot);
    let _ = ledger::record(
        project,
        Event {
            kind: kind.into(),
            subject: subject.into(),
            in_bytes: res.in_bytes,
            out_bytes: res.out_bytes + foot.len() + 1,
            in_lines: res.in_lines,
            out_lines: res.out_lines,
            exit,
            ..Default::default()
        },
    );
    code
}

fn display_path(dir: &Path, p: &Path) -> String {
    let dir = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let p = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    if let Ok(r) = p.strip_prefix(&dir) {
        return r.to_string_lossy().replace('\\', "/");
    }
    // outside the project (squeeze clip): show ~/... instead of the home path
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        let home = std::fs::canonicalize(&home).unwrap_or_else(|_| home.into());
        if let Ok(r) = p.strip_prefix(&home) {
            return format!("~/{}", r.to_string_lossy().replace('\\', "/"));
        }
    }
    p.to_string_lossy().replace('\\', "/")
}

/// UTC "YYYYMMDD-HHMMSS" for a unix timestamp.
pub fn stamp(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    // civil-from-days (Howard Hinnant)
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        y,
        m,
        d,
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// Write `raw` to `<state>/raw/<stamp>-<id>.log` and prune old logs.
pub fn save_raw(state: &Path, raw: &str, keep: usize) -> std::io::Result<PathBuf> {
    let dir = state.join("raw");
    std::fs::create_dir_all(&dir)?;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let id = (nanos ^ (std::process::id() << 8)) & 0xff_ffff;
    let p = dir.join(format!("{}-{:06x}.log", stamp(ledger::now()), id));
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&p)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = f.set_permissions(std::fs::Permissions::from_mode(0o600));
    }
    f.write_all(raw.as_bytes())?;
    prune(&dir, keep);
    Ok(p)
}

fn prune(dir: &Path, keep: usize) {
    if keep == 0 {
        return;
    }
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    let mut logs: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "log"))
        .collect();
    logs.sort(); // names start with a timestamp
    while logs.len() > keep {
        let _ = std::fs::remove_file(logs.remove(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("squeeze-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join(config::DIR)).unwrap();
        d
    }

    #[test]
    fn skips_wsl_launcher() {
        assert!(is_wsl_launcher(Path::new(r"C:\Windows\System32\bash.exe")));
        assert!(is_wsl_launcher(Path::new(
            r"C:\Users\me\AppData\Local\Microsoft\WindowsApps\bash.exe"
        )));
        assert!(!is_wsl_launcher(Path::new(
            r"C:\Program Files\Git\bin\bash.exe"
        )));
        assert!(!is_wsl_launcher(Path::new("/usr/bin/bash")));
    }

    #[test]
    fn stamp_known_dates() {
        assert_eq!(stamp(0), "19700101-000000");
        assert_eq!(stamp(1_793_680_200), "20261103-043000");
    }

    #[test]
    fn run_keeps_exit_code_and_saves_raw() {
        let d = tmp("run");
        let cmd = r#"for i in $(seq 1 400); do echo "step $i"; done; echo "error: disk full" >&2; exit 3"#;
        let (raw, exit) = capture(cmd, &d);
        assert_eq!(exit, 3);
        let mut out = Vec::new();
        emit(
            &raw,
            cmd,
            &d,
            "run",
            Some(exit),
            &Config::default(),
            &mut out,
        );
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains("error: disk full"), "{}", s);
        assert!(s.contains("Full output: .squeeze/raw/"), "{}", s);
        assert!(s.contains("(exit 3)"));
        assert_eq!(
            std::fs::read_dir(d.join(".squeeze/raw")).unwrap().count(),
            1
        );
        assert_eq!(ledger::read_project(&config::root(&d)).len(), 1);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn short_output_untouched() {
        let d = tmp("short");
        let (raw, exit) = capture("echo hello", &d);
        let mut out = Vec::new();
        emit(
            &raw,
            "echo hello",
            &d,
            "run",
            Some(exit),
            &Config::default(),
            &mut out,
        );
        assert_eq!(String::from_utf8(out).unwrap(), "hello\n");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn prune_keeps_newest() {
        let d = tmp("prune");
        for n in [
            "20260101-000000-a.log",
            "20260102-000000-b.log",
            "20260103-000000-c.log",
        ] {
            std::fs::write(d.join(n), "").unwrap();
        }
        prune(&d, 2);
        assert!(!d.join("20260101-000000-a.log").exists());
        assert!(d.join("20260103-000000-c.log").exists());
        let _ = std::fs::remove_dir_all(&d);
    }
}
