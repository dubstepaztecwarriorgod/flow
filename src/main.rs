#![no_std]
#![no_main]

mod vga;
mod flowlib;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::VgaBuffer::new().print_str(b"meowwww");
    loop {
        
    }
}