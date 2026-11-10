//! Test helpers: every test uses a private temp HOME for squeeze, Claude
//! Code and the desktop app config, never the real ones.

use std::path::{Path, PathBuf};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn base() -> PathBuf {
    std::env::temp_dir().join(format!("squeeze-test-{}", std::process::id()))
}

pub fn init() {
    INIT.call_once(|| {
        let b = base();
        let _ = std::fs::remove_dir_all(&b);
        std::fs::create_dir_all(b.join("claude")).unwrap();
        std::env::set_var("SQUEEZE_HOME", b.join("squeeze-home"));
        std::env::set_var("CLAUDE_CONFIG_DIR", b.join("claude"));
        std::env::set_var(
            "SQUEEZE_DESKTOP_CONFIG",
            b.join("desktop").join("claude_desktop_config.json"),
        );
    });
}

/// A fresh project dir with a `.squeeze` marker.
pub fn tmp_project(name: &str) -> PathBuf {
    init();
    let d = base().join("projects").join(name);
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join(".squeeze")).unwrap();
    std::fs::canonicalize(&d).unwrap()
}

/// A hook payload with session id, cwd and extra JSON fields.
pub fn payload(event: &str, session: &str, cwd: &Path, extra: &str) -> String {
    format!(
        r#"{{"hook_event_name":"{}","session_id":"{}","cwd":{},{}}}"#,
        event,
        session,
        crate::json::Value::str(cwd.to_string_lossy()).to_compact(),
        extra
    )
}
