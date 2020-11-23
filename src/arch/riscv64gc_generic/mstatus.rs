use core::fmt;
use core::fmt::{Display, Formatter};

struct MStatus {
    bits: usize
}

impl From<usize> for MStatus {
    fn from(u: usize) -> Self {
        Self {
            bits: u
        }
    }
}

impl Display for MStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "
MStatus: {}

        ", self.bits)
    }
}