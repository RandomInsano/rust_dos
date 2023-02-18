//! 
//! Currently implemented DOS INT 21 functions:
//! 
//! | Code  | Description                                                     |
//! |-------|-----------------------------------------------------------------|
//! | 01 ✓  | [Read character from STDIN](dos::console::readc)                |
//! | 02 ✓  | [Write character to STDOUT](dos::console::printc)               |
//! | 03    | Read character from AUX (serial)                                |
//! | 04    | Write character to AUX (serial)                                 |
//! | 05    | Write character to printer                                      |
//! | 06    | Console Input/Output                                            |
//! | 07    | Direct character read, no echo                                  |
//! | 08    | Char read from STDIN, no echo                                   |
//! | 09 ✓  | [Write string to STDOUT](dos::console::print)                   |
//! | 0A ✓  | [Buffered input](dos::console::prompt)                          |
//! | 0B    | Get STDIN status                                                |
//! | 0C    | Flush buffer from STDIN                                         |
//! | 0D    | Disk reset                                                      |
//! | 0E    | Select default drive                                            |
//! | 19    | Get current default drive                                       |
//! | 1B  ⚠️ | Replaced by 36 (Get free disk space)                            |
//! | 1C  ⚠️ | Replaced by 36 (Get free disk space)                            |
//! | 25    | Set interrupt vector                                            |
//! | 2A ✓  | Get system date                                                 |
//! | 2B ✓  | Set system date                                                 |
//! | 2C ✓  | Get system time                                                 |
//! | 2D ✓  | Set system time                                                 |
//! | 2E ✓  | [Enable write verification](dos::file::set_verify_writes)       |
//! | 30 ✓  | [Get DOS version](dos::misc::dos_version)                       |
//! | 35    | Get interrupt vector                                            |
//! | 36    | [Get free disk space](dos::file::StorageParameters::disk_space) |
//! | 39 ✓  | Create subdirectory                                             |
//! | 3A ✓  | Remove subdirectory                                             |
//! | 3B ✓  | Change current working directory                                |
//! | 3C    | Create file                                                     |
//! | 3D ~  | [Open file](dos::file::File::open)                              |
//! | 3E ✓  | [Close file](dos::file::File::close)                            |
//! | 3F ✓  | [Read file](dos::file::File::write)                             |
//! | 40 ✓  | [Write file handle](dos::file::File::write)                     |
//! | 41    | [Delete file](dos::file::File::delete)                          |
//! | 42 ✓  | [Seek file](dos::file::File::close)                             |
//! | 43 ~  | [Get/set file attributes](dos::file::File::attributes)          |
//! | 47    | Get current directory                                           |
//! | 4C ✓  | Exit program                                                    |
//! | 4D    | Get return code                                                 |
//! | 54    | [Check file verification status](dos::file::verify_writes)      |
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
pub mod software;
extern crate rlibc;
extern crate alloc;

use crate::dos::new_allocator::GLOBAL_ALLOCATOR;

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