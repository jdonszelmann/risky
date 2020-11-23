mod riscv64gc_generic;
mod sifive_unleashed;

use crate::pmm::memorymap::{MemoryMapEntry, MemoryMap};

pub const fn get_physical_memory_map() -> MemoryMap {
    MemoryMap{
        entries: &sifive_unleashed::PHYSICAL_MEMORY_MAP
    }
}

#[cfg(test)]
mod tests {
    use crate::pmm::arch::get_physical_memory_map;

    #[test_case]
    pub fn test_memory_map() {
        let mm = get_physical_memory_map();
        let mut last = &mm[0];

        // Each memory map should start at address 0.
        // This entry may be marked as reserved, but should at least be there.
        assert_eq!(last.start as usize, 0);

        for i in &mm[1..] {
            // Each next entry's start must be directly after the previous end
            assert_eq!(i.start as usize, last.end as usize+1);
            last = i;
        }
    }
}