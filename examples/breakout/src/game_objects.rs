use crate::graphics::{
    BlitOperation,
    Point,
    RawBitmap,
    Rect,
};

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
        surface.blit(self.history.rect, &mut self.history, history_point, BlitOperation::Direct);

        if let Some(mask) = &self.mask {
            mask.blit(self.image.rect, surface, point, BlitOperation::And);
            self.image.blit(self.image.rect, surface, point, BlitOperation::Or);
        } else {
            self.image.blit(self.image.rect, surface, point, BlitOperation::Keyed(0));
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

