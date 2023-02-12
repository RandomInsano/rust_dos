//! Example bitmap slideshow viewer
//! ==========================================================================
//! Mostly exists because of the need to load bitmaps for the Breakout game,
//! it's a good example of how to use the file reading calls and the RAMDAC
//! in VGA cards to set colour palettes
//! 
//! Warning here, this is a very fast and loose implementation that loads chunks
//! of memory into buffers then overlays those into data structures. These
//! assumptions mean you can't use this code on big endian targets and if the
//! file is corrupt you'll likely have a very bad day.
//! 
//! It also doesn't handle things like compression. The examples in this project
//! were made using GIMP and using the Image > Mode > Indexed... feature then
//! exporting them as Windows BMP files. It's not implemented here yet but you
//! should reserve two colours in the palette so that text and mouse cursor can
//! have a good foreground/background colour.
//! 
//! //! References:
//! * [BMP File Format](http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm)
//! * http://www.martinreddy.net/gfx/2d/BMP.txt
//! 
//! TODO items:
//! * Get intial graphics mode so we can swap back instead of making assumptions
//! * Allow specifying a path to look for bitmaps instead of hard coding
//! * Implement proper Read trait for DOS and promote (or replace) bitmap loading

#![no_std]
#![no_main]

entry!(main);

mod bitmap;
mod viewer;

extern crate alloc;

use bitmap::Bitmap;
use viewer::display_image;
use rust_dos::*;
use rust_dos::bios::{
    video,
    video::VideoMode,
};
use rust_dos::dos::console;
use rust_dos::dos::datetime::Time;
use rust_dos::software::mouse::Mouse;

const WALLPAPERS: &[&str] = &[
    "examples\\clouds.bmp\0",
    "examples\\bricks.bmp\0",
    "examples\\mountain.bmp\0",
];

const CYCLES: usize = 3;
const DELAY: u32 = 5;

/// Busywait until the time has elapsed
fn sleep(seconds: u32) {
    let start = Time::now().to_seconds();

    while start + seconds > Time::now().to_seconds() { }
}

fn main() {
    let mut wallpaper_index = 0;
    let mut bitmap: Bitmap;

    println!("This program will cycle through {} bitmaps every {} seconds", WALLPAPERS.len(), DELAY);
    println!("{} times then exit. This demo requires a VGA graphics card.", CYCLES);
    println!("");
    println!("Press any key to start.");
    console::readc();

    video::set_video(VideoMode::Graphics320_200C8);
    video::set_cursor_position(0, 13, 20);

    println!("Please wait...");
    Mouse::cursor_show();

    for _ in 0 .. 3 * WALLPAPERS.len() {
        // Clears the screen
        video::set_video(VideoMode::Graphics320_200C8);

        bitmap = Bitmap::load(WALLPAPERS[wallpaper_index])
            .expect("Unable to load bitmap");

        display_image(bitmap)
            .expect("Unable to display image");

        sleep(DELAY);

        wallpaper_index = (wallpaper_index + 1) % WALLPAPERS.len();
    }

    video::set_video(VideoMode::Text80_25C);
    println!("Done! I hope you enjoyed! \u{1}");

    Mouse::cursor_hide();
}
