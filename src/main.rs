#![no_std]
#![no_main]

extern crate alloc;

mod dos_tests;

use rust_dos::*;
use rust_dos::bios::video::{ColorTarget, VesaMode};
use rust_dos::bios::{
    video,
};
use crate::dos_tests::allocator_test::allocator_test;
use crate::dos_tests::datetime::datetime_test;
use crate::dos_tests::file::file_read_test;
use crate::dos_tests::file::file_attribute_test;
use crate::dos_tests::file::directory_test;
use crate::dos_tests::misc::misc_test;
use crate::dos_tests::console::print_test;

entry!(main);

#[allow(dead_code)]
const PICTURE_DATA: [[u8; 8]; 8] = [
    [000, 000, 000, 000, 000, 000, 000, 0], 
    [000, 000, 128, 000, 000, 128, 000, 0], 
    [000, 000, 128, 000, 000, 128, 000, 0], 
    [000, 000, 128, 000, 000, 128, 000, 0], 
    [000, 000, 000, 000, 000, 000, 000, 0], 
    [000, 128, 000, 000, 000, 000, 128, 0], 
    [000, 000, 128, 128, 128, 128, 000, 0], 
    [000, 000, 000, 000, 000, 000, 000, 0], 
];

fn main() {
    // Set resolution to 800x600x8
    let mode = VesaMode::new(0x103,
        false,
        true,
        false);

    video::set_video_vesa(mode).unwrap();
    video::set_cursor_size(0x01, 0x0f);
    video::set_page(1);
    video::set_palette(ColorTarget::Palette, 5);

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
