use crate::{CompareResult, Sisyphus, P10};
use rpm::Evr;

pub fn compare(sisyphus: Sisyphus, mut p10: P10) -> Vec<CompareResult> {
    let mut compare_result: Vec<CompareResult> = Vec::new();

    for (arch, v_sis) in sisyphus {
        let mut compare_temp: CompareResult = CompareResult { 
            arch: arch.clone(), 
            unique_for_sisyphus: Vec::new(), 
            unique_for_p10: Vec::new(), 
            sisyphus_has_greater_version: Vec::new() };
        if p10.contains_key(&arch) {
            let p10_value = p10.get_mut(&arch).expect("Could not get value in p10 by sisyphus arch");
            for (package_name, package_info) in v_sis {
                if p10_value.contains_key(&package_name) {
                    let p10_value_to_compare = p10_value.get(&package_name).expect("Package unwrap compare error");
                    if Evr::new(&package_info.epoch.to_string(), &package_info.version, &package_info.release) > Evr::new(&p10_value_to_compare.epoch.to_string(), &p10_value_to_compare.version, &p10_value_to_compare.release) {
                        compare_temp.sisyphus_has_greater_version.push(package_info.clone());
                    }
                    p10_value.remove(&package_name);
                } else {
                    compare_temp.unique_for_sisyphus.push(package_info);
                }
            }
        } else {
            // Arch is only in Sis. So, all packages are also only in sis
            for (_, package_info) in v_sis {
                compare_temp.unique_for_sisyphus.push(package_info);
            }
        }
        compare_result.push(compare_temp);
    }

    for (arch, v_p10) in p10 {
        let mut compare_temp: CompareResult = CompareResult { 
            arch: arch.clone(), 
            unique_for_sisyphus: Vec::new(), 
            unique_for_p10: Vec::new(), 
            sisyphus_has_greater_version: Vec::new() };
        for (_, package_info) in v_p10 {
            compare_temp.unique_for_p10.push(package_info);
        }
        if compare_result.iter().any(|match_result| &match_result.arch == &arch) {
            compare_result.iter_mut().for_each(|compare_result| {
                if &compare_result.arch == &arch {
                    compare_result.unique_for_p10.append(compare_temp.unique_for_p10.as_mut());
                }
            });
        } else {
            compare_result.push(compare_temp);
        }
    }
    compare_result
}