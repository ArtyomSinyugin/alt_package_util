use alt_pm::{branch_compare::compare, data_structurize::{arch_from_str, structurize}, fetch_from_url::fetch_packages, Sisyphus, P10};
use clap::{Parser, Subcommand};

// send output to file with -o flag(e.g. output.json as default or other)
// simplify json structure 
// do comparisment only for chosen via command line parameter architechture
// DONE. make possible to compare any two branches
// DONE. package compare via evr done. look at how can I do it via rpm crate

#[derive(Parser, Debug)]
#[command(name = "Package Comparer")]
#[command(version, about = "Compares packages in p10 and sisyphus branches", long_about = None)]
struct Cli {
    /// URL of the main branch to compare
    #[arg(short, long, default_value = "https://rdb.altlinux.org/api/export/branch_binary_packages/sisyphus")]
    main_branch: String,
    /// URL of the sub branch to compare
    #[arg(short, long, default_value = "https://rdb.altlinux.org/api/export/branch_binary_packages/p10")]
    sub_branch: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show all data output, including
    /// all packages unique for p10,
    /// all packages unique for sisyphus,
    /// all packages with version-release greater in sisyphus then in p10 branch
    AllOutput {
        /// Optional: filter by architecture
        arch: Option<String>,
    },
    /// Shows all packages unique for p10
    P10Unique {
        /// Optional: filter by architecture
        arch: Option<String>,
    },
    /// Shows all packages unique for sisyphus
    SisyphusUnique {
        /// Optional: filter by architecture
        arch: Option<String>,
    },
    /// Shows all packages with version-release greater in sisyphus then in p10 branch
    VCheck {
        /// Optional: filter by architecture
        arch: Option<String>,
    },
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
    let sis_packages: Sisyphus = structurize(url_sis_packages);

    let url_p10_packages = match fetch_packages(&cli.sub_branch) {
        Ok(data) => data,
        Err(_) =>{ 
            println!("No data from {}!", &cli.main_branch);
            Vec::new()
        },
    };
    let p10_packages: P10 = structurize(url_p10_packages);

    let compare_result = compare(sis_packages, p10_packages);



    match &cli.command {
        Commands::AllOutput { arch } => {
            let json: String;
            if let Some(arch) = arch {
                let input_arch = arch_from_str(arch).expect("Error: unknown architecture!");
                let compare_result = compare_result
                    .iter()
                    .find(|compare_result| compare_result.arch == input_arch)
                    .expect("Could not find comparison result for choosen arch");
                json = serde_json::to_string_pretty(compare_result).expect("Could not recieve json from comparison result");
            } else {
                json = serde_json::to_string_pretty(&compare_result).expect("Could not recieve json from comparison result");
            }
            println!("{json}");
        },
        Commands::P10Unique { arch } => {
            let json: String;
            if let Some(arch) = arch {
                let input_arch = arch_from_str(arch).expect("Error: unknown architecture!");
                let compare_result = compare_result
                    .iter()
                    .find(|compare_result| compare_result.arch == input_arch)
                    .expect("Could not find comparison result for choosen arch");
                json = serde_json::to_string_pretty(&compare_result.unique_for_p10).expect("Could not recieve unique p10 json from comparison result");
            } else {
                let mut json_temp = String::new();
                for result in compare_result {
                    json_temp.push_str(&serde_json::to_string_pretty(&result.unique_for_p10).expect("Could not recieve unique p10 json from comparison result"));
                }
                json = json_temp;
            }
            println!("{json}");
        },
        Commands::SisyphusUnique { arch } => {
            let json: String;
            if let Some(arch) = arch {
                let input_arch = arch_from_str(arch).expect("Error: unknown architecture!");
                let compare_result = compare_result
                    .iter()
                    .find(|compare_result| compare_result.arch == input_arch)
                    .expect("Could not find comparison result for choosen arch");
                json = serde_json::to_string_pretty(&compare_result.unique_for_sisyphus).expect("Could not recieve unique sisyphus json from comparison result");
            } else {
                let mut json_temp = String::new();
                for result in compare_result {
                    json_temp.push_str(&serde_json::to_string_pretty(&result.unique_for_sisyphus).expect("Could not recieve unique sisyphus json from comparison result"));
                }
                json = json_temp;
            }
            println!("{json}");
        },
        Commands::VCheck { arch } => {
            let json: String;
            if let Some(arch) = arch {
                let input_arch = arch_from_str(arch).expect("Error: unknown architecture!");
                let compare_result = compare_result
                    .iter()
                    .find(|compare_result| compare_result.arch == input_arch)
                    .expect("Could not find comparison result for choosen arch");
                json = serde_json::to_string_pretty(&compare_result.sisyphus_has_greater_version).expect("Could not recieve greater version packages for sisyphus in json from comparison result");
            } else {
                let mut json_temp = String::new();
                for result in compare_result {
                    json_temp.push_str(&serde_json::to_string_pretty(&result.sisyphus_has_greater_version).expect("Could not recieve greater version packages for sisyphus in json from comparison result"));
                }
                json = json_temp;
            }
            println!("{json}");
        },
    }
}