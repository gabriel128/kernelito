use super::{Color, VgaDriver};

pub fn print_ok_loading_message(message: &str) {
    let green_vga = VgaDriver::new(Color::Green);
    let default_vga = VgaDriver::new(Color::default());
    default_vga.print("[");
    green_vga.print("OK");
    default_vga.print("] ");
    default_vga.println(message);
}

pub fn print_error_loading_message(message: &str) {
    let red_vga = VgaDriver::new(Color::Red);
    let default_vga = VgaDriver::new(Color::default());
    default_vga.print("[");
    red_vga.print("ERROR");
    default_vga.print("] ");
    default_vga.println(message);
}

pub fn test_screen() {
    kprintln!("Starting test...");
    for _ in 0..(80 * 22) {
        kprint!("X");
    }

    let x: Option<i32> = None;
    x.unwrap();
}
