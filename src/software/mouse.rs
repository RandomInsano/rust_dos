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

    /// Show the mouse cursor
    /// 
    /// Note: Graphics may not show cursor until you've uploaded a bitmap
    pub fn cursor_show() {
        unsafe {
            asm!(
                "mov ax, 0x0001",
                "int 0x33"
            );
        }
    }

    /// Hide mouse cursor
    pub fn cursor_hide() {
        unsafe {
            asm!(
                "mov ax, 0x0002",
                "int 0x33"
            );
        }
    }

    /// Set graphics cursor bitmap
    /// 
    /// bitmap is two 16x16 bit arrays. The first is the screen map, the second
    /// is the cursor mask. 
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

                "pop ax",
                "mov es, ax",
                in("bx") x_point,
                in("cx") y_point,
                in("di") segment,
                in("dx") offset,
            )
        }
    }
}
