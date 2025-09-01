#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_byte(b'h', 0);
    vga_buffer::print_string("Welcome to Rust OS",0);
    loop {}
}
