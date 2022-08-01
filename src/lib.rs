#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), feature(panic_info_message))]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(not(test), feature(abi_x86_interrupt))]

// Adding std manually so rust-analyzer don't freek out
#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod vga;

mod checks;
mod cpu;
mod errors;
mod idt;
mod io;
mod mem;
mod pic;

use core::{arch::asm, panic::PanicInfo};

use errors::KernelError;
use vga::utils::print_ok_loading_message;

pub type Result<T> = core::result::Result<T, KernelError>;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    if let Err(kernel_error) = init() {
        kprinterror!("{}\n", kernel_error);
        panic!("Kernel init Error");
    }

    loop {
        halt()
    }
}

#[inline(always)]
fn init() -> Result<()> {
    welcome_msg();

    print_ok_loading_message("Bootlader");
    print_ok_loading_message("VGA Driver");

    idt::init();

    print_ok_loading_message("IDT set");

    pic::init();

    print_ok_loading_message("PIC loaded");

    mem::init()?;

    print_ok_loading_message("Kernel Memory Managing initialized");
    print_ok_loading_message("Paging Enabled");

    idt::enable_interrupts();

    print_ok_loading_message("Interrupts Enabled");

    kprintln!("\nkernelito>");

    checks::run();

    Ok(())
}

fn welcome_msg() {
    kprintln!("");
    kprintln!("              []");
    kprintln!("||  //              ========   =====");
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
