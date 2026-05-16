# toml-repair

[![crates.io](https://img.shields.io/crates/v/toml-repair.svg)](https://crates.io/crates/toml-repair)

Repair messy TOML emitted by LLMs into something a real TOML parser
will accept.

```rust
use toml_repair::repair;
let raw = "```toml\nname = \u{201C}Claude\u{201D}\n```";
let fixed = repair(raw);
assert!(fixed.contains("name = \"Claude\""));
```

Strips ``` fences, normalizes CRLF, converts smart quotes, trims
trailing whitespace. Zero deps. MIT or Apache-2.0.
