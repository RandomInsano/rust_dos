use alloc::vec::Vec;
use image_viewer::bitmap::{BitmapColourEntry, Bitmap};
use rust_dos::bios::video::VgaDacColour;
use rust_dos::dos::error_code::ErrorCode;
use rust_dos::{
    bios::video::get_vga_pointer,
    print,
    println
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

pub fn display_image(bitmap: &RawBitmap) -> Result<(), ErrorCode> {
    let screen_memory = get_vga_pointer();
    let dimensions = bitmap.rect;
    let bitmap = &bitmap.bitmap;
    
    if dimensions.width != 320 {
        println!("Image dimensions for splash screen are wrong. Exiting");
        return Err(ErrorCode::InvalidFormat)
    }

    // Copy image data to the video card
    unsafe {
        let mut image_pointer = bitmap.as_ptr();
        let mut screen_pointer = screen_memory.clone();

        for _ in 0 .. dimensions.height {
            screen_pointer.copy_from(image_pointer, 320);

            screen_pointer = screen_pointer.add(320);
            image_pointer = image_pointer.add(320)
        }
    }

    Ok(())
}

#[derive(Copy, Clone)]
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

    pub fn location(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

/// Just a block of memory with a width and a height than can blit between
/// instances of itself
pub struct RawBitmap {
    pub(crate) rect: Rect,
    pub(crate) bitmap: &[u8]
}

pub enum BlitOperation {
    And,
    Or,
    Direct,
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

        RawBitmap::new(rect, Vec::from(buffer))
    }
}

impl RawBitmap {
    pub fn new(rect: Rect, bitmap: Vec<u8>) -> Self {
        Self {
            rect,
            bitmap,
        }
    }

    pub fn new_blank(buffer_rect: Rect) -> Self {
        let mut buffer_data = Vec::new();
        buffer_data.resize((buffer_rect.width * buffer_rect.height) as usize, 0u8);
        RawBitmap::new(buffer_rect, buffer_data)
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
