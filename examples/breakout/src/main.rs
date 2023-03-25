//! A simple (for now) game of destroying blocks with a bouncy ball
//! 

#![no_std]
#![no_main]

pub mod graphics;
pub mod game_objects;

extern crate alloc;
extern crate image_viewer;

entry!(main);

use alloc::vec::Vec;
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

use graphics::{
    BlitOperation,
    Point,
    Sprite
};

use game_objects::BrickGraphics;

use crate::game_objects::Brick;

const PATH_SPLASH_SCREEN: &str = ".\\clouds.bmp\0";
const PATH_ASSETS: &str = ".\\assets.bmp\0";

const PLAYFIELD_WIDTH: usize = 32;
const PLAYFIELD_HEIGHT: usize = 20;
const GAME_DATA: [u8; PLAYFIELD_WIDTH * PLAYFIELD_HEIGHT] = [
    0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa2, 0xc2, 0xa2, 0xc2, 0xa2, 0xc2, 0xa2, 0xc2, 0xa2, 0xc2, 0xa2, 0xc2, 0xa3, 0xc3, 0xa3, 0xc3, 0xa3, 0xc3, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0xc4, 0xa4, 0xc4, 0xa4, 0xc4, 0xa4, 0xc4, 0xa5, 0xc5, 0xa5, 0xc5, 0xa5, 0xc5, 0xa6, 0xc6, 0xa6, 0xc6, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa7, 0xc7, 0xa7, 0xc7, 0xa7, 0xc7, 0xa7, 0xc7, 0xa8, 0xc8, 0xa8, 0xc8, 0xa8, 0xc8, 0xa8, 0xc8, 0xa9, 0xc9, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa9, 0xc9, 0xa9, 0xc9, 0xaa, 0xca, 0xaa, 0xca, 0xaa, 0xca, 0xaa, 0xca, 0xab, 0xcb, 0xab, 0xcb, 0xab, 0xcb, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0xcc, 0xac, 0xcc, 0xac, 0xcc, 0xac, 0xcc, 0xad, 0xcd, 0xad, 0xcd, 0xad, 0xcd, 0xae, 0xce, 0xae, 0xce, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xcf, 0xaf, 0xcf, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0xa1, 0xc1, 0x00, 0x00, 0x00, 0x61,
    0x61, 0xa1, 0xc1, 0xa1, 0xc1, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61,
    0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61,
];

fn create_ball<'a>(assets: &RawBitmap) -> Sprite {
    let ball_rect = Rect::new(0, 0, 10, 10).unwrap();
    let dest_point = Point::new(0, 0);
    let mut ball_image = RawBitmap::new_blank(ball_rect);

    assets.blit(ball_image.rect, &mut ball_image, dest_point.clone(), BlitOperation::Direct);

    Sprite::new(ball_rect, ball_image, None)
}

/// Bricks are loaded as an n by 3 dimensional array. The columns are the part of
/// the brick to draw (left, right, half) and the row is a style to lend some
/// variety. Eventually the six colours used to draw the bricks will vary depending
/// on the brick type and they will index along the colour palette to save on
/// storage
/// 
/// After this has all been set up, you can draw a brick using the [BrickGraphics]
/// that's returned from this function
fn create_brick<'a>(assets: &RawBitmap) -> BrickGraphics {
    let brick_rect = Rect::new(0, 0, 10, 10).unwrap();
    let mut source_rect =  Rect::new(10, 0, 10, 10).unwrap();
    let mut images = Vec::new();

    for y in 0 .. 8 {
        let mut row = Vec::new();

        for x in 0 .. 3 {
            source_rect.y = y * 10;
            source_rect.x = x * 10 + 10;

            let mut brick_image = RawBitmap::new_blank(brick_rect);
            assets.blit(source_rect, &mut brick_image, brick_rect.location(), BlitOperation::Direct);
            row.push(brick_image);
        }

        images.push(row)
    }

    BrickGraphics::new(images)
}

fn point_to_data(location: Point, dimensions: Point) -> usize {
    (location.y as usize * dimensions.x as usize) + location.x as usize
}

fn main() {
    let mut ball_x: f32 = 20.0;
    let mut ball_y: f32 = 20.0;
    let mut ball_velocity_x: f32 = 0.3;
    let mut ball_velocity_y: f32 = 0.6;
    let mut ball_last_rect = Rect::new(0, 0, 0, 0).unwrap();

    video::set_video(VideoMode::Graphics320_200C8);
    video::set_cursor_position(0, 13, 20);

    println!("Please wait...");
    
    let bitmap_assets = Bitmap::load(PATH_ASSETS).unwrap();
    let bitmap_background = RawBitmap::from(Bitmap::load(PATH_SPLASH_SCREEN).unwrap());
    //graphics::set_vga_dac(bitmap_background.palette());
    graphics::set_vga_dac(bitmap_assets.palette());
    
    let bitmap_assets = RawBitmap::from(bitmap_assets);
    let bitmap_background = RawBitmap::from(bitmap_background);
    let mut bitmap_framebuffer = graphics::get_framebuffer();

    bitmap_background.blit(bitmap_background.rect, &mut bitmap_framebuffer, bitmap_background.rect.location(), BlitOperation::Direct);

    let mut ball = create_ball(&bitmap_assets);
    let mut ball_rect = ball.image_rect();
    let mut brick = create_brick(&bitmap_assets);
    let mut brick_point = Point::new(0, 0);
    // Data grid is 1/10 the graphics. We'll divide the ball positions by this amount
    let data_scale = Point::new(10, 10);
    // Dimensions of the data grid
    let data_dimensions = Point::new(PLAYFIELD_WIDTH as i32, PLAYFIELD_HEIGHT as i32);

    for y in 0 .. PLAYFIELD_HEIGHT {
        for x in 0 .. PLAYFIELD_WIDTH {
            brick_point.x = x as i32 * 10;
            brick_point.y = y as i32 * 10;

            let index = (y * PLAYFIELD_WIDTH) + x;
            let brick_data = GAME_DATA[index];

            brick.draw(&mut bitmap_framebuffer, brick_point, Brick::from(brick_data));
        }
    }

    ball.draw(&mut bitmap_framebuffer, ball_rect.location());

    loop {
        ball_x += ball_velocity_x;
        ball_y += ball_velocity_y;
        ball_rect = ball.image_rect();
        ball_rect.x = ball_x as i32;
        ball_rect.y = ball_y as i32;

        if ball_velocity_y > 0.0 {
            // If we're travelling down, see if we've crashed into something
            let ll = ball_rect.lower_left() / data_scale;
            let lr = ball_rect.lower_right() / data_scale;            
            let ll_data = GAME_DATA[point_to_data(ll, data_dimensions)];
            let lr_data = GAME_DATA[point_to_data(lr, data_dimensions)];

            if Brick::from(ll_data).brick_type() != 0 && Brick::from(lr_data).brick_type() != 0 {
                ball_velocity_y *= -1.0
            }
        } else if ball_velocity_y < 0.0 {
            // If we're travelling up, see if we've crashed into something
            let ul = ball_rect.upper_left() / data_scale;
            let ur = ball_rect.upper_right() / data_scale;
            let ul_data = GAME_DATA[point_to_data(ul, data_dimensions)];
            let ur_data = GAME_DATA[point_to_data(ur, data_dimensions)];

            if Brick::from(ul_data).brick_type() != 0 && Brick::from(ur_data).brick_type() != 0 {
                ball_velocity_y *= -1.0
            }
        }

        if ball_velocity_x > 0.0 {
            // If we're travelling rigth, see if we've crashed into something
            let ur = ball_rect.upper_right() / data_scale;
            let lr = ball_rect.lower_right() / data_scale;            
            let ur_data = GAME_DATA[point_to_data(ur, data_dimensions)];
            let lr_data = GAME_DATA[point_to_data(lr, data_dimensions)];

            if Brick::from(ur_data).brick_type() != 0 && Brick::from(lr_data).brick_type() != 0 {
                ball_velocity_x *= -1.0
            }
        } else if ball_velocity_x < 0.0 {
            // If we're travelling left, see if we've crashed into something
            let ul = ball_rect.upper_left() / data_scale;
            let ll = ball_rect.lower_left() / data_scale;
            let ul_data = GAME_DATA[point_to_data(ul, data_dimensions)];
            let ll_data = GAME_DATA[point_to_data(ll, data_dimensions)];

            if Brick::from(ul_data).brick_type() != 0 && Brick::from(ll_data).brick_type() != 0 {
                ball_velocity_x *= -1.0
            }
        }

        // Wait for VSync here somwheres

        // Don't draw if the ball hasn't changed position
        if ball_rect.location() == ball_last_rect.location() {
            continue;
        }

        ball_last_rect = ball_rect;

        ball.erase(&mut bitmap_framebuffer);
        ball.draw(&mut bitmap_framebuffer, ball_rect.location());
    }

    //println!("Done! I hope you enjoyed! \u{1}");
}
