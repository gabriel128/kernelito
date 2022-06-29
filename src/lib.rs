#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]
// use te::vga::kprintln;

mod vga;

use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
pub fn _start() -> ! {
    vga::init();
    vga::utils::print_ok_loading_message("VGA Driver loaded");

    vga::utils::test_screen();

    loop {
        halt();
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let (Some(args), Some(location)) = (panic_info.message(), panic_info.location()) {
        let panic_message = args.as_str().unwrap();

        kprinterror!(
            "Panic occurred: {} in {}:{}:{}",
            panic_message,
            location.file(),
            location.line(),
            location.column(),
        );
    } else {
        kprinterror!("Panic: Unknown ERROR");
    }

    loop {
        halt();
    }
}

fn halt() {
    unsafe {
        asm!("hlt");
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
