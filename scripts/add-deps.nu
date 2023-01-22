#!/usr/bin/env nu

# add the dependencies of `nu_plugin_len` to the `Cargo.toml` file
#
# Example:
# as i store all my git-related projects under `$env.GIT_REPOS_HOME`, inside a directory
# with format "{host}/{owner}/{repository}":
# ```bash
# ./scripts/add-deps.nu (
#     $env.GIT_REPOS_HOME | path join "github.com" "nushell" "nushell"
# )
# ```
# might result in a diff in the repo similar to 
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
def main [
    nushell_path: string  # the path to the root of the nushell source
] {
    let crates_path = ($nushell_path | path join "crates")
    cargo add nu-plugin --path ($crates_path | path join "nu-plugin")
    cargo add nu-protocol --path ($crates_path | path join "nu-protocol") --features plugin
}

