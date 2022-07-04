#![allow(dead_code)]
use core::arch::asm;

use crate::{kprint, kprint_color, vga::Color};

#[cfg(not(feature = "checks-mode"))]
pub fn run() {}

#[cfg(feature = "checks-mode")]
pub fn run() {
    kprint_color!(Color::Green, "Starting checks... \n");

    check_vga();
    // check_interrupts();
    // check_opt_panics();
    check_res_panics();
}

pub fn check_vga() {
    kprint_color!(
        Color::Green,
        "\n============== Starting VGA checks ==============\n"
    );
    for _ in 0..(81 * 10) {
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

fn check_interrupts() {
    divide_by_zero()
}

fn divide_by_zero() {
    unsafe { asm!("mov dx, 0; div dx") }
}
