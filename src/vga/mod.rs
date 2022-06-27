use core::convert::Infallible;

use ufmt::uWrite;

static VGA_MEMORY_ADDR: u32 = 0xb8000;
const MAX_WIDTH: u8 = 80;
const MAX_HEIGHT: u8 = 25;

// TODO
// - add print fn

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VgaDriver {
    curr_x: u8,
    curr_y: u8,
}

impl core::fmt::Write for VgaDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_byte(b'C', Color::Red);
        // self.printstr(s);
        Ok(())
    }
}

impl uWrite for VgaDriver {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        // self.print_byte(b'C', Color::Red);
        self.printstr(s);
        Ok(())
    }
}

impl VgaDriver {
    pub fn init() -> Self {
        let mut this = Self {
            curr_x: 0,
            curr_y: 0,
        };

        this.clean_screen();
        this.curr_x = 0;
        this.curr_y = 0;

        this
    }

    fn current_mem_position(&self) -> *mut u8 {
        let y: u32 = (self.curr_y as u16 * 2 * MAX_WIDTH as u16).into();
        let x: u32 = (2 * self.curr_x).into();
        (VGA_MEMORY_ADDR + y + x) as *mut u8
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

        unsafe {
            *curr_mem_position.offset(0 as isize) = a_byte;
            *curr_mem_position.offset(1 as isize) = color.into();
        }

        self.move_cursor_next();
    }

    pub fn print(&mut self, a_char: char) {
        self.print_byte(a_char as u8, Color::White)
    }

    pub fn printstr2(&mut self, a_str: &str) {
        self.printstr3(a_str);
    }

    pub fn printstr3(&mut self, a_str: &str) {
        self.printstr(a_str);
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
