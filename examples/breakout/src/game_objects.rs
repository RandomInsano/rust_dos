use alloc::vec::Vec;
use crate::graphics::{RawBitmap, Point, BlitOperation, Rect};

pub struct BrickGraphics {
    /// Store a variety of bricks. Broken into left side, right side, and square
    images: Vec<Vec<RawBitmap>>,
    /// The brick to draw
    rand: RandomNumberGenerator,
}

impl BrickGraphics {
    pub fn new(images: Vec<Vec<RawBitmap>>) -> Self {
        Self {
            images,
            rand: RandomNumberGenerator::new(13145, 535)
        }
    }

    // Draw using an indexed colour (fixed to 0 at the moment)
    pub fn draw(&mut self, surface: &mut RawBitmap, point: Point, brick: Brick) {
        // Type of zero is invisible/non-existant so we have no work to do
        if brick.brick_type() == 0 {
            return;
        }

        // Format index 0 is reserved
        let format = (brick.format_raw() - 1) as usize;
        // Same for type. Zero is considered 'none'
        let brick_palette_offset = (brick.brick_type() - 1) * 6;
        // Grab a random number for which half of the brick to draw
        let index = self.rand.next() as usize % self.images.len();

        let image = &self.images[index][format];
        image.blit(image.rect, surface, point, BlitOperation::Keyed(255));

        // Shift colour palette depending on brick type
        let rect = Rect::new(point.x, point.y, image.rect.width, image.rect.height).unwrap();
        surface.shift_colour(rect, brick_palette_offset);
    }
}

pub enum BrickFormat {
    /// Unused
    _Reserved = 0,
    /// The left side of a brick
    Left = 1,
    /// The right side of a brick
    Right = 2,
    /// A standalone half-brick
    Half = 3,
}

/// Represents a brick in the playfield. Attributes are bitpacked into a u8
/// 
/// bit 7   - breakable
/// bit 5-6 - format (see [BrickFormat])
/// bit 0-4 - block type
pub struct Brick {
    data: u8
}

const BRICK_BREAK_BIT: u8 = 0x80;
const BRICK_TYPE_MASK: u8 = 0x1f;
const BRICK_FORMAT_MASK: u8 = 0x60;

impl From<u8> for Brick {
    fn from(value: u8) -> Self {
        Self {
            data: value,
        }
    }
}

impl Brick {
    pub fn new(breakable: bool, format: BrickFormat, brick_type: u8) -> Self {
        let mut value = Self {
            data: 0
        };

        value.set_breakable(breakable);
        value.set_format(format);
        value.set_brick_type(brick_type);

        value
    }

    pub fn breakable(&self) -> bool {
        self.data & BRICK_BREAK_BIT > 0
    }

    pub fn set_breakable(&mut self, value: bool) {
        if value {
            self.data |= BRICK_BREAK_BIT;    
        } else {
            self.data &= !BRICK_BREAK_BIT;
        }
    }

    pub fn format(&self) -> BrickFormat {
        match self.data & BRICK_FORMAT_MASK {
            0x20 => BrickFormat::Left,
            0x40 => BrickFormat::Right,
            0x60 => BrickFormat::Half,
            _ => panic!("Incorrect BrickFormat")
        }
    }

    pub fn format_raw(&self) -> u8 {
        (self.data & BRICK_FORMAT_MASK) >> 5
    }

    pub fn set_format(&mut self, value: BrickFormat) {
        let value = match value {
            BrickFormat::Left => 0x20,
            BrickFormat::Right => 0x40,
            BrickFormat::Half => 0x60,
            BrickFormat::_Reserved => panic!("Used reserved value"),
        };

        self.data &= !BRICK_FORMAT_MASK;
        self.data |= value;
    }

    pub fn brick_type(&self) -> u8 {
        self.data & BRICK_TYPE_MASK
    }

    pub fn set_brick_type(&mut self, value: u8) {
        if value > BRICK_TYPE_MASK {
            panic!("Provided a type value larger than Brick can hold");
        }

        self.data &= !BRICK_TYPE_MASK;
        self.data |= value
    }
}

/// I asked ChatGPT to write me a Permuted Congruential Generator random number
/// generator. I also asked to write a LCG but that didn't work when doing the
/// modulo of smaller numbers like I needed for bricks
/// https://en.wikipedia.org/wiki/Permuted_congruential_generator
struct RandomNumberGenerator {
    state: u64,
    inc: u64,
}

impl RandomNumberGenerator {
    /// Create a new PGC instance
    fn new(seed: u64, seq: u64) -> Self {
        let mut rng = Self { state: 0, inc: (seq << 1) | 1 };
        rng.seed(seed);
        rng
    }

    fn seed(&mut self, seed: u64) {
        self.state = 0;
        self.inc = self.inc
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc)
            .wrapping_add(seed);
    }

    // Generate the next random number
    fn next(&mut self) -> u32 {
        let old_state = self.state;
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(self.inc);
        let xor_shifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xor_shifted >> rot) | (xor_shifted << ((-(rot as i32)) & 31))
    }
}