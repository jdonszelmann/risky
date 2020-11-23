use static_assertions::_core::fmt::{Display, Formatter};
use core::fmt;

pub mod arch;

pub enum AnyValue {
    String(&'static str),
    Int(usize)
}

impl Display for AnyValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AnyValue::String(v) => write!(f, "{}", v),
            AnyValue::Int(v) => write!(f, "{}", v),
        }
    }
}

pub struct SystemInfo {
    pub(self) vendor: AnyValue,
    pub(self) architecture: AnyValue,
}

impl SystemInfo {
    pub fn get() -> Self {
        arch::get_systeminfo()
    }
}

impl Display for SystemInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vendor (id): {}", self.vendor)?;
        writeln!(f, "Architecture (id): {}", self.architecture)
    }
}
