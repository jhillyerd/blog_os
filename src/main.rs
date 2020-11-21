#![no_std] // don't link std library
#![no_main] // disable normal entry points
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

/// The linker entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("** blog_os starting **");

    #[cfg(test)]
    test_main();

    loop {}
}

/// Called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial() {
    assert_eq!(1, 1);
}
