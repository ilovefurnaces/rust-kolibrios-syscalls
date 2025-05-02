KolibriOS syscalls for rust. 

Use macro `syscall!(&mut eax, &mut ebx, ..)`.

Example:
```rust
fn kolibrios_exit() -> ! {
    unsafe {
        syscall!(&mut -1);
    }
}
```

Using returned value(s):
```rust
use core::ffi::c_void;
unsafe fn malloc(mut size: u32) -> *mut c_void {
    //            ^----- is not actually mutated

    let mut eax = 68; //Function number
    unsafe {
        // Sysfunc 68.12, allocate memory block
        syscall!(&mut eax, &mut 12, &mut size)
    }
    eax as *mut c_void
}
```

Hello world: (will write to debug board)
```rust
let string = "hello world";
for i in string.bytes() {
unsafe {
    syscall!(&mut 63, &mut 1, &mut (i as u32))
};
}
```
