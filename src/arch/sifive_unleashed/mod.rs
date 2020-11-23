///! TODO: may be moved to riscv64_generic

use crate::kernel_main;
use crate::ds::range::Range;

extern {
    fn wfi() -> !;
}

#[no_mangle]
pub extern "C" fn sifive_unleashed_kernel_main(start: usize, end: usize) -> !{
    kernel_main(Range::new(start, end));
    kernel_exit()
}

pub fn kernel_exit() -> ! {
    unsafe {
        wfi()
    }
}