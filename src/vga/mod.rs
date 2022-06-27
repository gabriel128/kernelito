#![allow(dead_code)]
use core::convert::Infallible;

use lazy_static::lazy_static;
use spin::Mutex;
use ufmt::uWrite;

static VGA_MEMORY_ADDR: u32 = 0xb8000;
const MAX_WIDTH: u8 = 80;
const MAX_HEIGHT: u8 = 25;

lazy_static! {
    pub static ref VGA_DRIVER: Mutex<VgaDriver> = Mutex::new(VgaDriver::new());
}

pub fn init() {
    VGA_DRIVER.lock().init();
}

pub fn printstr(s: &str) {
    VGA_DRIVER.lock().printstr(s);
}

pub fn driver_guard() -> spin::MutexGuard<'static, VgaDriver> {
    VGA_DRIVER.lock()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VgaDriver {
    curr_x: u8,
    curr_y: u8,
}

impl uWrite for VgaDriver {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.printstr(s);
        Ok(())
    }
}

impl VgaDriver {
    pub fn new() -> Self {
        Self {
            curr_x: 0,
            curr_y: 0,
        }
    }

    pub fn init(&mut self) {
        self.clean_screen();
        self.curr_x = 0;
        self.curr_y = 0;
    }

    fn current_mem_position(&self) -> *mut u16 {
        let y: u32 = (self.curr_y as u16 * 2 * MAX_WIDTH as u16).into();
        let x: u32 = (2 * self.curr_x).into();
        (VGA_MEMORY_ADDR + y + x) as *mut u16
    }

    fn move_cursor_next(&mut self) {
        if self.curr_y >= MAX_HEIGHT {
            // Nothing for now
        } else if self.curr_x >= MAX_WIDTH {
            self.next_line();
        } else {
            self.curr_x += 1;
        }
    }

    fn next_line(&mut self) {
        self.curr_x = 0;
        self.curr_y += 1;
    }

    pub fn clean_screen(&mut self) {
        for _ in 0..MAX_HEIGHT {
            for _ in 0..MAX_WIDTH {
                self.print_byte(b' ', Color::Black);
            }
        }
    }

    fn print_byte(&mut self, a_byte: u8, color: Color) {
        if a_byte == b'\n' {
            self.next_line();
            return;
        }

        let curr_mem_position = self.current_mem_position();

        let the_color: u8 = color.into();

        unsafe {
            *curr_mem_position.offset(0 as isize) = ((the_color as u16) << 8) | a_byte as u16;
        }

        self.move_cursor_next();
    }

    pub fn print(&mut self, a_char: char) {
        self.print_byte(a_char as u8, Color::White)
    }

    pub fn printstr(&mut self, a_str: &str) {
        for a_char in a_str.as_bytes() {
            self.print_byte(*a_char, Color::White)
        }
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White,
    Red,
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0x0,
            Color::White => 0xF,
            Color::Red => 0x4,
        }
    }
}

// Call with b"Check this out, all this stuff is coming from rust!!!",
pub fn test_print(text: &[u8]) {
    let vga_buffer = VGA_MEMORY_ADDR as *mut u8;

    unsafe {
        *vga_buffer.offset(0 as isize) = *text.get(0).unwrap();
        *vga_buffer.offset(1 as isize) = 0xb;
    }
}
