//! A simple (for now) game of destroying blocks with a bouncy ball
//! 
//! References:
//! * [BMP File Format](http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm)
//! * http://www.martinreddy.net/gfx/2d/BMP.txt


#![no_std]
#![no_main]

extern crate alloc;

use core::arch::asm;
use core::mem;
use alloc::vec::Vec;
use rust_dos::*;
use rust_dos::bios::video::VgaDacColour;
use rust_dos::bios::{
    video,
    video::VideoMode,
};
use rust_dos::dos::file::{File, SeekFrom};
use rust_dos::software::mouse::Mouse;
entry!(main);

#[derive(Debug)]
#[repr(C, packed)]
pub struct BitmapFileHeader {
    id: [u8; 2],
    size: u32,
    _reserved: [u8; 4],
    image_offset: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct BitmapInfoHeader {
    header_size: u32,
    width: u32,
    height: u32,
    colour_planes: u16,
    bpp: u16,
    compression: u32,
    image_size: u32,
    pixel_width_per_meter: u32,
    pixel_height_per_metere: u32,
    pallette_colour_count: u32,
    important_colour_count: u32,
}

#[repr(C, packed)]
/// Colours for each index
pub struct BitmapColourEntry {
    blue: u8,
    green: u8,
    red: u8,
    _reserved: u8
}

fn read_bitmap(filename: &str) -> (BitmapInfoHeader, Vec<BitmapColourEntry>, Vec<u8>) {
    let file_header: BitmapFileHeader;
    let bitmap_header: BitmapInfoHeader;
    let mut colour_palette: Vec<BitmapColourEntry> = Vec::new();

    let file_handle = File::open(filename).unwrap();

    let mut buffer = [0u8; mem::size_of::<BitmapFileHeader>()];
    file_handle.read(&mut buffer).unwrap();
    file_header = unsafe {
        mem::transmute(buffer)
    };

    assert_eq!(file_header.id, [0x42, 0x4d]);

    let mut buffer = [0u8; mem::size_of::<BitmapInfoHeader>()];
    file_handle.read(&mut buffer).unwrap();
    bitmap_header = unsafe {
        mem::transmute(buffer)
    };

    let palette_data_offset = mem::size_of::<BitmapFileHeader>() as u32 + bitmap_header.header_size;
    file_handle.seek(SeekFrom::Start(palette_data_offset)).unwrap();

    if bitmap_header.bpp <= 8 {
        let mut buffer = [0u8; mem::size_of::<BitmapColourEntry>()];

        for _ in 0 .. bitmap_header.pallette_colour_count {
            file_handle.read(&mut buffer).unwrap();
            colour_palette.push(unsafe {
                mem::transmute(buffer)
            });
        }
    }

    // Jump to the start of image data
    file_handle.seek(SeekFrom::Start(file_header.image_offset)).unwrap();

    let mut image_data = Vec::new();
    image_data.resize(bitmap_header.image_size as usize, 0u8);
    file_handle.read(&mut image_data).unwrap();

    (bitmap_header, colour_palette, image_data)
}

fn main() {
    let code_segment: u16;

    video::set_video(VideoMode::Graphics320_200C8);
    video::set_cursor_position(0, 0, 20);
    video::set_page(1);
    
    // Because Rust pointers are referenced based on where the program is loaded
    // in memory, we need to get that offset and do some math on it 
    unsafe {
        asm!(
            "mov ax, cs",
            out("ax") code_segment
        );
    }

    let screen_memory = (0xA_0000 - (code_segment as u32 * 16)) as *mut u8;

    let (
        header,
        palette,
        image_data
    ) = read_bitmap("examples\\bricks.bmp\0");

    if header.width != 320 || header.height != 200 || header.bpp != 8 || header.compression != 0 {
        println!("Image dimensions for splash screen are wrong. Exiting");
        return;
    }

    // Copy image data to the video card
    unsafe {
        // Bitmaps' scalines start at the bottom instead of the top and are
        // padded to multiples of 4 so we need to do some smart copying
        let mut image_pointer = image_data.as_ptr();
        let mut screen_pointer = screen_memory.clone();
        screen_pointer = screen_pointer.offset(320 * 200);
        let image_data_width = header.width + (header.width % 4);

        for _ in 0 .. header.height {
            screen_pointer = screen_pointer.sub(320);

            screen_pointer.copy_from(image_pointer, 320);

            image_pointer = image_pointer.add(image_data_width as usize)
        }
    }

    // Set the DAC to show the right colours for the image bitmap
    // TODO: I think it's possible to define the DAC to be 8bit...
    let vga_dac: Vec<VgaDacColour> = palette.iter().map(|x| {
        VgaDacColour {
            red: x.red / 4,
            green: x.green / 4,
            blue: x.blue / 4
        }
    }).collect();

    video::set_vga_dac(&vga_dac, 0);

    // Show the first page that's now fill of our picture
    video::set_page(0);

    Mouse::cursor_show();

    println!("Done! I hope you enjoyed :)");
}
