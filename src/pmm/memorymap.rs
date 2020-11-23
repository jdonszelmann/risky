use core::slice;
use bitflags::bitflags;
use core::ops::{Index, Deref};
use crate::ds::range::Range;

pub struct MemoryMap {
    pub entries: &'static [MemoryMapEntry]
}

impl Deref for MemoryMap {
    type Target = &'static [MemoryMapEntry];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl MemoryMap {
    pub const fn new(entries: &'static [MemoryMapEntry]) -> Self {
        Self {
            entries
        }
    }

    /// Returns an iterator over all areas in the memory map with the marker [ram](Marker) which are
    /// readable, writable and executable. This means
    pub fn ram(&self) -> impl Iterator<Item=&MemoryMapEntry> {
        self.entries.iter()
            .filter(|i| {
                i.marker == Marker::Ram &&
                    i.access.contains(Access::Read | Access::Write | Access::Execute)
            })
    }
}

#[derive(Debug)]
pub struct MemoryMapEntry {
    pub start: *const u8,
    pub end: *const u8,
    pub marker: Marker,
    pub access: Access,
    pub description: &'static str,
}

impl MemoryMapEntry {
    pub fn memory(&self) -> &[u8]{
        unsafe {
            slice::from_raw_parts(self.start, (self.end as usize)-(self.start as usize))
        }
    }
}

impl Into<Range> for &MemoryMapEntry {
    fn into(self) -> Range {
        Range::new(self.start as usize, self.end as usize)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Marker {
    /// Should be used for sections that don't have a common marker
    None,
    /// Should be used for sections that are general purpose RAM, and can be indexed by the pmm.
    /// This should only be used on sections with access of at least [R+W+X](Access)
    Ram,
    /// Should be used for sections that contain read only data, for example boot code rom
    Rom,
    /// Should be used for unusable sections in the address space.
    Reserved
}

bitflags! {
    pub struct Access: u8 {
        /// Not accessible in any way
        const None        = 0b00000;
        /// Readable
        const Read        = 0b00001;
        /// Writable
        const Write       = 0b00010;
        /// Executable
        const Execute     = 0b00100;
        /// Supports atomic memory access
        const Atomic      = 0b01000;
        /// Memory is cachable (usually off)
        const Cachable    = 0b10000;

        /// RWX+Atomic
        const All = Self::Read.bits | Self::Write.bits | Self::Execute.bits | Self::Atomic.bits;
        /// RW+Atomic
        const NoExec = Self::Read.bits | Self::Write.bits | Self::Atomic.bits;
        /// RW
        const ReadWrite = Self::Read.bits | Self::Write.bits;
        /// RX
        const ReadExec = Self::Read.bits | Self::Execute.bits;
    }
}