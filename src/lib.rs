#![no_std]
#![cfg(any(target_arch = "x86", doc))]

mod raw;
pub use raw::*;

///
///
///
///
///


/// Runs a sysfunc (syscall). All arguments are u32: argument 1 is eax, 2 is ebx, 3 is ecx, etc.
/// Returns (u32, u32) with (eax, ebx).
///
///
/// Example:
/// ```rust
/// fn kolibrios_exit() -> ! {
///     unsafe {
///         syscall!(u32::MAX);
///         unreachable!()
///     }
/// }
/// ```
///
/// Using returned value(s):
/// ```rust
/// use core::ffi::c_void;
/// unsafe fn malloc(size: u32) -> *mut c_void {
///
///     let eax; 
///     unsafe {
///         // Sysfunc 68.12, allocate memory block
///         eax = syscall!(68, 12, size).0
///     }
///     eax as *mut c_void
/// }
/// ```
#[macro_export]
macro_rules! syscall {
    ($eax:expr) => {
        $crate::syscall1(
            $eax as u32,
        )
    };


    ($eax:expr, $ebx:expr) => {
        $crate::syscall2(
            $eax as u32,
            $ebx as u32,
        )
    };

    ($eax:expr, $ebx:expr, $ecx:expr) => {
        $crate::syscall3(
            $eax as u32,
            $ebx as u32,
            $ecx as u32,
        )
    };


    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr) => {
        $crate::syscall4(
            $eax as u32,
            $ebx as u32,
            $ecx as u32,
            $edx as u32,
        )
    };


    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr, $esi:expr) => {

        $crate::syscall5(
            $eax as u32,
            $ebx as u32,
            $ecx as u32,
            $edx as u32,
            $esi as u32,
        )
    };

    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr, $esi:expr, $edi:expr) => {
        $crate::syscall6(
            $eax as u32,
            $ebx as u32,
            $ecx as u32,
            $edx as u32,
            $esi as u32,
            $edi as u32,
        )
    };

    ($eax:expr, $ebx:expr, $ecx:expr, $edx:expr, $esi:expr, $edi:expr, $ebp:expr) => {
        $crate::syscall7(
            $eax as u32,
            $ebx as u32,
            $ecx as u32,
            $edx as u32,
            $esi as u32,
            $edi as u32,
            $ebp as u32,
        )
    };

}
