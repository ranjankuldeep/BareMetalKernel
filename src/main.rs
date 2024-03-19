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
    println!("Helllo World {}", "!");

    loop {}
}