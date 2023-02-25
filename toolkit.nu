def add [
    dependency
    --path: string
] {
    if not (open Cargo.toml | get -i "dependencies" | get -i $dependency.name | is-empty) {
        cargo remove $dependency.name
    }
    if ($dependency.features | is-empty) {
        cargo add $dependency.name --path ($path | path join $dependency.name)
    } else {
        cargo add $dependency.name --path ($path | path join $dependency.name) --features $dependency.features
    }
}

# add the dependencies of `nu_plugin_len` to the `Cargo.toml` file
#
# # Example:
# as i store all my git-related projects under `$env.GIT_REPOS_HOME`, inside a directory
# with format "{host}/{owner}/{repository}":
#
# ```bash
# > use toolkit.nu
# > toolkit setup (
#     $env.GIT_REPOS_HOME | path join "github.com" "nushell" "nushell"
# )
# ```
#
# might result in a diff in the repo similar to
#
# ```diff
# diff --git a/Cargo.toml b/Cargo.toml
# index 58213a0..a663b42 100644
# --- a/Cargo.toml
# +++ b/Cargo.toml
# @@ -6,5 +6,5 @@ edition = "2021"
#  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
#
#  [dependencies]
# -nu-plugin = { version = "0.74.1", path = "../../nushell/nushell/crates/nu-plugin" }
# -nu-protocol = { version = "0.74.1", path = "../../nushell/nushell/crates/nu-protocol", features = ["plugin"] }
# +nu-plugin = { version = "<version>", path = "/path/to/nushell/crates/nu-plugin" }
# +nu-protocol = { version = "<version>", path = "/path/to/nushell/crates/nu-protocol", features = ["plugin"] }
# ```
export def setup [
    nushell_path: string  # the path to the root of the nushell source
] {
    let crates_path = ($nushell_path | path join "crates")

    let dependencies = [
        [name features];
        [nu-plugin []]
        [nu-protocol [plugin]]
    ]

    for dependency in $dependencies {
        add $dependency --path $crates_path
    }
}

export def doc [] {
    open src/main.rs
    | lines
    | parse "//!{line}"
    | get line
    | str replace " " ""
    | str join "\n"
    | save --force docs/README.md
}
