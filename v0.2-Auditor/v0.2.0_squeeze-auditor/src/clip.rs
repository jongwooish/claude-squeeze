//! `squeeze clip`: compress the clipboard in place, so a long log can be
//! pasted into claude.ai (or any chat) at a fraction of its size.

use crate::config::{self, Config};
use crate::runner;
use std::io::Write;
use std::process::{Command, Stdio};

fn has(prog: &str) -> bool {
    Command::new(prog)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

/// (read command, write command) for this platform.
fn tools() -> Result<(Vec<String>, Vec<String>), String> {
    let v = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    if cfg!(target_os = "macos") {
        return Ok((v(&["pbpaste"]), v(&["pbcopy"])));
    }
    if cfg!(windows) {
        return Ok((
            v(&["powershell", "-NoProfile", "-Command", "[Console]::OutputEncoding=[Text.Encoding]::UTF8; Get-Clipboard -Raw"]),
            v(&["powershell", "-NoProfile", "-Command", "[Console]::InputEncoding=[Text.Encoding]::UTF8; Set-Clipboard -Value ([Console]::In.ReadToEnd())"]),
        ));
    }
    if std::env::var_os("WAYLAND_DISPLAY").is_some() && has("wl-paste") {
        return Ok((v(&["wl-paste", "--no-newline"]), v(&["wl-copy"])));
    }
    if has("xclip") {
        return Ok((
            v(&["xclip", "-selection", "clipboard", "-o"]),
            v(&["xclip", "-selection", "clipboard", "-i"]),
        ));
    }
    if has("xsel") {
        return Ok((
            v(&["xsel", "--clipboard", "--output"]),
            v(&["xsel", "--clipboard", "--input"]),
        ));
    }
    Err("no clipboard tool found (install wl-clipboard, xclip or xsel)".into())
}

fn read_clipboard(cmd: &[String]) -> Result<String, String> {
    let o = Command::new(&cmd[0])
        .args(&cmd[1..])
        .output()
        .map_err(|e| e.to_string())?;
    if !o.status.success() {
        return Err(String::from_utf8_lossy(&o.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&o.stdout).into_owned())
}

fn write_clipboard(cmd: &[String], text: &str) -> Result<(), String> {
    let mut c = Command::new(&cmd[0])
        .args(&cmd[1..])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    c.stdin
        .take()
        .unwrap()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())?;
    let st = c.wait().map_err(|e| e.to_string())?;
    if st.success() {
        Ok(())
    } else {
        Err("clipboard write failed".into())
    }
}

/// Compress the clipboard. With `print`, show the result instead of
/// replacing the clipboard.
pub fn clip(print: bool, cfg: &Config) -> Result<(), String> {
    let (rd, wr) = tools()?;
    let raw = read_clipboard(&rd)?;
    if raw.trim().is_empty() {
        return Err("clipboard is empty".into());
    }
    let state = config::global_dir();
    let mut out = Vec::new();
    let cwd = std::env::current_dir().unwrap_or_else(|_| ".".into());
    runner::emit_to(
        &raw,
        "(clipboard)",
        &cwd,
        &state,
        None,
        "clip",
        None,
        cfg,
        &mut out,
    );
    let text = String::from_utf8_lossy(&out).into_owned();
    if text == raw {
        eprintln!(
            "squeeze: clipboard is already short ({} lines), left unchanged",
            raw.lines().count()
        );
        return Ok(());
    }
    if print {
        print!("{}", text);
        return Ok(());
    }
    write_clipboard(&wr, &text)?;
    let last = text.lines().last().unwrap_or("");
    eprintln!("{}\nClipboard replaced. Paste it into the chat.", last);
    Ok(())
}
