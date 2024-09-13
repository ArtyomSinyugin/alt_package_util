use alt_pm::{branch_compare::compare, data_structurize::structurize, process_lib_types::arch_from_str, fetch_from_url::fetch_packages, BranchData};
use clap::Parser;
use std::{fs::File, io::Write};

// send output to file with -o flag(e.g. output.json as default or other)
// simplify json structure 
// DONE. do comparisment only for chosen via command line parameter architechture
// DONE. make possible to compare any two branches
// DONE. package compare via evr done. look at how can I do it via rpm crate

#[derive(Parser, Debug)]
#[command(name = "Package Comparer")]
#[command(version, about = "Compares packages in p10 and sisyphus branches", long_about = None)]
struct Cli {
    /// Required parameter! Shows the result for chosen architecture: armh, i586, x86_64, ppc64le, aarch64, x86_64-i586 or noarch
    #[arg(short, long)]
    arch: String,
    /// URL of the main branch to compare
    #[arg(short, long, default_value = "https://rdb.altlinux.org/api/export/branch_binary_packages/sisyphus")]
    main_branch: String,
    /// URL of the sub branch to compare
    #[arg(short, long, default_value = "https://rdb.altlinux.org/api/export/branch_binary_packages/p10")]
    sub_branch: String,
    /// Set the name of the output .json file
    #[arg(short, long, default_value = "output")]
    output: String, 
}

fn main () {
    let cli = Cli::parse();

    let url_sis_packages = match fetch_packages(&cli.main_branch) {
        Ok(data) => data,
        Err(_) =>{ 
            println!("No data from {}!", &cli.main_branch);
            Vec::new()
        },
    };
    let main_branch_packages: BranchData = structurize(url_sis_packages);

    let url_p10_packages = match fetch_packages(&cli.sub_branch) {
        Ok(data) => data,
        Err(_) =>{ 
            println!("No data from {}!", &cli.main_branch);
            Vec::new()
        },
    };
    let sub_branch_packages: BranchData = structurize(url_p10_packages);

    let input_arch = arch_from_str(&cli.arch).expect("Error: unknown architecture!");

    let compare_result = compare(main_branch_packages, sub_branch_packages, input_arch);

    let json = serde_json::to_string_pretty(&compare_result).expect("Could not recieve json from comparison result");

    let mut output_file_name = cli.output;
    output_file_name.push_str(".json");
    let mut file = File::create(output_file_name).expect("Could not create output file");
    file.write_all(json.as_bytes()).expect("Error writing json data to output file");

}