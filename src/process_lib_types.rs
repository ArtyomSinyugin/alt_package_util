use crate::Arch;

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