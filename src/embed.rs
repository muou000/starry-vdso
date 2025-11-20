use core::arch::global_asm;

#[cfg(target_arch = "riscv64")]
global_asm!("
    .global vdso_start, vdso_end
    .section .data
    .balign 4096
vdso_start:
    .incbin \"vdso/vdso_rv.so\"
    .balign 4096
vdso_end:
    .type vdso_start, @object
    .type vdso_end, @object
    .previous
");

#[cfg(target_arch = "loongarch64")]
global_asm!("
    .global vdso_start, vdso_end
    .section .data
    .balign 4096
vdso_start:
    .incbin \"vdso/vdso_la.so\"
    .balign 4096
vdso_end:
    .type vdso_start, @object
    .type vdso_end, @object
    .previous
");

#[cfg(target_arch = "aarch64")]
global_asm!("
    .global vdso_start, vdso_end
    .section .data
    .balign 4096
vdso_start:
    .incbin \"vdso/vdso_aarch.so\"
    .balign 4096
vdso_end:
    .type vdso_start, @object
    .type vdso_end, @object
    .previous
");

#[cfg(target_arch = "x86_64")]
global_asm!("
    .global vdso_start, vdso_end
    .section .data
    .balign 4096
vdso_start:
    .incbin \"vdso/vdso_x86.so\"
    .balign 4096
vdso_end:
    .type vdso_start, @object
    .type vdso_end, @object
    .previous
");