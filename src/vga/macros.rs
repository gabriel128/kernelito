#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {{
        use crate::vga::{VgaDriver, Color};
        use core::fmt::Write;

        let mut vga = VgaDriver::new(Color::default());
        let _ = writeln!(&mut vga, $($arg)*);
        // ufmt::uwrite!(&mut vga, "\n").unwrap();
    }}
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        use crate::vga::{VgaDriver, Color};
        use core::fmt::Write;

        let mut vga = VgaDriver::new(Color::default());
        let _ = write!(&mut vga, $($arg)*);
    }}
}

#[macro_export]
macro_rules! kprint_color {
    ($color:expr, $($arg:tt)*) => {{
        use crate::vga::VgaDriver;
        use core::fmt::Write;

        let mut vga = VgaDriver::new($color);
        let _ = write!(&mut vga, $($arg)*);
    }}
}

#[macro_export]
macro_rules! kprinterror {
    ($($arg:tt)*) => {{
        use crate::vga::{VgaDriver, Color};
        use core::fmt::Write;

        let mut vga = VgaDriver::new(Color::Red);
        let _ = write!(&mut vga, $($arg)*);
    }}
}
