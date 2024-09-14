use alt_pm::{BRANCHES_URL, branch_compare::compare, data_structurize::structurize, process_lib_types::arch_from_str, fetch_from_url::fetch_packages, BranchData};
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
    /// Input main branch name to compare
    #[arg(short, long, default_value = "sisyphus")]
    main_branch: String,
    /// Input sub branch name to compare
    #[arg(short, long, default_value = "p10")]
    sub_branch: String,
    /// Set the name of the output .json file
    #[arg(short, long, default_value = "output")]
    output: String, 
}

fn main () {
    let cli = Cli::parse();
    let main_branch_url = format!("{BRANCHES_URL}{}", &cli.main_branch.to_lowercase());
    let sub_branch_url = format!("{BRANCHES_URL}{}", &cli.sub_branch.to_lowercase());

    let url_main_branch_packages = match fetch_packages(&main_branch_url) {
        Ok(data) => data,
        Err(_) =>{ 
            println!("No data from {}! Check the correctness of branch name", &cli.main_branch);
            Vec::new()
        },
    };
    let url_sub_branch_packages = match fetch_packages(&sub_branch_url) {
        Ok(data) => data,
        Err(_) =>{ 
            println!("No data from {}! Check the correctness of branch name", &cli.sub_branch);
            Vec::new()
        },
    };

    let main_branch_packages: BranchData = structurize(url_main_branch_packages);
    let sub_branch_packages: BranchData = structurize(url_sub_branch_packages);

    let input_arch = arch_from_str(&cli.arch).expect("Error: unknown architecture!");

    let compare_result = compare(main_branch_packages, sub_branch_packages, input_arch);

    let json = serde_json::to_string_pretty(&compare_result).expect("Could not recieve json from comparison result");

    let mut output_file_name = cli.output;
    output_file_name.push_str(".json");
    let mut file = File::create(output_file_name).expect("Could not create output file");
    file.write_all(json.as_bytes()).expect("Error writing json data to output file");

}