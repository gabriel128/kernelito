#![no_std]
#![no_main]
mod vga;
use core::panic::PanicInfo;

use vga::print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(
        b"Hello there my dear friend, trying some stuff over here",
        Some(&mut 1),
        &mut 2,
        &mut 4,
        &mut 5,
        &mut 7,
        &mut 8,
    );
    loop {}
}
