extern crate static_assertions as sa;


/// size of pages in bytes
pub const PHYSICAL_PAGE_SIZE: usize = 4096;


#[repr(C, packed)]
pub struct PhysicalPage {
    page: [u8; PHYSICAL_PAGE_SIZE]
}

pub struct PhysicalPageInfo {
    location: &'static PhysicalPage,

    // make sure that on a 32 bit machine, location still takes up 64 bits
    #[cfg(target_pointer_width = "32")]
    __reserved_1: u32,

    next: &'static PhysicalPageInfo,

    // make sure that on a 32 bit machine, next still takes up 64 bits
    #[cfg(target_pointer_width = "32")]
    __reserved_2: u32,

    prev: &'static PhysicalPageInfo,

    // make sure that on a 32 bit machine, prev still takes up 64 bits
    #[cfg(target_pointer_width = "32")]
    __reserved_3: u32,

    page_type: PageType
}

#[repr(u8)]
enum PageType {
    Page,
    Info
}

sa::assert_eq_size!(PhysicalPageInfo, [u64; 4]);