//! `squeeze-auditor`: opens the squeeze-Auditor control panel. Built as a
//! windowed program on Windows so no console window appears.
#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() {
    let open = !std::env::args().any(|a| a == "--no-open");
    let _ = squeeze::auditor::run(squeeze::auditor::Options {
        port: 0,
        open,
        // close the server a while after the window is gone
        idle_exit_secs: 45,
    });
}
