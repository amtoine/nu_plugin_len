#!/usr/bin/env nu

open src/main.rs
| lines
| parse "//!{line}"
| get line
| str replace " " ""
| str join "\n"
| save --force docs/README.md
