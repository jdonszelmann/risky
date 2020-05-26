#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

mod std;
mod serial;

use core::panic::PanicInfo;

extern {
    fn wfi() -> !;
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe {
        wfi()
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {

    println!("Hello world!");

    unsafe {
        wfi()
    }
}

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}


// #[lang = "stack_exhausted"] extern fn stack_exhausted() {}
// #[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }