use crate::pmm::page::PhysicalPage;
use core::ptr::NonNull;
use crate::println;
use crate::pmm::arch::get_physical_memory_map;
use crate::ds::range::Range;

pub mod page;
pub mod arch;
pub mod memorymap;


static mut GLOBAL_PMM: Pmm = Pmm {};

/// Gets the global pmm
/// panics when the pmm isn't yet initialized
pub fn pmm() -> &'static Pmm {
    unsafe {
        &GLOBAL_PMM
    }
}

/// All actual data for the PMM is stored in allocated memory.
/// Therefore there are very few fields inside the pmm describing it's state.
pub struct Pmm {}

impl Pmm {
    pub fn new(memory: Range, excludes: &[Range]) -> Self {
        Self::initialize([memory].iter().cloned(), excludes)
    }

    pub fn from_ram(excludes: &[Range]) -> Self {
        let mm = get_physical_memory_map();

        for i in mm.ram() {
            println!("{:?}", Into::<Range>::into(i));
        }

        Self::initialize(mm.ram().map(|i| i.into()), excludes)
    }

    pub fn initialize(mut memory: impl Iterator<Item=Range>, excludes: &[Range]) -> Self {
        if let Some(first) = memory.next() {


            for i in memory {

            }


            Self{}
        } else {
            /// Return some kind of error
            todo!();
        }
    }

    pub fn set_global(self) {
        unsafe {
            GLOBAL_PMM = self;
        }
    }

    pub fn allocate_page() -> &'static PhysicalPage {
        todo!()
    }

    pub fn free_page(page: PhysicalPage) {
        todo!()
    }
}

