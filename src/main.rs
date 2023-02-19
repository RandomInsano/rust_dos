#![no_std]
#![no_main]

extern crate alloc;

mod dos_tests;

use rust_dos::*;
use rust_dos::bios::video::VesaMode;
use rust_dos::bios::{
    video,
};
use rust_dos::dos::process;

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
    // let mode = VesaMode::new(0x103,
    //     false,
    //     true,
    //     false);

    // video::set_video_vesa(mode).unwrap();

    let psp = process::current_psp();

    println!("Command line: {:?}", psp.command_line());

    println!("PSP: {:?}", psp);
    println!("Environment: {:#?}", psp.environment());
}
