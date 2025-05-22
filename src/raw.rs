use core::arch::asm;

#[inline]
pub unsafe fn syscall1(mut eax: u32) -> (u32, u32) {
    let ebx;
    unsafe {
        asm!(
            "int 0x40",
            inlateout("eax") eax,
            out("ebx") ebx, 
        )
    }
    (eax, ebx)
}

#[inline]
pub unsafe fn syscall2(mut eax: u32, mut ebx: u32) -> (u32, u32) {
    unsafe {
            asm!(
            "int 0x40",
            inlateout("eax") eax,
            inlateout("ebx") ebx,
        )
    }
    (eax, ebx)
}

#[inline]
pub unsafe fn syscall3(mut eax: u32, mut ebx: u32, ecx: u32) -> (u32, u32) {
    unsafe { asm!(
        "int 0x40",
        inlateout("eax") eax,
        inlateout("ebx") ebx,
        in("ecx") ecx
    ) }
    (eax, ebx)
}

#[inline]
pub unsafe fn syscall4(mut eax: u32, mut ebx: u32, ecx: u32, edx: u32) -> (u32, u32) {
    unsafe { asm!(
        "int 0x40",
        inlateout("eax") eax,
        inlateout("ebx") ebx,
        in("ecx") ecx,
        in("edx") edx,
    ) }
    (eax, ebx)
}

#[inline]
pub unsafe fn syscall5(mut eax: u32, mut ebx: u32, ecx: &mut u32, edx: &mut u32, esi: &mut u32) -> (u32, u32) {
    unsafe { asm!(
        "xchg esi, {esi}",
        "int 0x40",
        "xchg {esi}, esi",
        inlateout("eax") eax,
        inlateout("ebx") ebx,
        in("ecx") ecx,
        in("edx") edx,
        esi = in(reg) esi, // llvm reserved
    ) }
    (eax, ebx)
}

#[inline]
pub unsafe fn syscall6(mut eax: u32, mut ebx: u32, ecx: u32,
                       edx: u32, esi: u32, edi: u32) -> (u32, u32) {
    unsafe { asm!(
        "xchg esi, {esi}",
        "int 0x40",
        "xchg {esi}, esi",
        inlateout("eax") eax,
        inlateout("ebx") ebx,
        in("ecx") ecx,
        in("edx") edx,
        esi = in(reg) esi,
        in("edi") edi,
    ) }
    (eax, ebx)
}

#[inline]
pub unsafe fn syscall7(mut eax: u32, mut ebx: u32, ecx: u32, edx: u32,
                       esi: u32, edi: u32, ebp: u32) -> (u32, u32) {
    unsafe { asm!(
        "push esi",
        "push ebp",
        "mov esi, DWORD PTR [ecx + 0]", // set esi
        "mov ebp, DWORD PTR [ecx + 4]", // set ebp
        "mov ecx, DWORD PTR [ecx + 8]", // set ecx
        "int 0x40",
        "pop ebp",
        "pop esi",
        inlateout("eax") eax,
        inlateout("ebx") ebx,
        in("ecx") &[esi, ebp, ecx],
        in("edx") edx,
        in("edi") edi,
    ) }
    (eax, ebx)
}
