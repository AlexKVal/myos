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

// cargo xrun
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
