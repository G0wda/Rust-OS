#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



static HELLO: &[u8] = b"Hello world";
static MESSAGE: &[u8] = b"Welcome to RUST OS";
static MESSAGE_2: &[u8] = b"This is a Test Kernel written in Rust Language";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    
    // Write HELLO
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let row_length = 80; // VGA width
    let mut offset = row_length; // start of 2nd line

    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((offset + i) as isize * 2) = byte;
            *vga_buffer.offset((offset + i) as isize * 2 + 1) = 0xb;
        }
    }
    offset = 160;

    for ( i, &byte) in  MESSAGE_2.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((offset + i) as isize * 2) = byte;
            *vga_buffer.offset((offset + i) as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
