use core::fmt;
use spin::Mutex;
use crate::serial::arch::sifive_unleashed::SiFiveUnleashedWriter;
pub use Serial::*;

pub(self) mod arch;
pub mod println;

pub(self) trait SerialWriter {
    unsafe fn write_u8(&self, c: u8);
}

#[derive(Copy, Clone,Debug)]
/// TODO: Add custom targets/features, and gate the enum variants behind this.
pub enum Serial {
    /// SiFive Unleashed boot serial driver
    SiFiveUnleashedBasic
}

/// TODO: when custom targets/features (see `Serial`) have been implemented, the default should automatically be correct for that target
impl Default for Serial {
    #[allow(dead_code)]
    fn default() -> Self {
        return SiFiveUnleashedBasic
    }
}

pub(super) static mut SERIAL_WRITER: Option<Mutex<Serial>> = None;

impl Serial {
    fn get_serial_writer(&self) -> impl SerialWriter {
        match self {
            Serial::SiFiveUnleashedBasic => SiFiveUnleashedWriter
        }
    }

    pub fn init(self) {
        unsafe {
            SERIAL_WRITER = Some(Mutex::new(self))
        }
    }
}

impl fmt::Write for Serial {
    /// This function can be called in any impl of fmt::Write. It can't be implemented here due to limitations of Rust
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                self.get_serial_writer().write_u8(byte);
            }
        }
        Ok(())
    }
}

pub fn default() -> Serial{
    Serial::default()
}

