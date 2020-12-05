#![no_std] // don't link std library
#![no_main] // disable normal entry points
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use blog_os::{memory, memory::BootInfoFrameAllocator, println};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use alloc::boxed::Box;

    println!("** blog_os starting **");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    blog_os::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let x = Box::new(41);
    println!("heap value at {:p}", x);

    #[cfg(test)]
    test_main();

    println!("** blog_os did not crash **");
    blog_os::hlt_loop();
}

/// Called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
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
