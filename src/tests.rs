#[cfg(test)]
use crate::{Arch, ApiResponse, branch_compare::compare, data_structurize::structurize, BranchData};
#[cfg(test)]
use serde_json::from_str;

#[test]
fn compare_test() {
    let sis_packages: ApiResponse = from_str(SISYPHUS_DATA).expect("Could not deserialize from request to ApiResponse struct");
    let p10_packages: ApiResponse = from_str(P10_DATA).expect("Could not deserialize from request to ApiResponse struct");

    let sis_packages: BranchData = structurize(sis_packages.packages);
    let p10_packages: BranchData = structurize(p10_packages.packages);

    let arch_vec = vec![Arch::Aarch64,Arch::Armh,Arch::I586,Arch::NoArch,Arch::Ppc64le,Arch::X86_64,Arch::X86_64_i586];

    for arch in &arch_vec {
        let compare_result = compare(sis_packages.clone(), p10_packages.clone(), arch.clone());
        if arch == &Arch::Aarch64 {
            let test_result1 = compare_result.unique_for_sub_branch.iter().any(|package| {
                package.name == "86box_tsdtd".to_string()
            });
            let test_result2 = compare_result.unique_for_main_branch.iter().any(|package| {
                package.name == "389-ds-base-devel".to_string()
            });
            let test_result3 = compare_result.main_branch_has_greater_version.iter().any(|package| {
                package.name == "AFLplusplus".to_string()
            });
            let test_result4 = compare_result.main_branch_has_greater_version.iter().any(|package| {
                package.name == "9wm-debuginfo".to_string()
            });
            assert!(test_result1);
            assert!(test_result2);
            assert!(test_result3);
            assert!(test_result4);
            println!("{:?}", compare_result);
            break;
        }
    }
}

#[cfg(test)]
const SISYPHUS_DATA: &str = r#"
{"request_args": {"arch": null}, 
"length": 165681, 
"packages": 
    [{"name": "0ad", "epoch": 1, "version": "0.0.26", "release": "alt0_9_alpha", "arch": "aarch64", "disttag": "sisyphus+350867.100.1.1", "buildtime": 1718210955, "source": "0ad"}, 
    {"name": "AFLplusplus", "epoch": 0, "version": "4.21c", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+355171.400.4.1", "buildtime": 1723712683, "source": "AFLplusplus"}, 
    {"name": "AFLplusplus-debuginfo", "epoch": 0, "version": "4.21c", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+355171.400.4.1", "buildtime": 1723712683, "source": "AFLplusplus"},
    {"name": "0ad-debuginfo", "epoch": 1, "version": "0.0.26", "release": "alt0_9_alpha", "arch": "aarch64", "disttag": "sisyphus+350867.100.1.1", "buildtime": 1718210955, "source": "0ad"}, 
    {"name": "389-ds-base", "epoch": 0, "version": "2.4.6", "release": "alt1", "arch": "aarch64", "disttag": "sisyphus+355588.100.2.1", "buildtime": 1724238943, "source": "389-ds-base"}, 
    {"name": "389-ds-base-debuginfo", "epoch": 0, "version": "2.4.6", "release": "alt1", "arch": "aarch64", "disttag": "sisyphus+355588.100.2.1", "buildtime": 1724238943, "source": "389-ds-base"}, 
    {"name": "389-ds-base-devel", "epoch": 0, "version": "2.4.6", "release": "alt1", "arch": "aarch64", "disttag": "sisyphus+355588.100.2.1", "buildtime": 1724238943, "source": "389-ds-base"}, 
    {"name": "86box-debuginfo", "epoch": 0, "version": "4.2", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+356214.100.1.1", "buildtime": 1724794815, "source": "86box"}, 
    {"name": "9wm", "epoch": 0, "version": "1.4.1", "release": "alt3", "arch": "aarch64", "disttag": "sisyphus+259420.100.1.1", "buildtime": 1602159269, "source": "9wm"}, 
    {"name": "9wm-debuginfo", "epoch": 1, "version": "1.4.1", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+259420.100.1.1", "buildtime": 1602159269, "source": "9wm"} 
    ]}
"#;
#[cfg(test)]
const P10_DATA: &str = r#"
{"request_args": {"arch": null}, 
"length": 187591, 
"packages": 
    [{"name": "0ad", "epoch": 1, "version": "0.0.26", "release": "alt0_1_alpha.p10", "arch": "aarch64", "disttag": "p10+307479.400.5.1", "buildtime": 1665497454, "source": "0ad"},
    {"name": "AFLplusplus", "epoch": 0, "version": "4.20c", "release": "alt1", "arch": "aarch64", "disttag": "p10+345310.100.2.1", "buildtime": 1713865829, "source": "AFLplusplus"},
    {"name": "86box-debuginfo", "epoch": 0, "version": "4.2", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+356214.100.1.1", "buildtime": 1724794815, "source": "86box"}, 
    {"name": "389-ds-base-debuginfo", "epoch": 0, "version": "2.4.6", "release": "alt1", "arch": "aarch64", "disttag": "sisyphus+355588.100.2.1", "buildtime": 1724238943, "source": "389-ds-base"}, 
    {"name": "86box_tsdtd", "epoch": 0, "version": "4.2", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+356214.100.1.1", "buildtime": 1724794815, "source": "86box"},
    {"name": "9wm", "epoch": 0, "version": "1.4.1", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+259420.100.1.1", "buildtime": 1602159269, "source": "9wm"},
    {"name": "9wm-debuginfo", "epoch": 0, "version": "1.4.1", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+259420.100.1.1", "buildtime": 1602159269, "source": "9wm"}, 
    {"name": "none-debuginfo", "epoch": 0, "version": "6.4.1", "release": "alt2", "arch": "aarch64", "disttag": "sisyphus+259420.100.1.1", "buildtime": 1602159269, "source": "9wm"} 
    ]}
"#;
