static VGA_MEMORY: u32 = 0xb8000;

#[no_mangle]
pub fn print(
    text: &[u8],
    a: Option<&mut u8>,
    a1: &mut u8,
    a2: &mut u8,
    a3: &mut u8,
    a4: &mut u8,
    a5: &mut u8,
) {
    *a.unwrap() += 1;
    *a1 += 1;
    // Force it to use the stack
    // *a2 += 1;
    // *a3 += 1;
    // *a4 += 1;
    // *a5 += 1;
    let vga_buffer = VGA_MEMORY as *mut u8;

    // unsafe {
    //     *vga_buffer.offset(0 as isize) = *text.get(0).unwrap();
    //     *vga_buffer.offset(1 as isize) = 0xb;
    // }

    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
        }
    }

    // unsafe {
    //     *vga_buffer.offset(2 as isize) = *text.get(0).unwrap();
    //     *vga_buffer.offset(3 as isize) = 0xb;
    // }
}
