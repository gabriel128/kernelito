#![no_std]
#![feature(abi_x86_interrupt)]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

#[macro_use]
mod vga;

mod checks;
mod idt;

use core::{arch::asm, panic::PanicInfo};

use vga::utils::print_ok_loading_message;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    welcome_msg();

    print_ok_loading_message("Bootlader");
    print_ok_loading_message("VGA Driver");

    // checks::run();

    kprintln!("kernelito>");

    idt::init();
    loop {
        halt()
    }
}

fn welcome_msg() {
    kprintln!("");
    kprintln!("||  //        ==    ========   =====");
    kprintln!("|| //         ||       ||     ||   ||");
    kprintln!("||       ==   ||       ||     ||   ||");
    kprintln!("|| \\\\         ||       ||     ||   ||");
    kprintln!("||  \\\\        ||       ||      =====");
    kprintln!("");
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let (Some(args), Some(location)) = (panic_info.message(), panic_info.location()) {
        let panic_message = args.as_str().unwrap_or("");

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
