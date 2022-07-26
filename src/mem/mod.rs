#![allow(dead_code)]
use core::arch::asm;

mod frame_allocator;
mod paging;
mod sync;

pub(crate) const PAGE_SIZE_BYTES: u32 = 4096;
pub(crate) const END_OF_MEMORY: u32 = 0xFFFFFFFF;
pub(crate) const NUM_OF_DYNAMIC_MEM_PAGES: u32 =
    (END_OF_MEMORY - FRAME_ALLOC_MEMORY_INIT) / PAGE_SIZE_BYTES;
pub(crate) const FRAME_ALLOC_MEMORY_INIT: u32 = 0x200000;

pub fn init() -> crate::Result<()> {
    paging::load_kernel_directory()?;
    paging::enable_paging()?;

    Ok(())
}

pub fn page_faulted_addr() -> u32 {
    let x: u32;
    unsafe {
        asm!("mov {}, cr2", out(reg) x);
    }
    x
}
