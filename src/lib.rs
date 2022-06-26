#![no_std]
#![no_main]
mod vga;
use core::{arch::asm, panic::PanicInfo};

use lazy_static::lazy_static;
use spin::Mutex;
use vga::VgaDriver;

lazy_static! {
    pub static ref VGA_DRIVER: Mutex<VgaDriver> = Mutex::new(VgaDriver::init());
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    VGA_DRIVER.lock().printstr("well well \nyep");
    // VGA_DRIVER.lock().print('A');

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    VGA_DRIVER.lock().print('F');
    loop {}
}
