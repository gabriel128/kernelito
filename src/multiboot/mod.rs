//          -------------------+
// 0       | flags             |    (required)
//         +-------------------+
// 4       | mem_lower         |    (present if flags[0] is set)
// 8       | mem_upper         |    (present if flags[0] is set)
//         +-------------------+
// 12      | boot_device       |    (present if flags[1] is set)
//         +-------------------+
// 16      | cmdline           |    (present if flags[2] is set)
//         +-------------------+
// 20      | mods_count        |    (present if flags[3] is set)
// 24      | mods_addr         |    (present if flags[3] is set)
//         +-------------------+
// 28 - 40 | syms              |    (present if flags[4] or
//         |                   |                flags[5] is set)
//         +-------------------+
// 44      | mmap_length       |    (present if flags[6] is set)
// 48      | mmap_addr         |    (present if flags[6] is set)
//         +-------------------+
// 52      | drives_length     |    (present if flags[7] is set)
// 56      | drives_addr       |    (present if flags[7] is set)
//         +-------------------+
// 60      | config_table      |    (present if flags[8] is set)
//         +-------------------+
// 64      | boot_loader_name  |    (present if flags[9] is set)
//         +-------------------+
// 68      | apm_table         |    (present if flags[10] is set)
//         +-------------------+
// 72      | vbe_control_info  |    (present if flags[11] is set)
// 76      | vbe_mode_info     |
// 80      | vbe_mode          |
// 82      | vbe_interface_seg |
// 84      | vbe_interface_off |
// 86      | vbe_interface_len |
//         +-------------------+
// 88      | framebuffer_addr  |    (present if flags[12] is set)
// 96      | framebuffer_pitch |
// 100     | framebuffer_width |
// 104     | framebuffer_height|
// 108     | framebuffer_bpp   |
// 109     | framebuffer_type  |
// 110-115 | color_info        |
//         +-------------------+

use core::fmt::Display;

/// Partially parsed
#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct BootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u8; 16],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name_addr: *const u8,
    pub apm_table: u32,
}

impl Display for BootInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let field_ptr = core::ptr::addr_of!(self.flags);
        let flags = unsafe { field_ptr.read_unaligned() };

        write!(
            f,
            "Multiboot flags {:b}, boot_name {}",
            flags,
            self.boot_loader_name()
        )
    }
}
impl BootInfo {
    #[cfg(test)]
    pub fn boot_loader_name(&self) -> &str {
        "test"
    }

    #[cfg(not(test))]
    pub fn boot_loader_name(&self) -> &str {
        let mut len = 0;

        unsafe {
            // Limit it to chars
            for i in 0..50 {
                match (*self.boot_loader_name_addr.byte_offset(i)).as_ascii() {
                    Some(a_char) if a_char.to_char() == '\0' => break,
                    Some(_) => len = len + 1,
                    None => break, // Not an ascii char so we break
                }
            }
            core::str::from_utf8(core::slice::from_raw_parts(self.boot_loader_name_addr, len))
                .unwrap_or("Dunno")
        }
    }
}
