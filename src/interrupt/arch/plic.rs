extern crate static_assertions as sa;

/// The priority struct represents a pri
#[repr(C, packed)]
pub struct Priority {
    priority: u8,
    _reserved_1: u8,
    _reserved_2: u8,
    _reserved_3: u8,
}

sa::assert_eq_size!(Priority, [u8; 4]);

impl Priority {
    /// set_priority sets the priority of an interrupt. Priorities must be between 0 and 7,
    /// and this function panics if this is not the case.
    pub fn set_priority(&mut self, priority: u8) {
        if priority > 7 {
            panic!("Plic interrupt priority must be between 0 and 7 but was {}", priority)
        }
        self.priority = priority
    }

    /// get_priority returns the current priority of this interrupt
    pub fn get_priority(&self) -> u8 {
        self.priority
    }
}

/// Pending is a bifield, where each bit from 0 to 53 corresponds to an interrupt that is pending.
/// All bits above 53 are unused.

/// HartMModeInterruptEnable is a bifield, where each bit from 0 to 53 corresponds to an interrupt that is enabled.
/// All bits above 53 are unused.
#[repr(C, packed)]
pub struct HartInterruptBitfield {
    pending: u64,
}

sa::assert_eq_size!(HartInterruptBitfield, [u8; 8]);

#[repr(C, packed)]
pub struct Plic {
    // 0x0000
    /// A list of priorities for each interrupt. The 0th index is reserved and should never be written to.
    pub priorities: [Priority; 54],
    // skip to 0x1000
    _reserved_2: [u8; 0xf28],

    // 0x1000
    pub pending: HartInterruptBitfield,
    // skip to 0x2000
    _reserved_3: [u8; 0xff8],

    // 0x2000
    pub(self) hart0_enable_m: HartInterruptBitfield,
    _reserved_4: [u8; 0x78],

    // 0x2080
    pub(self) hart1_enable_m: HartInterruptBitfield,
    _reserved_5: [u8; 0x78],

    // 0x2100
    pub(self) hart1_enable_s: HartInterruptBitfield,
    _reserved_6: [u8; 0x78],

    // 0x2180
    pub(self) hart2_enable_m: HartInterruptBitfield,
    _reserved_7: [u8; 0x78],

    // 0x2200
    pub(self) hart2_enable_s: HartInterruptBitfield,
    _reserved_8: [u8; 0x78],

    // 0x2280
    pub(self) hart3_enable_m: HartInterruptBitfield,
    _reserved_9: [u8; 0x78],

    // 0x2300
    pub(self) hart3_enable_s: HartInterruptBitfield,
    _reserved_10: [u8; 0x78],

    // 0x2380
    pub(self) hart4_enable_m: HartInterruptBitfield,
    _reserved_11: [u8; 0x78],

    // 0x2400
    pub(self) hart4_enable_s: HartInterruptBitfield,
    _reserved_12: [u8; 0x1fdbf8],
    // 0x20_0000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart0_priority_threshold_m: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart0_claim_m: u32,
    _reserved_13: [u8; 0xff8],

    // 0x20_1000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart1_priority_threshold_m: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart1_claim_m: u32,
    _reserved_14: [u8; 0xff8],

    // 0x20_2000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart1_priority_threshold_s: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart1_claim_s: u32,
    _reserved_15: [u8; 0xff8],

    // 0x20_3000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart2_priority_threshold_m: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart2_claim_m: u32,
    _reserved_16: [u8; 0xff8],

    // 0x204000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart2_priority_threshold_s: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart2_claim_s: u32,
    _reserved_17: [u8; 0xff8],

    // 0x205000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart3_priority_threshold_m: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    hart3_claim_m: u32,
    _reserved_18: [u8; 0xff8],

    // 0x206000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart3_priority_threshold_s: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart3_claim_s: u32,
    _reserved_19: [u8; 0xff8],

    // 0x207000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    hart4_priority_threshold_m: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart4_claim_m: u32,
    _reserved_20: [u8; 0xff8],

    // 0x208000

    /// All interrupts with priority below this number will be masked.
    /// Setting this to 0 means all interrupts are enabled
    pub(self) hart4_priority_threshold_s: Priority,
    /// Claims contain the id of the highest pending interrupt, or 0 if no interrupt is pending.
    /// Writing the id of a handled interrupt here completes this interrupt. A new interrupt may
    /// become pending now
    pub(self) hart4_claim_s: u32,
    _reserved_21: [u8; 0xff8],

    // 0x209000

    _reserved_22: [u8; 0x3df7000],
}

// Assert that the plic is 64 kilobytes large.
sa::assert_eq_size!(Plic, [u8; 64 * 1024 * 1024]);

#[cfg(test)]
mod tests {
    use crate::interrupt::arch::plic::Plic;

    #[test_case]
    pub fn test_plic() {
        use crate::println;
        pub const PLIC_TEST: *mut Plic = 0x0C00_0000 as *mut Plic;


        assert_eq!(unsafe{&(*PLIC_TEST).priorities} as *const _ as usize,       PLIC_TEST as usize + 0x0000_0000);

        assert_eq!(unsafe{&(*PLIC_TEST).pending} as *const _ as usize,          PLIC_TEST as usize + 0x0000_1000);

        assert_eq!(unsafe{&(*PLIC_TEST).hart0_enable_m} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2000);

        assert_eq!(unsafe{&(*PLIC_TEST).hart1_enable_m} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2080);
        assert_eq!(unsafe{&(*PLIC_TEST).hart1_enable_s} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2100);

        assert_eq!(unsafe{&(*PLIC_TEST).hart2_enable_m} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2180);
        assert_eq!(unsafe{&(*PLIC_TEST).hart2_enable_s} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2200);

        assert_eq!(unsafe{&(*PLIC_TEST).hart3_enable_m} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2280);
        assert_eq!(unsafe{&(*PLIC_TEST).hart3_enable_s} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2300);

        assert_eq!(unsafe{&(*PLIC_TEST).hart4_enable_m} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2380);
        assert_eq!(unsafe{&(*PLIC_TEST).hart4_enable_s} as *const _ as usize,   PLIC_TEST as usize + 0x0000_2400);

        assert_eq!(unsafe{&(*PLIC_TEST).hart0_priority_threshold_m} as *const _ as usize,   PLIC_TEST as usize + 0x0020_0000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart0_claim_m} as *const _ as usize,                PLIC_TEST as usize + 0x0020_0004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart1_priority_threshold_m} as *const _ as usize,   PLIC_TEST as usize + 0x0020_1000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart1_claim_m} as *const _ as usize,                PLIC_TEST as usize + 0x0020_1004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart1_priority_threshold_s} as *const _ as usize,   PLIC_TEST as usize + 0x0020_2000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart1_claim_s} as *const _ as usize,                PLIC_TEST as usize + 0x0020_2004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart2_priority_threshold_m} as *const _ as usize,   PLIC_TEST as usize + 0x0020_3000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart2_claim_m} as *const _ as usize,                PLIC_TEST as usize + 0x0020_3004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart2_priority_threshold_s} as *const _ as usize,   PLIC_TEST as usize + 0x0020_4000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart2_claim_s} as *const _ as usize,                PLIC_TEST as usize + 0x0020_4004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart3_priority_threshold_m} as *const _ as usize,   PLIC_TEST as usize + 0x0020_5000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart3_claim_m} as *const _ as usize,                PLIC_TEST as usize + 0x0020_5004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart3_priority_threshold_s} as *const _ as usize,   PLIC_TEST as usize + 0x0020_6000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart3_claim_s} as *const _ as usize,                PLIC_TEST as usize + 0x0020_6004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart4_priority_threshold_m} as *const _ as usize,   PLIC_TEST as usize + 0x0020_7000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart4_claim_m} as *const _ as usize,                PLIC_TEST as usize + 0x0020_7004);

        assert_eq!(unsafe{&(*PLIC_TEST).hart4_priority_threshold_s} as *const _ as usize,   PLIC_TEST as usize + 0x0020_8000);
        assert_eq!(unsafe{&(*PLIC_TEST).hart4_claim_s} as *const _ as usize,                PLIC_TEST as usize + 0x0020_8004);

        // println!("{:x}", unsafe{&(*PLIC_TEST).hart0_priority_threshold_m} as *const _ as usize - PLIC_TEST as usize);
    }
}