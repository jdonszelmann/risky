#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![feature(const_fn)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod std;
mod serial;
mod interrupt;
mod arch;
mod pmm;
mod system_info;
mod ds;

use core::panic::PanicInfo;
use crate::system_info::SystemInfo;
use crate::arch::kernel_exit;
use crate::ds::range::Range;


#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    #[cfg(test)]
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {:?}", s);
    } else {
        println!("panic occurred");
    }

    println!("Panic");

    kernel_exit()
}

/// Kernel main takes one parameter, a range of addresses in which kernel code and data is stored.
fn kernel_main(kernel: Range) -> () {
    serial::default().init();
    println!("{}", SystemInfo::get());

    // pmm::Pmm::from_ram(&[kernel]).set_global();

    #[cfg(test)]
    {
        test_main();
    }
    #[cfg(not(test))]
    {
        println!("Hello world!");
    }
}

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for (index, test) in tests.iter().enumerate() {
        test();
        println!("Test {} passed", index + 1);
    }

    println!("Tests complete");
}


// #[lang = "stack_exhausted"] extern fn stack_exhausted() {}
// #[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }