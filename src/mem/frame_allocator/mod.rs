/// Allocates 4KB frames for being used by paging
#[derive(Debug)]
struct FrameAllocator<const PAGES_QTY: usize> {
    frames_used_bitmap: [u8; PAGES_QTY],
    highest_used_frame_index: Option<usize>,
    mem_init: u32,
}

impl<const PAGES_QTY: usize> FrameAllocator<PAGES_QTY> {
    fn new(mem_init: u32) -> Self {
        Self {
            frames_used_bitmap: [0; PAGES_QTY],
            highest_used_frame_index: None,
            mem_init,
        }
    }

    fn get_4kb_frame(&mut self) -> Option<u32> {
        if self.highest_used_frame_index.is_some()
            && self.highest_used_frame_index.unwrap() >= self.frames_used_bitmap.len()
        {
            return None;
        }

        let next_index = self.highest_used_frame_index.map(|x| x + 1).unwrap_or(0);

        self.frames_used_bitmap[next_index];

        let mem_addr = self.mem_from_index(next_index);

        // TODO: Check for alignment
        Some(mem_addr)
    }

    fn mem_from_index(&self, index: usize) -> u32 {
        self.mem_init + (crate::mem::PAGE_SIZE_BYTES * (index as u32))
    }
}

#[cfg(test)]
mod tests {
    use crate::mem;

    use super::FrameAllocator;

    #[test]
    fn frame_alloc_size_should_be_less_than_2mb() {
        let size =
            core::mem::size_of::<FrameAllocator<{ mem::NUM_OF_DYNAMIC_MEM_PAGES as usize }>>();
        println!("Frame Alloc mem size bytes {}", size);
        assert!(
            size < (2 * 10_usize.pow(6)),
            "Should be less than {}",
            2 * 10_usize.pow(6)
        )
    }
}
