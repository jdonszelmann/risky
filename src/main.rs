#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod std;
mod serial;
mod interrupt;

use core::panic::PanicInfo;

extern {
    fn wfi() -> !;
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    #[cfg(test)]
    println!("Test failed");

    println!("Panic");

    unsafe {
        wfi()
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    serial::default().init();


    #[cfg(test)]
    {
        test_main();
    }
    #[cfg(not(test))]
    {
        println!("Hello world!");
    }


    unsafe {
        wfi()
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