//! A simple (for now) game of destroying blocks with a bouncy ball
//! 

#![no_std]
#![no_main]

pub mod graphics;
pub mod game_objects;

extern crate alloc;
extern crate image_viewer;

entry!(main);

use image_viewer::bitmap::Bitmap;
use graphics::{
    Rect,
    RawBitmap,
    set_vga_dac,
    display_image,
};
use rust_dos::*;
use rust_dos::bios::{
    video,
    video::VideoMode,
};

use crate::game_objects::Sprite;
use crate::graphics::{Point, BlitOperation};

const PATH_SPLASH_SCREEN: &str = ".\\clouds.bmp\0";
const PATH_ASSETS: &str = ".\\assets.bmp\0";
const PATH_ASSETS_MASK: &str = ".\\assetsm.bmp\0";

fn create_ball<'a>(assets: RawBitmap, mask: RawBitmap) -> Sprite {
    let ball_rect = Rect::new(0, 0, 10, 10).unwrap();
    let dest_point = Point::new(0, 0);
    let mut ball_image = RawBitmap::new_blank(ball_rect);
    let mut ball_mask = RawBitmap::new_blank(ball_rect);

    assets.blit(ball_image.rect, &mut ball_image, dest_point.clone(), BlitOperation::Direct);
    mask.blit(ball_mask.rect, &mut ball_mask, dest_point, BlitOperation::Direct);

    Sprite::new(ball_rect, ball_image, Some(ball_mask))
}

fn main() {
    let mut ball_x: f32 = 50.0;
    let mut ball_y: f32 = 50.0;
    let mut ball_velocity_x: f32 = 1.0;
    let mut ball_velocity_y: f32 = 1.1;
    let screen_rect = Rect::new(1, 1, 319, 176).unwrap();

    video::set_video(VideoMode::Graphics320_200C8);
    video::set_cursor_position(0, 13, 20);

    println!("Please wait...");
    
    let bitmap_assets = RawBitmap::from(Bitmap::load(PATH_ASSETS).unwrap());
    let bitmap_assets_mask = RawBitmap::from(Bitmap::load(PATH_ASSETS_MASK).unwrap());
    let bitmap_background = Bitmap::load(PATH_SPLASH_SCREEN).unwrap();
    set_vga_dac(bitmap_background.palette());
    
    let mut bitmap_background = RawBitmap::from(bitmap_background);
    //let mut bitmap_background = RawBitmap::new_blank(Rect::new(0, 0, 320, 200).unwrap());

    let mut ball = create_ball(bitmap_assets, bitmap_assets_mask);
    let mut ball_rect;

    loop {
        ball_x += ball_velocity_x;
        ball_y += ball_velocity_y;
        ball_rect = ball.image_rect();
        ball_rect.x = ball_x as i32;
        ball_rect.y = ball_y as i32;

        // Bounce if we hit an edge. Not well, but bounce
        if let Some(interection) = ball_rect.intersection(&screen_rect) {
            if interection.width < ball_rect.width {
                ball_velocity_x *= -1.0;
            }

            if interection.height < ball_rect.height {
                ball_velocity_y *= -1.0;
            }
        }

        ball.draw(&mut bitmap_background, ball_rect.location());

        display_image(&bitmap_background).unwrap();

        ball.erase(&mut bitmap_background);
    }

    //println!("Done! I hope you enjoyed! \u{1}");
}
