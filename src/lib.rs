#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

mod vga;

use core::fmt::{Arguments, Formatter, Write};
use core::panic::Location;
use core::{arch::asm, panic::PanicInfo};
use ufmt::uwrite;
// use lazy_static::lazy_static;
// use spin::Mutex;
use vga::VgaDriver;

// lazy_static! {
//     pub static ref VGA_DRIVER: Mutex<VgaDriver> = Mutex::new(VgaDriver::init());
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGA_DRIVER.lock().printstr("well well \nyep");
    // VGA_DRIVER.lock().print('A');
    let mut vga = VgaDriver::init();

    vga.printstr("Starting\n");
    let x: Option<i32> = None;
    x.unwrap();
    vga.print('B');
    vga.print('C');

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut vga = VgaDriver::init();

    if let (Some(args), Some(location)) = (panic_info.message(), panic_info.location()) {
        let panic_message = args.as_str().unwrap();
        uwrite!(
            vga,
            "Panic occurred: {} in {}:{}:{}",
            panic_message,
            location.file(),
            location.line(),
            location.column(),
        )
        .unwrap();
    } else {
        uwrite!(vga, "Panic: Unknown ERROR").unwrap();
    }
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
