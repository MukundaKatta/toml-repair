use toml_repair::repair;

#[test]
fn strips_toml_fence() {
    let raw = "```toml\nname = \"Claude\"\n```";
    assert_eq!(repair(raw), "name = \"Claude\"");
}

#[test]
fn unsmart_quotes() {
    let raw = "name = \u{201C}Claude\u{201D}";
    assert_eq!(repair(raw), "name = \"Claude\"");
}

#[test]
fn normalizes_crlf() {
    assert_eq!(repair("a = 1\r\nb = 2\r\n"), "a = 1\nb = 2");
}

#[test]
fn trims_trailing_ws() {
    assert_eq!(repair("a = 1   \n"), "a = 1");
}

#[test]
fn no_fence_passes_through() {
    assert_eq!(repair("a = 1\nb = 2"), "a = 1\nb = 2");
}

#[test]
fn combined_messy_input() {
    let raw = "Sure:\n```toml\nname = \u{201C}x\u{201D}   \nversion = \"1.0\"\n```\nDone!";
    let fixed = repair(raw);
    assert!(fixed.contains("name = \"x\""));
    assert!(fixed.contains("version = \"1.0\""));
    assert!(!fixed.contains("```"));
    assert!(!fixed.contains("\u{201C}"));
}
