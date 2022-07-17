#![allow(dead_code)]
use core::arch::asm;

mod attr {
    // If the bit is set, the page will not be cached. Otherwise, it will be.
    pub const CACHE_DISABLED: u32 = 0b0001_0000;
    // If the bit is set, write-through caching is enabled. If not, then write-back is enabled instead.
    pub const WRITE_THROUGH: u32 = 0b0000_1000;
    // If the bit is set, then the page may be accessed by all
    pub const USER_ACCESS_ENABLED: u32 = 0b0000_0100;
    // Read/Write' permissions flag. If the bit is set, the page is read/write.
    // Otherwise when it is not set, the page is read-only.
    pub const WRITABLE: u32 = 0b0000_0010;
    // If the bit is set, the page is actually in physical memory at the moment
    pub const PRESENT: u32 = 0b0000_0001;
}

const PAGE_SIZE: u32 = 4096;
const PAGE_ENTRIES_QTY: usize = 1024;
const KERNEL_PAGE_DIRECTORY_ADDR: u32 = 0x100000;
const KERNEL_CODE_ADDR: u32 = 0x101000;

#[repr(transparent)]
struct PageDirectory {
    entries: [PageDirectoryEntry; PAGE_ENTRIES_QTY],
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct PageDirectoryEntry(u32);

#[repr(transparent)]
struct PageTable {
    entries: [PageTableEntry; PAGE_ENTRIES_QTY],
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct PageTableEntry(u32);

impl Default for PageDirectoryEntry {
    fn default() -> Self {
        Self(attr::WRITABLE)
    }
}

impl Default for PageTableEntry {
    fn default() -> Self {
        Self(attr::WRITABLE | attr::USER_ACCESS_ENABLED | attr::PRESENT)
    }
}
impl PageDirectoryEntry {
    fn new_with_flags(addr: u32, flags: u32) -> Self {
        if (addr % PAGE_SIZE) != 0 {
            panic!("PDE addres is not 4KB aligned, addr: {}", addr);
        }

        Self(addr | flags)
    }
}
impl PageTableEntry {
    fn new_with_flags(addr: u32, flags: u32) -> Self {
        if (addr % PAGE_SIZE) != 0 {
            panic!("Page table entry addres is not 4KB aligned, addr: {}", addr);
        }

        Self(addr | flags)
    }
}

impl PageDirectory {
    fn new_for_kernel() -> Self {
        let mut entries = [PageDirectoryEntry::default(); PAGE_ENTRIES_QTY];
        let flags = attr::WRITABLE | attr::USER_ACCESS_ENABLED | attr::PRESENT;

        // let kernel_code_addr = (&KERNEL_CODE_PAGE_TABLE as *const _) as u32;

        // kprintln!(
        //     "Leading PDE code addr at {} with flags, {:08b}",
        //     &kernel_code_addr,
        //     flags
        // );

        entries[0] = PageDirectoryEntry::new_with_flags(KERNEL_CODE_ADDR, flags);

        Self { entries }
    }

    // pub fn load_to_cr3(&'static self) {
    //     let directory_addr = self as *const _ as u32;

    //     kprintln!("Leading Directory at {:p}", &directory_addr);

    //     unsafe {
    //         asm!("mov cr3, {}", in(reg) &directory_addr);
    //     }
    // }

    pub fn load_to_cr3() {
        let directory_addr = KERNEL_PAGE_DIRECTORY_ADDR;

        // kprintln!("Leading Directory at {:p}", &directory_addr);

        unsafe {
            asm!("mov cr3, {}", in(reg) directory_addr);
        }
    }

    // fn add_page_table(&mut self, index: usize, page_table: PageTable) -> Result<(), KernelError> {
    //     if index >= PAGE_ENTRIES_QTY {
    //         return Err(KernelError::new(KernelError::PagingError));
    //     }

    //     self.entries[index]
    // }
}

impl PageTable {
    fn new_for_kernel() -> Self {
        let mut entries = [PageTableEntry::default(); PAGE_ENTRIES_QTY];
        // identity mapped
        for (i, entry) in entries.iter_mut().enumerate() {
            let flags = attr::WRITABLE | attr::USER_ACCESS_ENABLED | attr::PRESENT;
            *entry = PageTableEntry::new_with_flags((i as u32) * PAGE_SIZE, flags);
        }

        Self { entries }
    }
}

pub fn load_kernel_directory() {
    // KERNEL_PAGE_DIRECTORY.load_to_cr3();
    let pd_addr = KERNEL_PAGE_DIRECTORY_ADDR as *mut PageDirectory;
    let pt_addr = KERNEL_CODE_ADDR as *mut PageTable;
    unsafe {
        *pd_addr = PageDirectory::new_for_kernel();
        *pt_addr = PageTable::new_for_kernel();
        PageDirectory::load_to_cr3();
        // let a: u32;
        // asm!("mov {}, cr3", out(reg) a);
        // kprintln!("CR3 points at {:x}", a);
    }
}

pub fn enable_paging() {
    unsafe {
        asm!("mov eax, cr0", "or eax, 0x80000000", "mov cr0, eax");
    }
}
