#![allow(dead_code)]
use core::sync::atomic::{AtomicU32, Ordering};

pub mod utils;
#[macro_use]
pub mod macros;

static VGA_MEMORY_ADDR: u32 = 0xb8000;

const WIDTH: u32 = 80;
const HEIGHT: u32 = 25;
// 80 * 25 * 2
const MEM_OFFSET_MAX: u32 = 4000;

// Global state on where the x coordinate is positioned
static CURRX: AtomicU32 = AtomicU32::new(0);

#[inline(always)]
pub fn clean_screen() {
    let vga = VgaDriver::new(Color::default());
    vga.clean_screen();
}

pub fn get_x_coord() -> u32 {
    CURRX.load(Ordering::SeqCst)
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
    vga_mem_addr: u32,
}

impl core::fmt::Write for VgaDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl VgaDriver {
    pub fn new(char_color: Color) -> Self {
        Self {
            char_color,
            vga_mem_addr: VGA_MEMORY_ADDR,
        }
    }

    pub fn new_with_addr(char_color: Color, vga_mem_addr: u32) -> Self {
        Self {
            char_color,
            vga_mem_addr,
        }
    }

    #[inline(always)]
    pub fn print_char(&self, a_char: char) {
        self.print_byte(a_char as u8, &self.char_color)
    }

    #[inline(always)]
    pub fn print(&self, a_str: &str) {
        for a_char in a_str.as_bytes() {
            self.print_byte(*a_char, &self.char_color)
        }
    }

    #[inline(always)]
    pub fn println(&self, a_str: &str) {
        self.print(a_str);
        self.print("\n");
    }

    #[inline(always)]
    pub fn clean_screen(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.print_byte_at(b' ', &Color::Black, x, y);
            }
        }
    }

    pub fn scroll_up_one_line(&self) {
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                let read_mem_addr = self.mem_position_from_coords(x, y);
                let write_mem_addr = self.mem_position_from_coords(x, y - 1);

                let mem_value = self.read_screen_char_from_screen(read_mem_addr);
                self.write_to_screen(write_mem_addr, mem_value);
            }
        }

        self.clean_last_row()
    }

    fn curr_x(&self) -> u32 {
        CURRX.load(Ordering::SeqCst)
    }

    #[inline(always)]
    fn set_x(&self, x: u32) {
        CURRX.store(x, Ordering::SeqCst)
    }

    fn increase_x(&self) {
        CURRX.fetch_add(1, Ordering::SeqCst);
    }

    fn last_row(&self) -> u32 {
        HEIGHT - 1
    }

    #[inline(always)]
    fn print_byte(&self, a_byte: u8, color: &Color) {
        if a_byte == b'\n' {
            self.next_line();
            return;
        }

        self.print_byte_at(a_byte, color, self.curr_x(), self.last_row());

        self.move_cursor_right();
    }

    #[inline(always)]
    fn print_byte_at(&self, a_byte: u8, color: &Color, x: u32, y: u32) {
        let curr_mem_position = self.mem_position_from_coords(x, y);

        self.write_to_screen(
            curr_mem_position,
            ScreenChar::new(a_byte, color.into()).into(),
        );
    }

    fn mem_position_from_coords(&self, x: u32, y: u32) -> u32 {
        let translated_y: u32 = (2 * y * WIDTH).into();
        let translated_x: u32 = (2 * x).into();

        self.vga_mem_addr + translated_x + translated_y
    }

    fn move_cursor_right(&self) {
        if self.curr_x() >= (WIDTH - 1) {
            self.next_line();
        } else {
            self.increase_x();
        }
    }

    fn next_line(&self) {
        self.set_x(0);
        self.scroll_up_one_line();
    }

    fn clean_last_row(&self) {
        for x in 0..WIDTH {
            self.print_byte_at(b' ', &Color::Black, x, self.last_row());
        }
    }

    #[inline(always)]
    fn write_to_screen(&self, mem_addr: u32, screen_char: u16) {
        self.assert_vga_memory(mem_addr);
        // Safety: It shouldn never reach an invalid memory. Protected
        // by the conditionals above
        unsafe {
            core::ptr::write_volatile(mem_addr as *mut u16, screen_char);
        }
    }

    #[inline(always)]
    fn read_screen_char_from_screen(&self, mem_addr: u32) -> u16 {
        self.assert_vga_memory(mem_addr);
        // Safety: It shouldn never reach an invalid memory. Protected
        // by the conditionals above
        unsafe { core::ptr::read_volatile(mem_addr as *const u16) }
    }

    #[inline(always)]
    fn assert_vga_memory(&self, mem_addr: u32) {
        if mem_addr < self.vga_mem_addr || mem_addr > (self.vga_mem_addr + MEM_OFFSET_MAX) {
            panic!("[VGA] SEGFAULT {:#x}", mem_addr);
        }
    }
}

// For local dummy testing.
pub fn test_print() {
    let vga_buffer = VGA_MEMORY_ADDR as *mut u8;

    unsafe {
        *vga_buffer.offset(0 as isize) = b'E';
        *vga_buffer.offset(1 as isize) = 0x4;
        *vga_buffer.offset(2 as isize) = b'F';
        *vga_buffer.offset(3 as isize) = 0x4;
    }
}

#[cfg(test)]
mod tests {
    use crate::vga::*;
    use std::boxed::Box;

    #[test]
    fn writes_a_red_char_at_the_bottom_of_the_screen() {
        CURRX.store(0, Ordering::SeqCst);
        let vga_mem = Box::leak(Box::new([0_u8; 4000]));
        let vga = VgaDriver::new_with_addr(Color::Red, (vga_mem as *mut u8) as u32);
        assert_eq!(vga.curr_x(), 0);

        vga.print("a");

        let first_char = 80 * 24 * 2;
        assert_eq!(vga.curr_x(), 1);
        assert_eq!(vga_mem[first_char], 97);
        assert_eq!(vga_mem[first_char + 1], Color::Red.into());
    }

    #[test]
    fn can_use_the_whole_screen() {
        CURRX.store(0, Ordering::SeqCst);
        let vga_mem = Box::leak(Box::new([0_u8; 4000]));
        let vga = VgaDriver::new_with_addr(Color::Red, (vga_mem as *mut u8) as u32);

        for _ in 0..4000 {
            vga.print("a");
        }

        // last line is always empty
        for i in 0..(4000 - 80 * 2) {
            assert_ne!(vga_mem[i], 0, "error in {}", i);
        }
    }

    #[test]
    fn println_goes_next_line() {
        CURRX.store(0, Ordering::SeqCst);
        let vga_mem = Box::leak(Box::new([0_u8; 4000]));
        let vga = VgaDriver::new_with_addr(Color::Red, (vga_mem as *mut u8) as u32);
        assert_eq!(vga.curr_x(), 0);

        vga.println("a");

        assert_eq!(vga.curr_x(), 0);

        let a_position = 80 * 23 * 2;

        assert_eq!(vga_mem[a_position], 97);
        assert_eq!(vga_mem[a_position + 1], Color::Red.into());
    }

    #[test]
    fn after_80_chars_goes_next_line() {
        CURRX.store(0, Ordering::SeqCst);
        let vga_mem = Box::leak(Box::new([0_u8; 4000]));
        let vga = VgaDriver::new_with_addr(Color::Red, (vga_mem as *mut u8) as u32);

        for _ in 0..80 {
            vga.print("a");
        }
        vga.print("b");

        assert_eq!(vga.curr_x(), 1);

        let a_position = 80 * 23 * 2;
        let b_position = 80 * 24 * 2;

        assert_eq!(vga_mem[a_position], 97);
        assert_eq!(vga_mem[a_position + 1], Color::Red.into());
        assert_eq!(vga_mem[b_position], 98);
        assert_eq!(vga_mem[b_position + 1], Color::Red.into());
    }

    #[test]
    #[should_panic]
    fn write_to_invalid_address_upper_bound_panics() {
        CURRX.store(0, Ordering::SeqCst);
        let vga_mem = Box::leak(Box::new([0_u8; 5000]));
        let mem_begining = (vga_mem as *mut u8) as u32;
        let vga = VgaDriver::new_with_addr(Color::Red, mem_begining);
        vga.write_to_screen(mem_begining + MEM_OFFSET_MAX + 1, 0);
    }

    #[test]
    #[should_panic]
    fn write_to_invalid_address_lower_bound_panics() {
        CURRX.store(0, Ordering::SeqCst);
        let vga_mem = Box::leak(Box::new([0_u8; 5000]));
        let mem_begining = (vga_mem as *mut u8) as u32;
        let vga = VgaDriver::new_with_addr(Color::Red, mem_begining);
        vga.write_to_screen(mem_begining - 1, 0);
    }

    // (1, 0) => VGA_ADDR + 2
    // (10, 0) => VGA_ADDR + 20
    // (0, 1) => VGA_ADDR + 81
    // (80, 1) => VGA_ADDR + 81
    #[test]
    fn test_linear_addresing() {
        let vga_mem = Box::leak(Box::new([0_u8; 4000]));
        let mem_beginning = (vga_mem as *mut u8) as u32;
        let vga = VgaDriver::new_with_addr(Color::Red, mem_beginning);
        let addr = vga.mem_position_from_coords(0, 0);
        assert_eq!(addr, mem_beginning);

        let addr = vga.mem_position_from_coords(1, 0);
        assert_eq!(addr, mem_beginning + 2);

        let addr = vga.mem_position_from_coords(0, 1);
        assert_eq!(addr, mem_beginning + 160);

        let addr = vga.mem_position_from_coords(1, 2);
        assert_eq!(addr, mem_beginning + 80 * 2 * 2 + 2);

        let addr = vga.mem_position_from_coords(WIDTH, HEIGHT - 1);
        assert_eq!(addr, mem_beginning + MEM_OFFSET_MAX);
    }
}
