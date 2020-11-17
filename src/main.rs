#![no_std] // don't link std library
#![no_main] // disable normal entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// The linker entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello macro World{}", "!");

    loop {}
}

/// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
