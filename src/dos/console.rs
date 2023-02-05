use core::arch::asm;
use core::fmt::{self, Write};

use alloc::string::String;
use alloc::vec::Vec;

use super::error_code::ErrorCode;
use super::misc::ptr_to_segments;

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
/// charater. There is no way to print a $ from official documentation. This
/// function fails with InvalidFormat if the string doesn't contain a dollar
/// sign.
/// 
/// Note that while this won't crash, it does no checks that the string provided
/// is truely ascii and will happily dump whatever to the console
pub fn print(string: &str) -> Result<(), ErrorCode> {
    // TODO: Learn how to do this at compile time with macros instead?
    if !string.contains("$") {
        return Err(ErrorCode::InvalidFormat)
    }

    let (segment, offset) = ptr_to_segments(string.as_ptr() as u32);

    unsafe {
        asm!(
            "mov ax, ds",
            "push ax",          // Preserve data segment register
            "add ax, cx",       // Offset the segment. I have no idea why
            "mov ds, ax",

            "mov ah, 0x09",     
            "int 0x21",         // Call interrupt handler

            "pop ax",           // Restore data segment register
            "mov ds, ax",

            in("cx") segment,
            in("dx") offset,
        )
    }

    Ok(())
}

pub fn printc(character: u8) {
    unsafe {
        asm!(
            "int 0x21",
            in("ah") 0x02_u8,
            in("dl") character
        ) 
    }
}

/// Read a byte from the keyboard (0x01)
/// 
/// ```
/// let mut result;
/// println!("Echo out all text as uppercase");
/// println!("=================================================")
/// println!("Type something, press 'enter' to exit");
/// println!("");
/// loop {
///     result = readc();
///
///     if result == b'\r' {
///         println!("");
///         println!("Have a good day!");
///         break;
///     } else if result >= b'a' && result <= b'z'  {
///         printc(result + b'A' - b'a');
///     } else {
///         printc(result);
///     }
/// }
/// ```
pub fn readc() -> u8 {
    let character: u8;

    unsafe {
        asm!(
            "mov ah, 0x01",
            "int 0x21",
            out("al") character
        )
    }

    return character
}

/// Read a string from the user
/// 
/// Allows for the user to edit input. From testin in DOSBox, it seems that it's
/// not possible to fill the entire buffer
/// 
/// ```
///     loop {
///        println!("Please write 'cats' to exit");
///        let text = prompt(6);
///
///        if text == "cats" {
///            println!("Thanks for cats!");
///            break;
///        }
///    }
/// ```
pub fn prompt(length: u8) -> String {
    let mut buffer: Vec<u8> = Vec::new();
    let bytes_read;

    buffer.resize((length + 3) as usize, 0);
    buffer[0] = length;

    let (segment, address) = ptr_to_segments(buffer.as_ptr() as u32);

    unsafe {
        asm!(
            "mov ax, ds",       // We need to move the data segment register to
            "push ax",          // point to the right part of the heap that is 
            "add ax, cx",       // holding our buffer
            "mov ds, ax",

            "mov ah, 0x0a",     
            "int 0x21",         // Call function

            "pop ax",           // Restore data segment register
            "mov ds, ax",
            in("dx") address,
            in("cx") segment
        );

        bytes_read = buffer[1] as usize;

        String::from_raw_parts(buffer[2 .. 2 + bytes_read].as_mut_ptr(), bytes_read, bytes_read)
    }    
}