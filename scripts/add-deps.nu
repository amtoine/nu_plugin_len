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
def main [
    nushell_path: string  # the path to the root of the nushell source
] {
    let crates_path = ($nushell_path | path join "crates")
    cargo add nu-plugin --path ($crates_path | path join "nu-plugin")
    cargo add nu-protocol --path ($crates_path | path join "nu-protocol") --features plugin
}

