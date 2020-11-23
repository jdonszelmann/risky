use crate::println;
use crate::arch::{Registers, kernel_exit};
use crate::serial::SERIAL_WRITER;
// Csr is allowed here because sifive_unleashed is a riscv64gc architecture
use crate::arch::riscv64gc_generic::csr::Csr;
// MCause is allowed here because sifive_unleashed is a riscv64gc architecture
use crate::arch::riscv64gc_generic::mcause::MCause;

static mut EXCEPTION_COUNT: usize = 0;
const MAX_EXCEPTIONS: usize = 10;

#[no_mangle]
pub extern "C" fn sifive_unleashed_core_local_interrupt_handler(regs: Registers) -> () {
    let cause: MCause = Csr::read(Csr::MCause).into();
    let mtval: usize = Csr::read(Csr::MTVal);
    let mepc: usize = Csr::read(Csr::Mepc);

    // force unlock the stdout mutex so this function even works when an exception occurs
    // *WHILE* stdout is locked
    unsafe {
        if let Some(ref s) = SERIAL_WRITER {
            s.force_unlock()
        }
    }

    let count = unsafe {
        EXCEPTION_COUNT += 1;
        EXCEPTION_COUNT
    };

    if count > MAX_EXCEPTIONS {
        kernel_exit();
    }

    println!("Exception occured ({})", count);
    println!("MCause: {}", cause);
    println!("MTVal: {}", mtval);
    println!("Mepc: {}", mepc);

    println!("{:?}", regs);
}