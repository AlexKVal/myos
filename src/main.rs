#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

static HELLO: &[u8] = b"Oh Yeah! This is MyOS :)";

// cargo xrun
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
        }
    }

    loop {}
}
