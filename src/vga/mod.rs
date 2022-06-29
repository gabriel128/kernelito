#![allow(dead_code)]
use core::{
    convert::Infallible,
    sync::atomic::{AtomicI16, Ordering},
};

pub mod utils;
use ufmt::uWrite;
#[macro_use]
pub mod macros;

static VGA_MEMORY_ADDR: i32 = 0xb8000;
const WIDTH: i16 = 80;
const HEIGHT: i16 = 25;

static CURRX: AtomicI16 = AtomicI16::new(0);
static CURRY: AtomicI16 = AtomicI16::new(0);

#[inline(always)]
pub fn init() {
    let vga = VgaDriver::new(Color::default());
    vga.init()
}

pub fn cursor_coords() -> (i16, i16) {
    (CURRX.load(Ordering::Relaxed), CURRY.load(Ordering::Relaxed))
}

#[derive(Debug, Clone)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    LightGreen,
    Gray,
}

impl Default for Color {
    fn default() -> Self {
        Color::Gray
    }
}

impl From<&Color> for u8 {
    fn from(color: &Color) -> Self {
        match color {
            Color::Black => 0x0,
            Color::White => 0xF,
            Color::Red => 0x4,
            Color::Green => 0x2,
            Color::Gray => 0x7,
            Color::LightGreen => 0xA,
        }
    }
}
impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0x0,
            Color::White => 0xF,
            Color::Red => 0x4,
            Color::Green => 0x2,
            Color::Gray => 0x7,
            Color::LightGreen => 0xA,
        }
    }
}

#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: u8,
}

impl ScreenChar {
    fn new(ascii: u8, color: u8) -> Self {
        Self { ascii, color }
    }
}

impl Into<u16> for ScreenChar {
    fn into(self) -> u16 {
        ((self.color as u16) << 8) | self.ascii as u16
    }
}

#[derive(Debug, Clone)]
pub struct VgaDriver {
    char_color: Color,
}

impl uWrite for VgaDriver {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.print(s);
        Ok(())
    }
}

impl VgaDriver {
    pub fn new(char_color: Color) -> Self {
        Self { char_color }
    }

    pub fn init(&self) {
        self.clean_screen();
        CURRX.store(0, Ordering::Relaxed);
        CURRY.store(0, Ordering::Relaxed);
    }

    fn curr_x(&self) -> i16 {
        CURRX.load(Ordering::Relaxed)
    }
    fn curr_y(&self) -> i16 {
        CURRY.load(Ordering::Relaxed)
    }

    // (1, 0) => 2
    // (10, 0) => 20
    // (0, 1) => 81
    // (80, 1) => 81
    fn current_mem_position(&self) -> *mut u16 {
        let y: i32 = (2 * self.curr_y() * WIDTH).into();
        let x: i32 = (2 * self.curr_x()).into();
        (VGA_MEMORY_ADDR + y + x) as *mut u16
    }

    fn move_cursor_next(&self) {
        if self.curr_y() as i16 >= HEIGHT {
            // Nothing for now
        } else if self.curr_x() >= WIDTH {
            self.next_line();
        } else {
            CURRX.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn next_line(&self) {
        CURRX.store(0, Ordering::Relaxed);
        CURRY.fetch_add(1, Ordering::Relaxed);
    }

    pub fn clean_screen(&self) {
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                self.print_byte(b' ', &Color::Black);
            }
        }
    }

    fn print_byte(&self, a_byte: u8, color: &Color) {
        if a_byte == b'\n' {
            self.next_line();
            return;
        }

        let curr_mem_position = self.current_mem_position();

        unsafe {
            core::ptr::write_volatile(
                curr_mem_position.offset(0 as isize),
                ScreenChar::new(a_byte, color.into()).into(),
            );
        }

        self.move_cursor_next();
    }

    pub fn print_char(&self, a_char: char) {
        self.print_byte(a_char as u8, &self.char_color)
    }

    pub fn print(&self, a_str: &str) {
        for a_char in a_str.as_bytes() {
            self.print_byte(*a_char, &self.char_color)
        }
    }

    pub fn println(&self, a_str: &str) {
        self.print(a_str);
        self.print_byte(b'\n', &self.char_color)
    }
}

// For local dummy testing.
// Call with b"Check this out, all this stuff is coming from rust!!!",
fn test_print(text: &[u8]) {
    let vga_buffer = VGA_MEMORY_ADDR as *mut u8;

    unsafe {
        *vga_buffer.offset(0 as isize) = *text.get(0).unwrap();
        *vga_buffer.offset(1 as isize) = 0xb;
    }
}
