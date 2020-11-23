


pub struct SATP {
    bits: usize,
}

impl From<usize> for SATP {
    fn from(u: usize) -> Self {
        Self {
            bits: u,
        }
    }
}

impl Into<usize> for SATP {
    fn into(self) -> usize {
        self.bits
    }
}
