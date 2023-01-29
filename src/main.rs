#![no_std]
#![no_main]


extern crate alloc;

mod dos_tests;

use rust_dos::*;
use crate::dos_tests::allocator_test::allocator_test;
use crate::dos_tests::datetime::datetime_test;
use crate::dos_tests::file::file_read_test;
use crate::dos_tests::file::file_attribute_test;
use crate::dos_tests::file::directory_test;
use crate::dos_tests::misc::misc_test;
use crate::dos_tests::console::print_test;

entry!(main);

fn main() {
    println!("-- Allocator tests");
    allocator_test();
    println!("-- File read tests");
    file_read_test();
    println!("-- File attribute tests");
    file_attribute_test();
    println!("-- Directory tests");
    directory_test();
    println!("-- Date/time tests");
    datetime_test();
    println!("-- Misc tests");
    misc_test();

    println!("-- Print tests");
    print_test();
}
