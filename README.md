# nu_plugin_len
An example plugin for `nushell` to compute length of `String`s

## :package: change the path to the dependencies
> **Warning**  
> because all the dependencies for this plugin are not on *crates.io*, but rather
> only in the source of `nushell`, the path to all these dependencies,
> i.e. `nu-plugin` and `nu-protocol` are hardcoded to the `nushell` source on my machine.
>
> even though the paths do not mention `amtoine`, they still are unique to my system...

i've added a little script in [`scripts/`](scripts) called [`add-deps.nu`](scripts/add-deps.nu):
- have a look at `./scripts/add-deps.nu --help` for more information about the script
to correctly install these dependencies, which btw are not present at all in the `Cargo.toml`
to avoid any issues, one can (and probably should) run the following
```bash
> git clone https://github.com/nushell/nushell <path-to-nushell>
```
or choose another way to get the source code of `nushell` somewhere locally :yum:
then
```
> use toolkit.nu
> toolkit setup <path-to-nushell>
```

and *voila*
> **Note**  
> check `git diff Cargo.toml` to see the change :wink:

## :open_file_folder: install the plugin
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

## :gear: use the plugin
```bash
> "this is an example string" | len
25
```

## :scroll: documentation
one can have a look at the documentation of the crate by going to [`docs/`](docs)
> **Note**  
> this documentation has been generated with
> ```bash
> > use toolkit.nu
> > toolkit doc
> ```
