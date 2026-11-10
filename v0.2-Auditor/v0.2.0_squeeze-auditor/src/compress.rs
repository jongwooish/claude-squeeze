//! Shrinks long command output so only what a model needs reaches the
//! context: errors, warnings, the first lines and the last lines.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    /// Output at or below this many lines passes through untouched.
    pub max_lines: usize,
    /// Output at or below this many bytes passes through untouched.
    pub max_bytes: usize,
    pub head_lines: usize,
    pub tail_lines: usize,
    /// Lines of context kept around an error or warning.
    pub context: usize,
    /// Rough upper bound on kept lines.
    pub budget: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            max_lines: 80,
            max_bytes: 8000,
            head_lines: 8,
            tail_lines: 20,
            context: 2,
            budget: 120,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Outcome {
    pub text: String,
    pub compressed: bool,
    pub in_lines: usize,
    pub out_lines: usize,
    pub in_bytes: usize,
    pub out_bytes: usize,
}

/// Strip ANSI escapes and collapse carriage-return progress bars, keeping
/// the final state of each rewritten line.
pub fn clean(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    // CSI: parameters/intermediates, then a final byte in @..~
                    for d in chars.by_ref() {
                        if ('@'..='~').contains(&d) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    // OSC: until BEL or ESC \
                    chars.next();
                    while let Some(d) = chars.next() {
                        if d == '\x07' {
                            break;
                        }
                        if d == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {}
            }
            continue;
        }
        out.push(c);
    }
    let out = out.replace("\r\n", "\n");
    if !out.contains('\r') {
        return out;
    }
    out.split('\n')
        .map(|l| {
            let t = l.trim_end_matches('\r');
            match t.rfind('\r') {
                Some(j) => &t[j + 1..],
                None => t,
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// True when `word` occurs in `hay` starting at a word boundary.
fn starts_word(hay: &str, word: &str) -> bool {
    let b = hay.as_bytes();
    let mut from = 0;
    while let Some(p) = hay[from..].find(word) {
        let at = from + p;
        if at == 0 || !b[at - 1].is_ascii_alphanumeric() {
            return true;
        }
        from = at + word.len();
    }
    false
}

/// True when `word` occurs as a whole word.
fn whole_word(hay: &str, word: &str) -> bool {
    let b = hay.as_bytes();
    let mut from = 0;
    while let Some(p) = hay[from..].find(word) {
        let at = from + p;
        let end = at + word.len();
        let left = at == 0 || !b[at - 1].is_ascii_alphanumeric();
        let right = end >= b.len() || !b[end].is_ascii_alphanumeric();
        if left && right {
            return true;
        }
        from = end;
    }
    false
}

const SEVERE_WORDS: &[&str] = &["error", "fail", "fatal", "panic", "killed", "abort"];
const SEVERE_ANY: &[&str] = &[
    "err!",
    "exception",
    "traceback",
    "segmentation fault",
    "out of memory",
    "timed out",
    "timeout",
    "core dumped",
];
const WARN_WORDS: &[&str] = &["warn", "deprecat"];
const OTHER_ANY: &[&str] = &[
    "denied",
    "not found",
    "no such file",
    "cannot",
    "can't",
    "unable to",
    "refused",
    "assert",
];
const BENIGN: &[&str] = &[
    "0 errors",
    "0 error(s)",
    "0 failures",
    "0 failed",
    "0 warnings",
    "no errors",
    "no warnings",
    "no failures",
    "errors: 0",
    "failed: 0",
    "warnings: 0",
    "failures: 0",
    "error_log",
    "-werror",
];

/// Is this line an error rather than a warning?
pub fn is_severe(line: &str) -> bool {
    let l = line.to_lowercase();
    SEVERE_WORDS.iter().any(|w| starts_word(&l, w))
        || SEVERE_ANY.iter().any(|w| l.contains(w))
        || whole_word(&l, "err")
}

/// Should this line survive compression?
pub fn is_important(line: &str) -> bool {
    let l = line.to_lowercase();
    if BENIGN.iter().any(|w| l.contains(w)) {
        return false;
    }
    is_severe(line)
        || WARN_WORDS.iter().any(|w| starts_word(&l, w))
        || OTHER_ANY.iter().any(|w| l.contains(w))
}

/// Normalized form used to group repeated messages: lowercase, digits
/// become 0, hex ids become #.
pub fn shape(line: &str) -> String {
    let l = line.trim().to_lowercase();
    let mut out = String::with_capacity(l.len());
    let chars: Vec<char> = l.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        // a run of 7+ hex chars that contains a letter and a digit -> '#'
        let mut j = i;
        while j < chars.len() && chars[j].is_ascii_hexdigit() {
            j += 1;
        }
        let run = &chars[i..j];
        let boundary = (i == 0 || !chars[i - 1].is_ascii_alphanumeric())
            && (j == chars.len() || !chars[j].is_ascii_alphanumeric());
        if run.len() >= 7
            && boundary
            && run.iter().any(|c| c.is_ascii_digit())
            && run.iter().any(|c| c.is_ascii_alphabetic())
        {
            out.push('#');
            i = j;
            continue;
        }
        let c = chars[i];
        if c.is_ascii_digit() {
            out.push('0');
            while i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
                i += 1;
            }
        } else {
            out.push(c);
        }
        i += 1;
    }
    out
}

fn is_continuation(line: &str) -> bool {
    line.starts_with(' ') || line.starts_with('\t') || line.trim_start().starts_with("File \"")
}

/// Compress raw output. Output within the pass-through limits is returned
/// unchanged.
pub fn compress(raw: &str, o: &Options) -> Outcome {
    let cleaned = clean(raw);
    let text = cleaned.trim_end_matches('\n');
    let lines: Vec<&str> = if text.is_empty() {
        Vec::new()
    } else {
        text.split('\n').collect()
    };
    let n = lines.len();
    let mut res = Outcome {
        in_lines: n,
        in_bytes: raw.len(),
        ..Default::default()
    };
    if n <= o.max_lines && raw.len() <= o.max_bytes {
        res.text = raw.to_string();
        res.out_lines = n;
        res.out_bytes = raw.len();
        return res;
    }
    res.compressed = true;

    let mut keep = vec![false; n];
    for k in keep.iter_mut().take(o.head_lines.min(n)) {
        *k = true;
    }
    for k in keep.iter_mut().skip(n.saturating_sub(o.tail_lines)) {
        *k = true;
    }

    // Group important lines by shape: a warning repeated 30 times costs one
    // line plus a count. Errors are chosen before warnings.
    struct Group {
        first: usize,
        count: usize,
        severe: bool,
    }
    let mut order: Vec<String> = Vec::new();
    let mut groups: HashMap<String, Group> = HashMap::new();
    for (i, l) in lines.iter().enumerate() {
        if !is_important(l) {
            continue;
        }
        let k = shape(l);
        let g = groups.entry(k.clone()).or_insert_with(|| {
            order.push(k);
            Group {
                first: i,
                count: 0,
                severe: is_severe(l),
            }
        });
        g.count += 1;
    }
    let per_hit = 1 + 2 * o.context;
    let max_hits = (o.budget / (per_hit + 2)).max(4);
    let mut chosen: Vec<&Group> = Vec::new();
    for severe in [true, false] {
        for k in &order {
            let g = &groups[k];
            if g.severe == severe && chosen.len() < max_hits {
                chosen.push(g);
            }
        }
    }
    let mut more: HashMap<usize, usize> = HashMap::new();
    for g in &chosen {
        if g.count > 1 {
            more.insert(g.first, g.count - 1);
        }
        let i = g.first;
        for k in keep
            .iter_mut()
            .take((i + o.context).min(n - 1) + 1)
            .skip(i.saturating_sub(o.context))
        {
            *k = true;
        }
        // stack traces and compiler notes are indented: keep a few
        let mut j = i + o.context + 1;
        let mut extra = 0;
        while j < n && extra < 12 && is_continuation(lines[j]) {
            keep[j] = true;
            j += 1;
            extra += 1;
        }
    }

    // Emit kept lines; collapse gaps and runs of same-shaped lines.
    let mut out = String::new();
    let mut kept = 0;
    let mut gap = 0;
    let mut i = 0;
    while i < n {
        if !keep[i] {
            gap += 1;
            i += 1;
            continue;
        }
        if gap > 0 {
            out.push_str(&format!("… [{} lines omitted] …\n", gap));
            gap = 0;
        }
        let s = shape(lines[i]);
        let mut j = i + 1;
        while j < n && keep[j] && !is_important(lines[j]) && shape(lines[j]) == s {
            j += 1;
        }
        out.push_str(lines[i]);
        out.push('\n');
        kept += 1;
        if let Some(m) = more.get(&i) {
            out.push_str(&format!("… [same message {} more times] …\n", m));
        }
        let reps = j - i - 1;
        if reps > 1 {
            out.push_str(&format!("… [{} similar lines] …\n", reps));
            i = j;
        } else {
            i += 1;
        }
    }
    if gap > 0 {
        out.push_str(&format!("… [{} lines omitted] …\n", gap));
    }
    res.out_lines = kept;
    res.out_bytes = out.len();
    res.text = out;
    res
}

pub fn human(n: usize) -> String {
    if n >= 1 << 20 {
        format!("{:.1} MB", n as f64 / (1u64 << 20) as f64)
    } else if n >= 1 << 10 {
        format!("{:.1} KB", n as f64 / 1024.0)
    } else {
        format!("{} B", n)
    }
}

/// The one-line summary printed after compressed output.
pub fn footer(r: &Outcome, raw_path: Option<&str>, exit: Option<i32>) -> String {
    let mut s = format!(
        "[squeeze] {} lines ({}) → {} lines ({})",
        r.in_lines,
        human(r.in_bytes),
        r.out_lines,
        human(r.out_bytes)
    );
    if let Some(p) = raw_path {
        s.push_str(". Full output: ");
        s.push_str(p);
    }
    if let Some(e) = exit {
        s.push_str(&format!(" (exit {})", e));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_output_passes_through() {
        let r = compress("ok\nfine\n", &Options::default());
        assert!(!r.compressed);
        assert_eq!(r.text, "ok\nfine\n");
    }

    #[test]
    fn keeps_errors_and_drops_noise() {
        let mut s = String::new();
        for i in 0..500 {
            s += &format!("Downloading package-{} ({}/500)\n", i, i);
        }
        s += "ERROR: could not resolve dependency left-pad@9\n";
        for i in 0..300 {
            s += &format!("  resolving {}\n", i);
        }
        let r = compress(&s, &Options::default());
        assert!(r.compressed);
        assert!(r
            .text
            .contains("ERROR: could not resolve dependency left-pad@9"));
        assert!(
            r.out_bytes * 5 < r.in_bytes,
            "{} -> {}",
            r.in_bytes,
            r.out_bytes
        );
    }

    #[test]
    fn repeated_warnings_collapse() {
        let mut s = String::new();
        for i in 0..200 {
            s += &format!(
                "src/f{}.c:{}:3: warning: unused variable 'v{}'\nfiller\n",
                i, i, i
            );
        }
        let r = compress(&s, &Options::default());
        assert!(r.text.contains("same message 199 more times"), "{}", r.text);
    }

    #[test]
    fn errors_chosen_before_warnings() {
        let mut s = String::new();
        for i in 0..400 {
            let a = (b'a' + (i % 26) as u8) as char;
            let b = (b'a' + (i / 26) as u8) as char;
            s += &format!("warning: deprecated option kind-{}{}\n", a, b);
        }
        s += "fatal: repository corrupted\n";
        s += &"noise\n".repeat(100);
        let r = compress(&s, &Options::default());
        assert!(r.text.contains("fatal: repository corrupted"));
    }

    #[test]
    fn clean_strips_ansi_and_progress() {
        assert_eq!(clean("\x1b[32mok\x1b[0m\n10%\r50%\r100%\n"), "ok\n100%\n");
    }

    #[test]
    fn important_and_benign() {
        for l in [
            "0 errors, 0 warnings",
            "Tests: 12 passed, failed: 0",
            "terrorism report",
            "no errors found",
        ] {
            assert!(!is_important(l), "{}", l);
        }
        for l in [
            "--- FAIL: TestX",
            "npm ERR! code E404",
            "Traceback (most recent call last):",
            "warning: x",
            "Permission denied",
        ] {
            assert!(is_important(l), "{}", l);
        }
    }

    #[test]
    fn shape_normalizes() {
        assert_eq!(
            shape("src/a12.c:3:9: warning"),
            shape("src/a7.c:44:1: warning")
        );
        assert_eq!(shape("commit 3f2a9c1b0e"), "commit #");
    }

    #[test]
    fn fixtures_keep_every_distinct_message() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/fixtures");
        let Ok(rd) = std::fs::read_dir(dir) else {
            return;
        };
        for e in rd.flatten() {
            let raw = std::fs::read_to_string(e.path()).unwrap();
            let r = compress(&raw, &Options::default());
            if !r.compressed {
                continue;
            }
            let mut seen: HashMap<String, bool> = HashMap::new();
            for l in clean(&raw).lines().filter(|l| is_important(l)) {
                let hit = r.text.contains(l);
                let v = seen.entry(shape(l)).or_insert(false);
                *v = *v || hit;
            }
            for (k, ok) in seen {
                assert!(ok, "{:?}: lost {:?}", e.path(), k);
            }
        }
    }
}
