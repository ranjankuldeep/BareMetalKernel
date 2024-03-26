#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Helllo World {}", "!");

    blog_os::init();
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}