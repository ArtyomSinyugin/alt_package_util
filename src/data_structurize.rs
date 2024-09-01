use std::collections::HashMap;

use crate::{Arch, PName, PackageInfo};

pub fn arch_from_str(arch: &str) -> Result<Arch, ()> {
    match arch {
        "armh" => Ok(Arch::Armh),
        "i586" => Ok(Arch::I586),
        "noarch" => Ok(Arch::NoArch),
        "x86_64" => Ok(Arch::X86_64),
        "ppc64le" => Ok(Arch::Ppc64le),
        "aarch64" => Ok(Arch::Aarch64),
        "x86_64-i586" => Ok(Arch::X86_64_i586),
        _ => Err(()),
    }
}

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