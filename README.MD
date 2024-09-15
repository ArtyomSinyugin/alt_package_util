# ALT PM CLI
Compares packages in main and sub branches and sends output in json struct to the file. 

## CLI commands
Usage: `alt_pm [OPTIONS] --arch <ARCH>`

Options:
*  `-a, --arch <ARCH>                `Required parameter! Shows the result for chosen architecture: `armh`, `i586`, `x86_64`, `ppc64le`, `aarch64`, `x86_64-i586` or `noarch`
*  `-m, --main-branch <MAIN_BRANCH>  `Input main branch name to compare [default: `sisyphus`]
*  `-s, --sub-branch <SUB_BRANCH>    `Input sub branch name to compare [default: `p10`]
*  `-o, --output <OUTPUT>            `Set the name of the output .json file [default: `output`]
*  `-h, --help                       `Print help
*  `-V, --version                    `Print version

 Branches available for comparison with each other: `sisyphus`, `p10`, `p11`, `p9`

Usage examples: 
- `alt_pm --arch i586` - this command will compare default branches sisyphus and p10 for `i586` arch and will send output to `output.json` file 
- `alt_pm -a i586 -m <BRANCH> -s <BRANCH> -o i586` - this will compare branches and will send output to `i586.json` file 

## Installation
The recommended way of ALT PM CLI installation is via `cargo` from rust tools. 

* First of all install Rust via `rustup` (instructions: https://doc.rust-lang.org/book/ch01-01-installation.html)
* Download and unpack this repository into your Linux OS
* input command: `cargo install --path /*absolute path to the repository folder*/`   
