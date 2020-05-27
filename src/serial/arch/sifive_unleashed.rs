//! TODO: Refactor to be swappable for a second serial printer with support for alloc and format! etc

use volatile::Volatile;
use crate::serial::SerialWriter;

/// The Uart struct is the layout of the uart control structure in memory,
/// according to the SiFive manual found here:
/// https://static.dev.sifive.com/FU540-C000-v1.0.pdf#page=86&zoom=auto,-267,54
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct Uart {
    txdata: u32,
    rxdata: u32,

    // TODO: make bit field struct?
    txctl: u32,

    // TODO: make bit field struct?
    rxctl: u32,

    interrupt_enable: u32,
    interrupt_pending: u32,
    divisor: u32,
}

const UART0: *mut Volatile<Uart> = 0x10010000 as *mut Uart as *mut Volatile<Uart>;
#[allow(dead_code)]
const UART1: *mut Volatile<Uart> = 0x10011000 as *mut Uart as *mut Volatile<Uart>;

unsafe fn write_u8(c: u8) {
    // manual spinlock (wait for uart ready)
    while (*UART0).read().txdata & 0x8000000 != 0 {}

    (*UART0).update(|v| {
        v.txdata |= c as u32;
    })
}


pub struct SiFiveUnleashedWriter;

impl SerialWriter for SiFiveUnleashedWriter {
    unsafe fn write_u8(&self, c: u8) {
        write_u8(c)
    }
}