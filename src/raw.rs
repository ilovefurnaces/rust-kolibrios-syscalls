use core::arch::asm;

#[inline]
pub unsafe fn syscall1(eax: &mut i32) {
    unsafe {
        asm!(
            "int 0x40",
            inout("eax") *eax,
        )
    }
}

#[inline]
pub unsafe fn syscall2(eax: &mut i32, ebx: &mut u32) {
    unsafe {
            asm!(
            "int 0x40",
            inout("eax") *eax,
            inout("ebx") *ebx,
        )
    }
}

#[inline]
pub unsafe fn syscall3(eax: &mut i32, ebx: &mut u32, ecx: &mut u32) {
    unsafe { asm!(
        "int 0x40",
        inout("eax") *eax,
        inout("ebx") *ebx,
        inout("ecx") *ecx
    ) }
}

#[inline]
pub unsafe fn syscall4(eax: &mut i32, ebx: &mut u32, ecx: &mut u32, edx: &mut u32) {
    unsafe { asm!(
        "int 0x40",
        inout("eax") *eax,
        inout("ebx") *ebx,
        inout("ecx") *ecx,
        inout("edx") *edx,
    ) }
}

#[inline]
pub unsafe fn syscall5(eax: &mut i32, ebx: &mut u32, ecx: &mut u32, edx: &mut u32, esi: &mut u32) {
    unsafe { asm!(
        "push esi",
        "mov esi, {esi}",
        "int 0x40",
        "mov {esi}, esi",
        "pop esi",
        inout("eax") *eax,
        inout("ebx") *ebx,
        inout("ecx") *ecx,
        inout("edx") *edx,
        esi = in(reg) *esi, // "llvm reserved"
    ) }
}

#[inline]
pub unsafe fn syscall6(eax: &mut i32, ebx: &mut u32, ecx: &mut u32, edx: &mut u32, esi: &mut u32, edi: &mut u32) {
    unsafe { asm!(
        "push esi",
        "push edi",
        "mov esi, {esi}",
        "mov edi, {edi}",
        "int 0x40",
        "mov {esi}, esi",
        "mov {edi}, edi",
        "pop edi",
        "pop esi",
        inout("eax") *eax,
        inout("ebx") *ebx,
        inout("ecx") *ecx,
        inout("edx") *edx,
        esi = inout(reg) *esi,
        edi = inout(reg) *edi,
    ) }
}

#[inline]
pub unsafe fn syscall7(func: &mut i32, ebx: &mut u32, ecx: &mut u32, edx: &mut u32,
                        esi: &mut u32, edi: &mut u32, ebp: &mut u32) {
    unsafe { asm!(
        "push esi",
        "push edi",
        "push ebp",
        "mov esi, {esi}",
        "mov edi, {edi}",
        "mov ebp, {ebp}",
        "int 0x40",
        "mov {esi}, esi",
        "mov {edi}, edi",
        "mov {ebp}, ebp",
        "pop ebp",
        "pop edi",
        "pop esi",
        inout("eax") *func,
        inout("ebx") *ebx,
        inout("ecx") *ecx,
        inout("edx") *edx,
        esi = inout(reg) *esi,
        edi = inout(reg) *edi,
        ebp = inout(reg) *ebp,
    ) }
}
