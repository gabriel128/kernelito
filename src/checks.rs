use crate::{kprint, kprintln};

pub fn run_checks() {
    kprintln!("Starting checks...");

    check_vga();
}

fn check_vga() {
    kprintln!("Starting vga check...");
    for _ in 0..(80 * 20) {
        kprint!("X");
    }

    let x: Option<i32> = None;
    x.unwrap();
}
