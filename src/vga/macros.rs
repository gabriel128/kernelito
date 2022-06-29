#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {{
        use crate::vga::{VgaDriver, Color};

        let mut vga = VgaDriver::new(Color::default());
        ufmt::uwrite!(&mut vga, $($arg)*).unwrap();
        ufmt::uwrite!(&mut vga, "\n").unwrap();
    }}
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        use crate::vga::{VgaDriver, Color};

        let mut vga = VgaDriver::new(Color::default());
        ufmt::uwrite!(&mut vga, $($arg)*).unwrap();
    }}
}

#[macro_export]
macro_rules! kprinterror {
    ($($arg:tt)*) => {{
        use crate::vga::{VgaDriver, Color};

        let mut vga = VgaDriver::new(Color::Red);
        ufmt::uwrite!(&mut vga, $($arg)*).unwrap();
    }}
}
