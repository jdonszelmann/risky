
pub enum Csr {
    MStatus,
    MVendorID,
    MArchID,
    MTVal,
    Mepc,
    SATP,
    MCause,
}

extern {
    fn csrread_mstatus() -> usize;
    fn csrread_mvendorid() -> usize;
    fn csrread_marchid() -> usize;
    fn csrread_mcause() -> usize;
    fn csrread_mtval() -> usize;
    fn csrread_mepc() -> usize;
    fn csrread_satp() -> usize;
    fn csrwrite_satp(value: usize);
}

impl Csr {
    pub fn read(s: Self) -> usize {
        unsafe {
            match s {
                Csr::MStatus => csrread_mstatus(),
                Csr::MVendorID => csrread_mvendorid(),
                Csr::MTVal => csrread_mtval(),
                Csr::Mepc => csrread_mepc(),
                Csr::MArchID => csrread_marchid(),
                Csr::MCause => csrread_mcause(),
                Csr::SATP => csrread_satp(),
                _ => panic!("Tried to read from write only csr"),
            }
        }
    }

    pub fn write(s: Self, value: impl Into<usize>) {
        unsafe {
            match s {
                Csr::SATP => csrwrite_satp(value.into()),
                _ => panic!("Tried to write to read only csr"),
            }
        }
    }
}