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

fn create_ball<'a>(assets: RawBitmap) -> Sprite {
    let ball_rect = Rect::new(0, 0, 10, 10).unwrap();
    let dest_point = Point::new(0, 0);
    let mut ball_image = RawBitmap::new_blank(ball_rect);

    assets.blit(ball_image.rect, &mut ball_image, dest_point.clone(), BlitOperation::Direct);

    Sprite::new(ball_rect, ball_image, None)
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
    let bitmap_background = Bitmap::load(PATH_SPLASH_SCREEN).unwrap();
    graphics::set_vga_dac(bitmap_background.palette());
    
    let bitmap_background = RawBitmap::from(bitmap_background);
    let mut bitmap_framebuffer = graphics::get_framebuffer();

    bitmap_background.blit(bitmap_background.rect, &mut bitmap_framebuffer, bitmap_background.rect.location(), BlitOperation::Direct);

    let mut ball = create_ball(bitmap_assets);
    let mut ball_rect;

    loop {
        ball_x += ball_velocity_x;
        ball_y += ball_velocity_y;
        ball_rect = ball.image_rect();
        ball_rect.x = ball_x as i32;
        ball_rect.y = ball_y as i32;

        ball.draw(&mut bitmap_framebuffer, ball_rect.location());

        // Bounce if we hit an edge. Not well, but bounce
        if let Some(interection) = ball_rect.intersection(&screen_rect) {
            if interection.width < ball_rect.width {
                ball_velocity_x *= -1.0;
            }

            if interection.height < ball_rect.height {
                ball_velocity_y *= -1.0;
            }
        }

        ball.erase(&mut bitmap_framebuffer);
    }

    //println!("Done! I hope you enjoyed! \u{1}");
}
