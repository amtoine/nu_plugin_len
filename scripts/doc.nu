#!/usr/bin/env nu

open src/main.rs
| lines
| parse "//!{line}"
| get line
| str trim
| str join "\n"
| save --force docs/README.md
