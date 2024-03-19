#![no_std]
#![no_main]
use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", Some Numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}