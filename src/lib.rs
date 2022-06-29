#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

mod checks;
mod vga;

use core::{arch::asm, panic::PanicInfo};

use vga::utils::print_ok_loading_message;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::init();
    print_ok_loading_message("Bootlader");
    print_ok_loading_message("VGA Driver");

    checks::run_checks();

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
