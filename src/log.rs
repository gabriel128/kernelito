#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use crate::vga::Color;

        kprint_color!(Color::White, "[");
        kprint_color!(Color::LightGreen, "INFO");
        kprint_color!(Color::White, "] ");
        kprint_color!(Color::White, $($arg)*);
        kprintln!("");
    }}
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use crate::vga::Color;

        kprint_color!(Color::White, "[");
        kprint_color!(Color::LightBlue, "DEBUG");
        kprint_color!(Color::White, "] ");
        kprint_color!(Color::White, $($arg)*);
        kprintln!("");
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use crate::vga::Color;

        kprint_color!(Color::White, "[");
        kprint_color!(Color::Red, "ERROR");
        kprint_color!(Color::White, "] ");
        kprint_color!(Color::White, $($arg)*);
        kprintln!("");
    }}
}
