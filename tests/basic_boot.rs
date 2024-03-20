#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(blog_os::test_runner)]

use core::panic::PanicInfo;
use blog_os::{println, test_panic_handler};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
#[test_case]
fn test_println() {
    println!("test_println output");
}