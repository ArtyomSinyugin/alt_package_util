pub mod fetch_from_url;
pub mod branch_compare;
pub mod data_structurize;
pub mod process_lib_types;
pub mod tests;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::process_lib_types::arch_serializer;

pub type Sisyphus = HashMap<Arch, HashMap<PName, PackageInfo>>;
pub type P10 = HashMap<Arch, HashMap<PName, PackageInfo>>;

pub type PName = String;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[allow(non_camel_case_types)]
pub enum Arch {
    X86_64,
    X86_64_i586,
    I586,
    Aarch64,
    Ppc64le,
    Armh,
    NoArch,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct PackageInfo {
    #[serde(rename = "name")]
    pub name: PName,
    #[serde(rename = "epoch")]
    epoch: u32,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "release")]
    pub release: String,
    #[serde(rename = "arch")]
    pub arch: String,
    #[serde(rename = "disttag")]
    disttag: String,
    #[serde(rename = "buildtime")]
    buildtime: u64,
    #[serde(rename = "source")]
    source: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct ArchFromApi {
    arch: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct ApiResponse {
    request_args: ArchFromApi,
    length: u32,
    packages: Vec<PackageInfo>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct CompareResult {
    #[serde(serialize_with = "arch_serializer")]
    #[serde(rename = "arch")]
    pub arch: Arch,
    #[serde(rename = "unique for sisyphus")]
    pub unique_for_sisyphus: Vec<PackageInfo>,
    #[serde(rename = "unique for p10")]
    pub unique_for_p10: Vec<PackageInfo>,
    #[serde(rename = "sisyphus has greater version")]
    pub sisyphus_has_greater_version: Vec<PackageInfo>,
}