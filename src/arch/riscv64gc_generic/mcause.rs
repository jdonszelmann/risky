use core::{fmt, mem};
use core::fmt::{Display, Formatter};

#[derive(Debug)]
/// Reference: https://content.riscv.org/wp-content/uploads/2019/08/riscv-privileged-20190608-1.pdf#page=49&zoom=auto,-207,767
pub enum CauseType {
    Reserved,

    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,

    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,

    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,

    /// If this interrupt is triggered, a board-specific interrupt occurred
    Platform,

    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,

    BreakPoint,

    LoadAddressMisaligned,
    LoadAccessFault,

    StoreAddressMisaligned,
    StoreAccessFault,

    EnvironmentCallU,
    EnvironmentCallS,
    EnvironmentCallM,

    InstructionPageFault,
    LoadPageFault,
    StorePageFault,

    Custom
}

impl CauseType {
    pub fn new(code: usize, interrupt: bool) -> Self {
        match (interrupt, code) {
            (false, 0) => CauseType::InstructionAddressMisaligned,
            (false, 1) => CauseType::InstructionAccessFault,

            (false, 2) => CauseType::IllegalInstruction,

            (false, 3) => CauseType::BreakPoint,

            (false, 4) => CauseType::LoadAddressMisaligned,
            (false, 5) => CauseType::LoadAccessFault,

            (false, 6) => CauseType::StoreAddressMisaligned,
            (false, 7) => CauseType::StoreAccessFault,

            (false, 8) => CauseType::EnvironmentCallU,
            (false, 9) => CauseType::EnvironmentCallS,
            (false, 10) => CauseType::Reserved,
            (false, 11) => CauseType::EnvironmentCallM,

            (false, 12) => CauseType::InstructionPageFault,
            (false, 13) => CauseType::LoadPageFault,
            (false, 14) => CauseType::Reserved,
            (false, 15) => CauseType::StorePageFault,

            (false, n) if n >= 16 && n <=23 => CauseType::Reserved,
            (false, n) if n >= 24 && n <=31 => CauseType::Custom,
            (false, n) if n >= 32 && n <=47 => CauseType::Reserved,
            (false, n) if n >= 48 && n <=63 => CauseType::Custom,
            (false, n) if n >= 64 => CauseType::Reserved,

            (true, 0) => CauseType::UserSoftwareInterrupt,
            (true, 1) => CauseType::SupervisorSoftwareInterrupt,
            (true, 2) => CauseType::Reserved,
            (true, 3) => CauseType::MachineSoftwareInterrupt,

            (true, 4) => CauseType::UserTimerInterrupt,
            (true, 5) => CauseType::SupervisorTimerInterrupt,
            (true, 6) => CauseType::Reserved,
            (true, 7) => CauseType::MachineTimerInterrupt,

            (true, 8) => CauseType::UserExternalInterrupt,
            (true, 9) => CauseType::SupervisorExternalInterrupt,
            (true, 10) => CauseType::Reserved,
            (true, 11) => CauseType::MachineExternalInterrupt,

            (false, n) if n >= 12 && n <=15 => CauseType::Reserved,
            (false, n) if n >= 16 => CauseType::Platform,

            _ => CauseType::Reserved,
        }
    }
}

pub struct MCause {
    bits: usize
}

impl MCause {
    pub fn interrupt(&self) -> bool {
        self.bits & (1 << (mem::size_of::<usize>()-1)) > 1
    }

    pub fn exception(&self) -> bool {
        !self.interrupt()
    }

    pub fn code(&self) -> usize {
        self.bits & (1 << (mem::size_of::<usize>()-1)-1)
    }

    pub fn causetype(&self) -> CauseType {
        CauseType::new(self.code(), self.interrupt())
    }
}

impl From<usize> for MCause {
    fn from(u: usize) -> Self {
        Self {
            bits: u
        }
    }
}

impl Display for MCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "
MCause: {}
    interrupt: {}
    code: {:?} ({})
    for more info: https://content.riscv.org/wp-content/uploads/2019/08/riscv-privileged-20190608-1.pdf#page=49&zoom=auto,-207,748
        ", self.bits, self.interrupt(), self.causetype(), self.code())
    }
}