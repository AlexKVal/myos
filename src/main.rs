#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// cargo xrun
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    panic!("Some panic message");

    loop {}
}
