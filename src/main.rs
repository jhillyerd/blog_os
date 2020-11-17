#![no_std] // don't link std library
#![no_main] // disable normal entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// The linker entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        "The nums {} and {}",
        42,
        1.0 / 3.0
    )
    .unwrap();

    loop {}
}

/// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
