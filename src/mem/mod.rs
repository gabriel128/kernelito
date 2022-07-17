mod paging;

pub fn init() -> crate::Result<()> {
    paging::load_kernel_directory();
    paging::enable_paging();

    Ok(())
}
