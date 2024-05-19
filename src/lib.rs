#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), feature(panic_info_message))]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(not(test), feature(abi_x86_interrupt))]
#![cfg_attr(not(test), feature(ascii_char))]

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
mod io_ports;

#[macro_use]
mod log;

mod mem;
mod multiboot;
mod pic;

use core::{arch::asm, panic::PanicInfo};

use errors::KernelError;
use multiboot::BootInfo;

pub type Result<T> = core::result::Result<T, KernelError>;

#[no_mangle]
pub extern "C" fn kmain(_dummy_arg: u32, boot_info: *const BootInfo) -> ! {
    if let Err(kernel_error) = init(boot_info) {
        kprinterror!("{}\n", kernel_error);
        panic!("Kernel init Error");
    }

    loop {
        halt()
    }
}

#[inline(always)]
fn init(boot_info: *const BootInfo) -> Result<()> {
    welcome_msg();

    let mut biggest_memory_area = None;

    unsafe {
        debug!("{:?}", (*boot_info));
        biggest_memory_area = (*boot_info).biggest_map_entry();
    }

    debug!("Biggest memory entry {:?}", biggest_memory_area);

    info!("Bootloader Finished");
    info!("VGA Driver");

    idt::init();

    info!("IDT set");

    pic::init();

    info!("PIC loaded");

    mem::init().unwrap();

    info!("Kernel Memory Managing initialized");
    info!("Paging Enabled");

    idt::enable_interrupts();

    info!("Interrupts Enabled");

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

        error!(
            "KERNEL PANIC! {} in {}:{}:{}",
            panic_message,
            location.file(),
            location.line(),
            location.column(),
        );
    } else {
        error!("KERNEL PANIC! Unknown ERROR");
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
