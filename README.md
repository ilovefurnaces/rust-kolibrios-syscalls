## KolibriOS syscalls for rust. 

Use macro `syscall!(eax: u32, ebx: u32, ..) -> (u32, u32)`.
Macro returns (eax, ebx).

## Examples

### Exit:
```rust
fn kolibrios_exit() -> ! {
    unsafe {
        syscall!(u32::MAX);
        unreachable!()
    }
}
```

### Using returned value(s):
```rust
use core::ffi::c_void;
/// Allocates x pages so that x*page_size > size
unsafe fn alloc(size: u32) -> *mut c_void {

    let eax; 
    unsafe {
        // Sysfunc 68.12, allocate memory block
        eax = syscall!(68, 12, size).0;
    };
    eax as *mut c_void
}
```

### Hello World: (will write to debug board)
```rust
let string = "hello world";
for i in string.bytes() {
    unsafe {
        syscall!(63, 1, i);
    };
}
```
