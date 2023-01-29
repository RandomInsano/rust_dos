use core::arch::asm;
use core::cmp::min;
use core::fmt::{self, Write};

use super::error_code::ErrorCode;

const MAX_PRINT_STRING: usize = 256;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::dos::console::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = DosWriter {};
    writer.write_fmt(args).unwrap();
}

struct DosWriter;

impl Write for DosWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            printc(c);
        }
        Ok(())
    }
}

/// This wraps DOS' print function (0x90)
/// 
/// DOS's string writing expects the string to end with "$" instead of a null
/// charater. This function fails with InvalidFormat if the string doesn't end
/// in a dollar sign. Also, because the DOS interrupt requires the data segment
/// register ("DS", look up memory segmentation on x86) this function allocates
/// 255 bytes on the stack and is limited to that many characters in one run.
/// If there's a safe way to change DS at the rust level, please open a PR!
/// 
/// Note that while this won't crash, it does no checks that the string provided
/// is truely ascii and will happily dump whatever to the console
pub fn print(string: &str) -> Result<(), ErrorCode> {

    let mut buffer = [0; MAX_PRINT_STRING];
    let size = min(buffer.len(), string.len());

    buffer[0..size].copy_from_slice(string[0..size].as_bytes());

    if !string.ends_with("$") {
        return Err(ErrorCode::InvalidFormat)
    }

    unsafe {
        asm!(
            "int 0x21",
            in("ah") 0x09_u8,
            in("dx") buffer.as_ptr() as u16,
        )
    }

    Ok(())
}

pub fn printc(ch: u8) {
    unsafe {
        asm!(
            "int 0x21",
            in("ah") 0x02_u8,
            in("dl") ch
        ) 
    }
}
