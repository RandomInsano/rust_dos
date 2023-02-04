//! 
//! Currently implemented DOS INT 21 functions:
//! 
//! | Code  | Description                                                     |
//! |-------|-----------------------------------------------------------------|
//! | 01    | Read character from STDIN                                       |
//! | 02 ✓  | Write character to STDOUT                                       |
//! | 03    | Read character from AUX (serial)                                |
//! | 04    | Write character to AUX (serial)                                 |
//! | 05    | Write character to printer                                      |
//! | 06    | Console Input/Output                                            |
//! | 07    | Direct character read, no echo                                  |
//! | 08    | Char read from STDIN, no echo                                   |
//! | 09 ✓  | Write string to STDOUT                                          |
//! | 0A    | Buffered input                                                  |
//! | 0B    | Get STDIN status                                                |
//! | 0C    | Flush buffer from STDIN                                         |
//! | 0D    | Disk reset                                                      |
//! | 0E    | Select default drive                                            |
//! | 19    | Get current default drive                                       |
//! | 25    | Set interrupt vector                                            |
//! | 2A ✓  | Get system date                                                 |
//! | 2B ✓  | Set system date                                                 |
//! | 2C ✓  | Get system time                                                 |
//! | 2D ✓  | Set system time                                                 |
//! | 2E    | Set verify flag                                                 |
//! | 30 ✓  | Get DOS version                                                 |
//! | 35    | Get interrupt vector                                            |
//! | 36    | Get free disk space                                             |
//! | 39 ✓  | Create subdirectory                                             |
//! | 3A ✓  | Remove subdirectory                                             |
//! | 3B ✓  | Change current working directory                                |
//! | 3C    | Create file                                                     |
//! | 3D ~  | Open file                                                       |
//! | 3E ✓  | Close file                                                      |
//! | 3F ✓  | Read file                                                       |
//! | 40 ✓  | Write file                                                      |
//! | 41    | Delete file                                                     |
//! | 42 ✓  | Seek file                                                       |
//! | 43 ~  | Get/set file attributes                                         |
//! | 47    | Get current directory                                           |
//! | 4C ✓  | Exit program                                                    |
//! | 4D    | Get return code                                                 |
//! | 54    | Get verify flag                                                 |
//! | 56    | Rename file                                                     |
//! | 57 ~  | Get/set file date                                               |
//! 
//! Legend:
//! * ✓ = All features implemented
//! * ⚠️ = Deprecated function
//! * ~ = Partial features implemented
//! 
//! Reference material:
//! * [DOS INT 21h - DOS Function Codes](http://spike.scu.edu.au/~barry/interrupts.html#ah36)
//! * [Registers in x86 Assembly](https://www.cs.uaf.edu/2017/fall/cs301/lecture/09_11_registers.html)
//! * [MS-DOS Version 4.0 Programmer's Reference](https://www.pcjs.org/documents/books/mspl13/msdos/dosref40/)
//! 

#![no_std]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod dos;
pub mod bios;
pub mod dpkey;
extern crate rlibc;
extern crate alloc;

use crate::dos::allocator::GLOBAL_ALLOCATOR;

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe {
        GLOBAL_ALLOCATOR.init();
    }
    extern "Rust" {
        fn main() -> ();
    }
    unsafe {
        main();
    }
    dos::exit(0);
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;
            f()
        }
    };
}