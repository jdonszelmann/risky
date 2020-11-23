use crate::pmm::memorymap::{MemoryMapEntry, Marker, Access};


/// TODO: more entries (https://sifive.cdn.prismic.io/sifive%2F834354f0-08e6-423c-bf1f-0cb58ef14061_fu540-c000-v1.0.pdf#page=32&zoom=auto,-207,724)
pub const PHYSICAL_MEMORY_MAP: &[MemoryMapEntry] = &[
    MemoryMapEntry {
        start: 0x0000_0000 as *const u8,
        end: 0x0000_00FF as *const u8,
        marker: Marker::Reserved,
        access: Access::None,
        description: "",
    },
    MemoryMapEntry {
        start: 0x0000_0100 as *const u8,
        end: 0x0000_0FFF as *const u8,
        marker: Marker::None,
        access: Access::All,
        description: "DEBUG",
    },
    MemoryMapEntry {
        start: 0x0000_1000 as *const u8,
        end: 0x0000_1FFF as *const u8,
        marker: Marker::None,
        access: Access::ReadExec,
        description: "Mode Select",
    },
    MemoryMapEntry {
        start: 0x0000_2000 as *const u8,
        end: 0x0000_FFFF as *const u8,
        marker: Marker::Reserved,
        access: Access::None,
        description: "",
    },
    MemoryMapEntry {
        start: 0x0001_0000 as *const u8,
        end: 0x0001_7FFF as *const u8,
        marker: Marker::Rom,
        access: Access::ReadExec,
        description: "Mask ROM",
    },
    MemoryMapEntry {
        start: 0x0001_8000 as *const u8,
        end: 0x00FF_FFFF as *const u8,
        marker: Marker::Reserved,
        access: Access::None,
        description: "",
    },
    MemoryMapEntry {
        start: 0x0100_0000 as *const u8,
        end: 0x0100_1FFF as *const u8,
        marker: Marker::None,
        access: Access::All,
        description: "E51 DTIM (8 KiB)",
    },
    MemoryMapEntry {
        start: 0x0100_2000 as *const u8,
        end: 0x7FFF_FFFF as *const u8,
        marker: Marker::None,
        access: Access::None,
        description: "TODO: Marked as empty but contains unmapped sections",
    },

    MemoryMapEntry {
        start: 0x8000_0000 as *const u8,
        end: 0x1F_FFFF_FFFF as *const u8,
        marker: Marker::Ram,
        access: Access::All,
        description: "DDR Memory (126 GiB)",
    },
];

