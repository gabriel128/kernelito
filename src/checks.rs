#![allow(dead_code)]
use core::arch::asm;

use crate::{
    kprint, kprint_color,
    mem::sync::{spin_mutex::SpinMutex, spin_rw_lock::RwLock},
    vga::Color,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_MUTEX: SpinMutex<u8> = SpinMutex::new(0);
    static ref TEST_RW_LOCK: RwLock<u8> = RwLock::new(0);
}

#[cfg(not(feature = "checks-mode"))]
pub fn run() {}

#[cfg(feature = "checks-mode")]
pub fn run() {
    kprint_color!(Color::Green, "Starting checks... \n");

    check_vga();
    check_locks();
    // check_interrupts();
    // check_opt_panics();
    // check_res_panics();
    // page_fault();
}

pub fn check_vga() {
    kprint_color!(
        Color::Green,
        "\n============== Starting VGA checks ==============\n"
    );
    for _ in 0..(80 * 10) {
        kprint!("X");
    }

    kprint_color!(
        Color::Green,
        "==============vga checks finished==============\n"
    );
}

fn check_opt_panics() {
    kprint_color!(
        Color::Green,
        "\n============== Starting panic checks ==============\n"
    );
    let x: Option<i32> = None;
    x.unwrap();
}

fn check_res_panics() {
    kprint_color!(
        Color::Green,
        "\n============== Starting Result panic checks ==============\n"
    );
    let x: Result<i32, i32> = Err(0);
    x.unwrap();
}

fn check_locks() {
    kprint_color!(
        Color::Green,
        "\n============== Starting mutex checks ==============\n"
    );
    *TEST_MUTEX.lock() = 1;
    kprintln!("Locked result should be 1 and it is {}", *TEST_MUTEX.lock());

    let mut write_guard = TEST_RW_LOCK.write();
    *write_guard = 1;
    kprintln!("RwLocked result should be 1 and it is {}", *write_guard)
}

fn check_interrupts() {
    double_fault();
    // divide_by_zero();
}

fn divide_by_zero() {
    unsafe { asm!("mov dx, 0; div dx") }
}

fn page_fault() {
    let a = 0xdeadbeef as *mut u32;

    unsafe {
        *a = 11;
    }
}

fn double_fault() {
    unsafe {
        asm!("int 8");
    };
}
