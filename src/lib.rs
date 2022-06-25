#![no_std]
#![no_main]
mod vga;
use core::{arch::asm, panic::PanicInfo};

use vga::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print(
        b"Check this out, all this stuff is coming from rust!!!",
        Some(&mut 1),
        &mut 2,
        &mut 4,
        &mut 5,
        &mut 7,
        &mut 8,
        &mut 8,
        &mut 8,
    );
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
