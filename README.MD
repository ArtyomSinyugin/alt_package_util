# ALT PM CLI
Compares packages in p10 and sisyphus branches and send output in json struct to the file. 

## CLI commands
Usage: alt_pm [OPTIONS] --arch <ARCH>

Options:
*  -a, --arch <ARCH>                Show result for chosen architecture: `armh`, `i586`, `x86_64`, `ppc64le`, `aarch64`, `x86_64-i586` or `noarch`
*  -m, --main-branch <MAIN_BRANCH>  URL of the main branch to compare [default: https://rdb.altlinux.org/api/export/branch_binary_packages/sisyphus]
*  -s, --sub-branch <SUB_BRANCH>    URL of the sub branch to compare [default: https://rdb.altlinux.org/api/export/branch_binary_packages/p10]
*  -o, --output <OUTPUT>            Set the name of the output .json file [default: output]
*  -h, --help                       Print help
*  -V, --version                    Print version

Usage example: `alt_pm --arch i586` - this command will show you all unique packages for `i586` arch for p10 branch 

## Installation
The recommended way of ALT PM CLI installation is via `cargo` from rust tools. 

* First of all install Rust via `rustup` (instructions: https://doc.rust-lang.org/book/ch01-01-installation.html)
* Download and unpack this repository into your Linux OS
* input command: `cargo install --path /*absolute path to the repository folder*/`   