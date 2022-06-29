use crate::{kprint, kprint_color, kprintln, vga::Color};

pub fn run_checks() {
    kprintln!("");
    kprint_color!(Color::Green, "Starting checks...");
    kprintln!("");

    check_vga();
}

fn check_vga() {
    kprintln!("- Starting vga checks...");
    for _ in 0..(80 * 10) {
        kprint!("X");
    }

    kprintln!("");
    kprintln!("");
    kprintln!("- Starting panic checks...");
    let x: Option<i32> = None;
    x.unwrap();
}
