use std::collections::HashMap;

use crate::{process_lib_types::arch_from_str, Arch, PName, PackageInfo};

pub fn structurize(raw_data: Vec<PackageInfo>) -> HashMap<Arch, HashMap<PName, PackageInfo>> {
    let mut map: HashMap<Arch, HashMap<PName, PackageInfo>> = HashMap::new();
    for package in raw_data {
        let mut package_map = HashMap::new();
        package_map.insert(package.name.clone(), package.clone());
        map.entry(arch_from_str(&package.arch).expect("Error: unknown architecture!")).and_modify(|value| {
            value.extend(package_map.clone())
        }).or_insert(package_map);
    }
    map
}