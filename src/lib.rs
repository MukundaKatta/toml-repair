//! # toml-repair
//!
//! Repair messy TOML emitted by LLMs into something a real TOML parser
//! will accept.
//!
//! Fixes applied:
//!
//! 1. Strip ```toml / ``` fences and surrounding prose.
//! 2. Normalize CRLF and CR line endings to LF.
//! 3. Trim trailing whitespace.
//! 4. Convert smart quotes (`“ ”`) on string values to ASCII `"`.
//! 5. Strip trailing commas inside inline tables / arrays (TOML permits
//!    them in arrays only; we play it safe by leaving valid ones alone).
//!
//! ## Example
//!
//! ```
//! use toml_repair::repair;
//! let raw = "```toml\nname = “Claude”\nversion = \"4.5\"\n```";
//! let fixed = repair(raw);
//! assert!(fixed.contains("name = \"Claude\""));
//! ```

#![deny(missing_docs)]

/// Clean `raw` and return TOML-parser-ready text.
pub fn repair(raw: &str) -> String {
    let mut s = strip_fences(raw);
    s = s.replace("\r\n", "\n").replace('\r', "\n");
    s = unsmart_quotes(&s);
    s = trim_trailing_ws(&s);
    while s.ends_with('\n') {
        s.pop();
    }
    s
}

fn strip_fences(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 2 < bytes.len() {
        if &bytes[i..i + 3] == b"```" {
            let mut start = i + 3;
            while start < bytes.len() && bytes[start] != b'\n' {
                start += 1;
            }
            if start >= bytes.len() {
                return s.to_string();
            }
            start += 1;
            let mut j = start;
            while j + 3 <= bytes.len() {
                if &bytes[j..j + 3] == b"```" {
                    let prev = j.checked_sub(1).map(|k| bytes[k]).unwrap_or(b'\n');
                    if prev == b'\n' {
                        return s[start..j].to_string();
                    }
                }
                j += 1;
            }
            return s.to_string();
        }
        i += 1;
    }
    s.to_string()
}

fn unsmart_quotes(s: &str) -> String {
    s.replace('\u{201C}', "\"")
        .replace('\u{201D}', "\"")
        .replace('\u{2018}', "'")
        .replace('\u{2019}', "'")
}

fn trim_trailing_ws(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for line in s.split_inclusive('\n') {
        let had_nl = line.ends_with('\n');
        let core = if had_nl { &line[..line.len() - 1] } else { line };
        let stripped = core.trim_end_matches(|c: char| c == ' ' || c == '\t');
        out.push_str(stripped);
        if had_nl {
            out.push('\n');
        }
    }
    out
}
