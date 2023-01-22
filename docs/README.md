A simple plugin that ships the `len` command to `nushell` to compute the length of
strings.

This `nu_plugin_len` is greatly inspired by
- the [`nu_plugin_example`] official example plugin from the `nushell` source itself
- the [*Create a plugin (in `rust`)*] section of the old [contributors book] of `nushell`

> **Note**
> this plugin uses the `MsgPackSerializer` of `nu-plugin`

# Examples
### some valid input values
```bash
> "this is an example string" | len
25
> open -r Cargo.toml | len
377
```

### some invalid non-string values
```bash
> 1 | len
Error:
× Input is not a string: found int
╭─[entry #4:1:1]
1 │ 1 | len
· ┬
· ╰── Input is not a string: found int
╰────

> true | len
Error:
× Input is not a string: found bool
╭─[entry #5:1:1]
1 │ true | len
· ──┬─
·   ╰── Input is not a string: found bool
╰────

> 1.23 | len
Error:
× Input is not a string: found float
╭─[entry #7:1:1]
1 │ 1.23 | len
· ──┬─
·   ╰── Input is not a string: found float
╰────

> {a: "Happy", b: "new", c: "year"} | len
Error:
× Input is not a string: found record<a: string, b: string, c: string>
╭─[entry #8:1:1]
1 │ {a: "Happy", b: "new", c: "year"} | len
· ────────────────┬────────────────
·                 ╰── Input is not a string: found record<a: string, b: string, c: string>
╰────
```

[`nu_plugin_example`]: https://github.com/nushell/nushell/tree/b97bfe9297bed4c6063cdd27af0ac4ffe6c065ec/crates/nu_plugin_example
[*Create a plugin (in `rust`)*]: https://www.nushell.sh/contributor-book/plugins.html#creating-a-plugin-in-rust
[contributors book]: https://www.nushell.sh/contributor-book