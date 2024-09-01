use crate::Arch;
use serde::Serializer;

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

pub fn arch_serializer<S>(value: &Arch, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Arch::X86_64 => serializer.serialize_str("x86_64"),
        Arch::X86_64_i586 => serializer.serialize_str("x86_64-i586"),
        Arch::I586 => serializer.serialize_str("i586"),
        Arch::Aarch64 => serializer.serialize_str("aarch64"),
        Arch::Ppc64le => serializer.serialize_str("ppc64le"),
        Arch::Armh => serializer.serialize_str("armh"),
        Arch::NoArch => serializer.serialize_str("noarch"),
    }
}