# nu_plugin_len
An example plugin for `nushell` to compute length of `String`s

## install the plugin
- build and install the binary with
```bash
cargo install --path .
```
- register the plugin in `nushell` with
```bash
register ~/.local/share/cargo/bin/nu_plugin_len
```

> **Note**  
> i use `~/.local/share/cargo/bin/nu_plugin_len` in the `register`
> above because my `CARGO_HOME` is set to `($env.XDG_DATA_HOME | path join "cargo")`,
> i.e. `~/.local/share/cargo`, in my `env.nu`

## use the plugin
```bash
> "this is an example string" | my_str_len
25
```
