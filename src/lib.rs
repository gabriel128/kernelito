#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

mod vga;

use core::{arch::asm, panic::PanicInfo};
use ufmt::uwrite;
// use lazy_static::lazy_static;
// use spin::Mutex;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::init();
    vga::printstr("Starting\n");
    let x: Option<i32> = None;
    x.unwrap();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let (Some(args), Some(location)) = (panic_info.message(), panic_info.location()) {
        let panic_message = args.as_str().unwrap();
        uwrite!(
            vga::driver_guard(),
            "Panic occurred: {} in {}:{}:{}",
            panic_message,
            location.file(),
            location.line(),
            location.column(),
        )
        .unwrap();
    } else {
        uwrite!(vga::driver_guard(), "Panic: Unknown ERROR").unwrap();
    }
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
