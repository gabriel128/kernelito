#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), feature(panic_info_message))]
#![cfg_attr(test, allow(unused_imports))]
#![feature(abi_x86_interrupt)]

// Adding std manually so rust-analyzer don't freek out
#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod vga;

mod checks;
mod idt;
mod io;
mod pic;

use core::{arch::asm, panic::PanicInfo};

use vga::utils::print_ok_loading_message;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    welcome_msg();

    print_ok_loading_message("Bootlader");
    print_ok_loading_message("VGA Driver");

    idt::init();
    pic::init();

    checks::run();

    kprintln!("kernelito>");

    idt::enable_interrupts();

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

#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let (Some(args), Some(location)) = (panic_info.message(), panic_info.location()) {
        let panic_message = args.as_str().unwrap_or("Unknown Error");

        kprinterror!(
            "KERNEL PANIC! {} in {}:{}:{}",
            panic_message,
            location.file(),
            location.line(),
            location.column(),
        );
    } else {
        kprinterror!("KERNEL PANIC! Unknown ERROR");
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
