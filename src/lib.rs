#![no_std]
#![cfg(target_arch="x86")]

mod raw;
pub use raw::*;

///
///
///
///
///


/// Runs a sysfunc (syscall). Doesn't have a return value, instead mutates the arguments. That's because some (at least two) kolibriOS sysfunctions mutate many registers at once.
/// First argument (eax, function number) is a &mut i32, all others are &mut u32.
///
/// Example:
/// ```rust
/// fn kolibrios_exit() -> ! {
///     unsafe {
///         syscall!(&mut -1);
///     }
/// }
/// ```
///
/// Using returned value(s):
/// ```rust
/// use core::ffi::c_void;
/// unsafe fn malloc(mut size: u32) -> *mut c_void {
///     //            ^----- is not actually mutated
///
///     let mut eax = 68; //Function number
///     unsafe {
///         // Sysfunc 68.12, allocate memory block
///         syscall!(&mut eax, &mut 12, &mut size)
///     }
///     eax as *mut c_void
/// }
/// ```
#[macro_export]
macro_rules! syscall {
    ($eax:expr) => {
        $crate::syscall1(
            $eax,
        )
    };


    ($eax:expr, $ebx:expr) => {
        $crate::syscall2(
            $eax,
            $ebx,
        )
    };

    ($eax:expr, $ebx:expr, $ecx:expr) => {
        $crate::syscall3(
            $eax,
            $ebx,
            $ecx,
        )
    };


    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr) => {
        $crate::syscall4(
            $eax,
            $ebx,
            $ecx,
            $edx,
        )
    };


    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr, $esi:expr) => {

        $crate::syscall5(
            $eax,
            $ebx,
            $ecx,
            $edx,
            $esi,
        )
    };

    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr, $esi:expr, $edi:expr) => {
        $crate::syscall6(
            $eax,
            $ebx,
            $ecx,
            $edx,
            $esi,
            $edi,
        )
    };

    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr, $esi:expr, $edi:expr, $ebp:expr) => {
        $crate::syscall7(
            $eax,
            $ebx,
            $ecx,
            $edx,
            $esi,
            $edi,
            $ebp,
        )
    };

}
