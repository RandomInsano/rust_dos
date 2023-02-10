//! Microsoft Mouse Interrupt Service Routines
//! ======================================================================
//! 
//! This covers ISRs for 0x33! Support is outlined in the table below
//! 
//! Implementation progress:
//! 
//! | Op   | Function call                                           | Status |
//! |------|---------------------------------------------------------|--------|
//! | 00   | [Reset and read status](Mouse::initialize)              |   ✓    |
//! | 01   | [Show mouse cursor](Mouse::cursor_show)                 |   ✓    |
//! | 02   | [Hide mouse cursor](Mouse::cursor_hide)                 |   ✓    |
//! | ...  |                                                         |   ✓    |
//! | 09   | [Set graphics cursor](Mouse::set_graphics_cursor)       |   ✓    |

use core::arch::asm;

use crate::dos::misc::ptr_to_segments;

pub struct Mouse {}

/// Report on the button configuration of the current mouse
// TODO: Bad names 
#[derive(Debug)]
pub enum MouseButtons {
    Two = 0xffff,
    NotTwo = 0x0000,
    Logitech = 0x003,
    Unknown,
}

impl Mouse {
    /// Reset and read the mouse status. An error result means the driver is
    /// not installed
    pub fn initialize() -> Result<MouseButtons, ()> {
        let status: u16;
        let buttons: u16;

        unsafe {
            asm!(
                "mov ax, 0x0000",
                "int 0x33",
                lateout("ax") status,
                out("bx") buttons,
            );
        }

        if status == 0 {
            return Err(())
        }

        Ok(match buttons {
            0xffff => MouseButtons::Two,
            0x0000 => MouseButtons::NotTwo,
            0x0003 => MouseButtons::Logitech,
            _ => MouseButtons::Unknown,
        })
    }

    fn helper(function: u16, cx: u16, dx: u16) {
        unsafe {
            asm!(
                "int 0x33",
                in("ax") function,
                in("cx") cx,
                in("dx") dx,
            );
        }
    }

    /// Show the mouse cursor
    /// 
    /// Note: Graphics may not show while in a VESA resolution
    pub fn cursor_show() {
        Self::helper(0x0001, 0, 0);
    }

    /// Hide mouse cursor
    pub fn cursor_hide() {
        Self::helper(0x0002, 0, 0);
    }

    /// Define horizontal and horizontal range of cursor
    /// 
    /// Example:
    /// 
    /// ```
    ///     // Make a simple pong-style cursor in CGA mode
    ///     video::set_video(VideoMode::Graphics320_200C2);
    ///
    ///     Mouse::initialize().expect("Microsoft Mouse® driver not loaded");
    /// 
    ///     Mouse::cursor_show();
    ///     Mouse::set_range_vertical(180, 180);
    ///     Mouse::set_range_horizontal(0, 606);
    ///     Mouse::set_graphics_cursor(0, 0, &[
    ///         [0, 0, 0, 0, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff],
    ///         [0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ///     ]);
    ///
    /// ``` 
    pub fn set_range_horizontal(x_min: u16, x_max: u16) {
        Self::helper(0x0007, x_min, x_max);
    }

    /// Define virtical and horizontal range of cursor
    pub fn set_range_vertical(y_min: u16, y_max: u16) {
        Self::helper(0x0008, y_min, y_max);
    }


    /// Set graphics cursor bitmap
    /// 
    /// bitmap is two 16x16 bit arrays. The first is the screen map, the second
    /// is the cursor mask. 
    /// 
    /// Example:
    /// ```
    /// video::set_video(VideoMode::Graphics640x480C4);
    /// 
    /// let result = Mouse::initialize();
    /// println!("Mouse mode: {:?}", result);
    ///
    /// Mouse::set_graphics_cursor(0, 0, &[
    ///     [
    ///         0b0000001111111111,
    ///         0b0000001111111111,
    ///         0b0000001111111111,
    ///         0b0000011111111111,
    ///         0b0000111111111111,
    ///         0b0001111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///         0b1111111111111111,
    ///     ],
    ///     [
    ///         0b0000000000000000,
    ///         0b0111100000000000,
    ///         0b0100000000000000,
    ///         0b0100000000000000,
    ///         0b0100000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///         0b0000000000000000,
    ///     ]
    /// ]);
    ///
    ///     Mouse::cursor_show();
    /// ```
    pub fn set_graphics_cursor(x_point: u16, y_point: u16, bitmap: &[[u16; 16]; 2]) {
        
        let (segment, offset) = ptr_to_segments(bitmap.as_ptr() as u32);

        unsafe {
            asm!(
                "mov ax, es",   // Stash ES register and change it to point to
                "push ax",      // an offset from CS
                "mov ax, cs",
                "add ax, di",
                "mov es, ax",

                "mov ax, 0x0009",
                "int 0x33",

                "pop ax",       // Restore ES from the stack
                "mov es, ax",
                in("bx") x_point,
                in("cx") y_point,
                in("di") segment,
                in("dx") offset,
            )
        }
    }
}
