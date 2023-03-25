use alloc::boxed::Box;
use alloc::vec::Vec;
use image_viewer::bitmap::{BitmapColourEntry, Bitmap};
use rust_dos::bios::video::VgaDacColour;
use rust_dos::{
    bios::video::get_vga_pointer,
};

pub fn set_vga_dac(colour_entries: &[BitmapColourEntry]) {
    // Set the DAC to show the right colours for the image bitmap
    // TODO: I think it's possible to define the DAC to be 8bit...
    let vga_dac: Vec<VgaDacColour> = colour_entries
        .iter()
        .map(|x| {
            VgaDacColour {
                red: x.red() / 4,
                green: x.green() / 4,
                blue: x.blue() / 4
            }
        }).collect();

    rust_dos::bios::video::set_vga_dac(&vga_dac, 0);
}

pub fn get_framebuffer() -> RawBitmap {
    let rect = Rect::new(0, 0, 320, 200).unwrap();
    let bitmap = unsafe {
        Box::from_raw(get_vga_pointer() as *mut [u8; 320 * 200])
    };

    RawBitmap { rect, bitmap }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl core::ops::Div for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl core::ops::Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.y,
            y: self.y * rhs.y
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Rect {
    /// Create a new Rect. Returns an Err when either width and height are
    /// negative
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Result<Self, ()> {
        if width < 0 || height < 0 {
            return Err(())
        }

        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    /// Does this Rect intersect another?
    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    /// Calculates the intersecting rectangle (if it exists)
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x1 < x2 && y1 < y2 {
            Some(Self {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1
            })
        } else {
            None
        }
    }

    /// Create a Rect that is the union of two other rects
    pub fn union(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let width = (self.x + self.width).max(other.x + other.width) - x;
        let height = (self.y + self.height).max(other.y + other.height) - y;
        Self {
            x,
            y,
            width,
            height
        }
    }

    pub fn offset(&mut self, distance: Point) {
        self.x += distance.x;
        self.y += distance.y;
    }

    pub fn location(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn upper_left(&self) -> Point {
        self.location()
    }

    pub fn upper_right(&self) -> Point {
        Point::new(self.x + self.width, self.y)
    }

    pub fn lower_left(&self) -> Point {
        Point::new(self.x, self.y + self.height)
    }

    pub fn lower_right(&self) -> Point {
        Point::new(self.x + self.width, self.y + self.height)
    }
}

/// Just a block of memory with a width and a height than can blit between
/// instances of itself
pub struct RawBitmap {
    pub(crate) rect: Rect,
    pub(crate) bitmap: Box<[u8]>
}

pub enum BlitOperation {
    And,
    Or,
    Direct,
    /// The value provided here is considered the transparency colour
    Keyed(u8),
}

impl From<Bitmap> for RawBitmap {
    fn from(value: Bitmap) -> Self {
        let rect = Rect::new(
            0, 
            0, 
            value.header().width() as i32,
            value.header().height() as i32
        ).unwrap();

        // Bitmaps store scanlines in reverse so the top of the image is at the
        // end of the file. We have to flip it.
        let mut buffer = Vec::new();
        buffer.resize((rect.width * rect.height) as usize, 0);

        let width = rect.width as usize;
        let mut index_bitmap = 0;
        let mut index_buffer = buffer.len() - width;

        for _ in 0 .. rect.height {
            buffer[index_buffer .. index_buffer + width]
              .copy_from_slice(&value.image_data()[index_bitmap .. index_bitmap + width]);

            index_bitmap += width;
            index_buffer -= width;
        }

        RawBitmap::new(rect, buffer.into_boxed_slice())
    }
}

impl RawBitmap {
    pub fn new(rect: Rect, bitmap: Box<[u8]>) -> Self {
        Self {
            rect,
            bitmap,
        }
    }

    pub fn new_blank(buffer_rect: Rect) -> Self {
        let mut buffer_data = Vec::new();
        buffer_data.resize((buffer_rect.width * buffer_rect.height) as usize, 0u8);
        RawBitmap::new(buffer_rect, buffer_data.into_boxed_slice())
    }

    /// Rotate the pixels in the current rect by colour amount. If the result is
    /// more than 255 it will wrap around so to shift the palette in the
    /// opposite direction provide a value of (255 - amount).
    pub fn shift_colour(&mut self, rect: Rect, colour: u8) {
        let step = self.rect.width as usize;
        let width = rect.width as usize;
        let mut offset = (rect.y as usize * step) + rect.x as usize;

        for _ in 0..rect.height {
            let row = &mut self.bitmap[offset..offset + width];

            for x in 0 .. width {
                row[x] = row[x].wrapping_add(colour);
            }

            offset += step;
        }        
    }

    pub fn blit(&self, source_rect: Rect, destination: &mut Self, destination_point: Point, operation: BlitOperation) {
        let src_step = self.rect.width as usize;
        let mut src_offset = (source_rect.y as usize * src_step) + source_rect.x as usize;
        
        let dest_step = destination.rect.width as usize;
        let mut dest_offset = destination_point.y as usize * dest_step + destination_point.x as usize;

        let width = source_rect.width as usize;

        match operation {
            BlitOperation::And => {
                for _ in 0..source_rect.height {
                    let src_row = &self.bitmap[src_offset..src_offset + width];
                    let dest_row = &mut destination.bitmap[dest_offset..dest_offset + width];
        
                    for x in 0 .. width {
                        dest_row[x] = dest_row[x] & src_row[x];
                    }
        
                    src_offset += src_step;
                    dest_offset += dest_step;
                }        
            },
            BlitOperation::Or => {
                for _ in 0..source_rect.height {
                    let src_row = &self.bitmap[src_offset..src_offset + width];
                    let dest_row = &mut destination.bitmap[dest_offset..dest_offset + width];
        
                    for x in 0 .. width {
                        dest_row[x] = dest_row[x] | src_row[x];
                    }
        
                    src_offset += src_step;
                    dest_offset += dest_step;
                }        
            },
            BlitOperation::Keyed(transparency) => {
                for _ in 0..source_rect.height {
                    let src_row = &self.bitmap[src_offset..src_offset + width];
                    let dest_row = &mut destination.bitmap[dest_offset..dest_offset + width];
        
                    for x in 0 .. width {
                        if src_row[x] != transparency {
                            dest_row[x] = src_row[x];
                        }
                    }
        
                    src_offset += src_step;
                    dest_offset += dest_step;
                }        
            },
            BlitOperation::Direct => {
                for _ in 0..source_rect.height {
                    let src_row = &self.bitmap[src_offset..src_offset + width];
                    let dest_row = &mut destination.bitmap[dest_offset..dest_offset + width];
        
                    dest_row.copy_from_slice(src_row);
        
                    src_offset += src_step;
                    dest_offset += dest_step;
                }        
            },
        }
    }
}

/// Easy code for drawing/erasing an image on the screen with transparency
pub struct Sprite {
    /// Holds the screen as it was before drawing (to clean up after ourselves)
    history: RawBitmap,
    /// Graphics to be drawn for this sprite
    image: RawBitmap,
    /// The mask used for transparency.
    mask: Option<RawBitmap>
}

impl Sprite {
    pub fn new(rect: Rect, image: RawBitmap, mask: Option<RawBitmap>) -> Self {
        Self {
            image,
            mask,
            history: RawBitmap::new_blank(rect)
        }
    }

    pub fn draw(&mut self, surface: &mut RawBitmap, point: Point) {
        self.history.rect.x = point.x;
        self.history.rect.y = point.y;
        let history_point = Point::new(0, 0);

        // Capture what the destination looked like before drawing
        surface.blit(self.history.rect, &mut self.history, history_point, BlitOperation::Direct);

        if let Some(mask) = &self.mask {
            // Draw using a mask for transparency
            mask.blit(self.image.rect, surface, point, BlitOperation::And);
            self.image.blit(self.image.rect, surface, point, BlitOperation::Or);
        } else {
            // Draw using an indexed colour (fixed to 0 at the moment)
            self.image.blit(self.image.rect, surface, point, BlitOperation::Keyed(255));
        }
    }

    pub fn erase(&self, surface: &mut RawBitmap) {
        let dest_point = Point::new(self.history.rect.x, self.history.rect.y);
        self.history.blit(self.image.rect, surface, dest_point, BlitOperation::Direct);
    }

    pub fn width(&self) -> i32 {
        self.image.rect.width
    }

    pub fn height(&self) -> i32 {
        self.image.rect.height
    }

    pub fn image_rect(&self) -> Rect {
        self.image.rect
    }
}
