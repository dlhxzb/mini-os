#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// static HELLO: &[u8] = b"Hello World! This is Bo Zhang!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello again, some numbers: {} {}", 42, 1.337);

    loop {}
}

// fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
//     loop {}
// }

// bootloader_api::entry_point!(kernel_main);
