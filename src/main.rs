#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_one!("{info}");
    loop {}
}




#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_one!("Hello");
    print_one!();
    print_one!("Rust");
    panic!("Some panic message");
}
