pub mod sifive_unleashed;
pub mod riscv64gc_generic;


/// `Registers` defines a struct with all the registers of the machine
#[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
pub use riscv64gc_generic::registers::Registers;

/// `kernel_exit` defines a function to be called when the kernel
/// initialization code has finished. Usually this function will infinitely
/// wait for interrupts.
#[cfg(all(target_arch = "riscv64", target_pointer_width = "64"))]
pub use sifive_unleashed::kernel_exit;