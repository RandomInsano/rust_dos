//! A simple (for now) game of destroying blocks with a bouncy ball
//! 

#![no_std]
#![no_main]

entry!(main);

mod image_viewer;

extern crate alloc;

use image_viewer::bitmap::Bitmap;
use image_viewer::viewer::display_image;
use rust_dos::*;
use rust_dos::bios::{
    video,
    video::VideoMode,
};

const PATH_SPLASH_SCREEN: &str = "examples\\clouds.bmp\0";


fn main() {
    video::set_video(VideoMode::Graphics320_200C8);
    video::set_cursor_position(0, 13, 20);

    println!("Please wait...");
    
    let bitmap = Bitmap::load(PATH_SPLASH_SCREEN).unwrap();
    display_image(bitmap).unwrap();

    println!("Done! I hope you enjoyed! \u{1}");
}
