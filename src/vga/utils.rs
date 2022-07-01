use crate::{kprint_color, kprintln};

use super::Color;

#[inline(always)]
pub fn print_ok_loading_message(message: &str) {
    kprint_color!(Color::White, "[");
    kprint_color!(Color::LightGreen, "OK");
    kprint_color!(Color::White, "] {}", message);
    kprintln!("");
}
