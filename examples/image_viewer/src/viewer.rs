use alloc::vec::Vec;
use super::bitmap::Bitmap;
use rust_dos::dos::error_code::ErrorCode;
use rust_dos::print;
use rust_dos::println;
use rust_dos::bios::video::{
    VgaDacColour,
    get_vga_pointer,
    self
};

pub fn display_image(bitmap: Bitmap) -> Result<(), ErrorCode> {
    let screen_memory = get_vga_pointer();
    let header = bitmap.header();
    
    if header.width() != 320 || header.bpp() != 8 || header.compression() != 0 {
        println!("Image dimensions for splash screen are wrong. Exiting");
        return Err(ErrorCode::InvalidFormat)
    }

    // Set the DAC to show the right colours for the image bitmap
    // TODO: I think it's possible to define the DAC to be 8bit...
    let vga_dac: Vec<VgaDacColour> = bitmap.palette()
        .iter()
        .map(|x| {
            VgaDacColour {
                red: x.red() / 4,
                green: x.green() / 4,
                blue: x.blue() / 4
            }
        }).collect();

    video::set_vga_dac(&vga_dac, 0);

    // Copy image data to the video card
    unsafe {
        // Bitmaps' scalines start at the bottom instead of the top and are
        // padded to multiples of 4 so we need to do some smart copying
        let mut image_pointer = bitmap.image_data().as_ptr();
        let mut screen_pointer = screen_memory.clone();
        screen_pointer = screen_pointer.offset(320 * 200);
        let image_data_width = header.width() + (header.width() % 4);

        for _ in 0 .. header.height() {
            screen_pointer = screen_pointer.sub(320);

            screen_pointer.copy_from(image_pointer, 320);

            image_pointer = image_pointer.add(image_data_width as usize)
        }
    }

    Ok(())
}
