# ALT PM CLI
Compares packages in p10 and sisyphus branches and gives output in json

## CLI commands
Usage: `alt_pm <COMMAND>`

Commands:
* `all-output`       Show all data output in json, including all packages unique for p10, all packages unique for sisyphus, all packages with version-release greater in sisyphus then in p10 branch
* `p10-unique`       Shows all packages unique for p10 in json
* `sisyphus-unique`  Shows all packages unique for sisyphus in json
* `v-check`          Shows all packages with version-release greater in sisyphus then in p10 branch in json
* `help`             Print this message or the help of the given subcommand(s)

Additional you can use subcommands with supported architechtures.

Usage: `alt_pm <COMMAND> <SUBCOMMAND>`

Suported architechtures:
* `armh`
* `i586`
* `noarch`
* `x86_64`
* `ppc64le`
* `aarch64`
* `x86_64-i586`

Usage example: `alt_pm p10-unique i586` - this command will show you all unique packages for `i586` arch for p10 branch 

## Installation
The recommended way of ALT PM CLI installation is via `cargo` from rust tools. 

* First of all install Rust via `rustup` (instructions: https://doc.rust-lang.org/book/ch01-01-installation.html)
* Download and unpack this repository into your Linux OS
* input command: `cargo install --path /*absolute path to the repository folder*/`   