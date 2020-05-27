use core::fmt;
use crate::serial::SERIAL_WRITER;
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::serial::println::_print(format_args!($($arg)*));
    });
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        if let Some(ref s) = SERIAL_WRITER {
            s.lock().write_fmt(args).unwrap()
        };
    }
}