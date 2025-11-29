// Architecture-specific VVAR pages for vDSO mapping

#[cfg(target_arch = "riscv64")]
pub const VVAR_PAGES: usize = 2;

#[cfg(target_arch = "x86_64")]
pub const VVAR_PAGES: usize = 4;

#[cfg(target_arch = "aarch64")]
pub const VVAR_PAGES: usize = 5;

#[cfg(target_arch = "loongarch64")]
pub const VVAR_PAGES: usize = 44;
