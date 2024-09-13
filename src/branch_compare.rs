use crate::{Arch, CompareResult, BranchData};
use rpm::Evr;

pub fn compare(main_branch: BranchData, mut sub_branch: BranchData, arch: Arch) -> CompareResult {
    let mut compare_result: CompareResult = CompareResult { 
        unique_for_main_branch: Vec::new(), 
        unique_for_sub_branch: Vec::new(), 
        main_branch_has_greater_version: Vec::new() };
        
    let sub_branch_contains_arch: bool = sub_branch.contains_key(&arch);

    if let Some(main_branch_arch_packages) = main_branch.get(&arch) {
        if sub_branch_contains_arch {
            let sub_branch_arch_packages = sub_branch.get_mut(&arch).expect("Could not get value in p10 by sisyphus arch");
            for (package_name, package_info) in main_branch_arch_packages {
                if sub_branch_arch_packages.contains_key(package_name) {
                    let sub_branch_value_to_compare = sub_branch_arch_packages.get(package_name).expect("Package unwrap compare error");
                    if Evr::new(&package_info.epoch.to_string(), &package_info.version, &package_info.release) > Evr::new(&sub_branch_value_to_compare.epoch.to_string(), &sub_branch_value_to_compare.version, &sub_branch_value_to_compare.release) {
                        compare_result.main_branch_has_greater_version.push(package_info.clone());
                    }
                    sub_branch_arch_packages.remove(package_name);
                } else {
                    compare_result.unique_for_main_branch.push(package_info.clone());
                }
            }
        } else {
            // Arch is only in main branch. So, all packages are also only in main
            for (_, package_info) in main_branch_arch_packages {
                compare_result.unique_for_main_branch.push(package_info.clone());
            }
        }
    }

    if let Some(sub_branch_arch_packages) = sub_branch.get(&arch) {
        for (_, package_info) in sub_branch_arch_packages {
            compare_result.unique_for_sub_branch.push(package_info.clone());
        }
    }

    compare_result
}