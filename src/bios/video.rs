//! Video BIOS Interrupt Service Routines
//! ======================================================================
//! 
//! These are the functions for BIOS interrupt 10h functionality (video
//! services).
//! 
//! Implementation progress:
//! 
//! | Op   | Function call                                           | Status |
//! |------|---------------------------------------------------------|--------|
//! | 00   | [Set video mode](set_video) (missing VGA modes)         | Partly |
//! | 01   | [Set cursor size](set_cursor_size)                      |   ✔️    |
//! | 02   | [Set cursor position](set_cursor_position)              |   ✔️    |
//! | 03   | Get cursor position                                     |        |
//! | 04   | Not available                                           |  N/A   |
//! | 05   | [Change displayed page](set_page)                       |   ✔️    |
//! | 06   | Scroll active page up                                   |        |
//! | 07   | Scroll active page down                                 |        |
//! | 08   | Read character attribute                                |        |
//! | 09   | Write character attribute                               |        |
//! | 0A   | Write character at cursor                               |        |
//! | 0B   | [Set colour palette](set_cga_palette)                   |   ✔️    |
//! | 0C   | [Write graphics pixel](set_pixel)                       |   ✔️    |
//! | 0D   | [Read graphics pixel](pixel)                            |   ✔️    |
//! | 0E   | Teleytype write character                               |        |
//! | 0F   | Read current video parameters                           |        |
//! | 1012 | [Set VGA DAC registers](set_vga_dac)                    |        |
//! 
//! ## VESA Extensions
//! 
//! | Op   | Function call                                           | Status |
//! |------|---------------------------------------------------------|--------|
//! | 4f00 | Get VBE controller information                          |        |  
//! | 4f01 | Get VBE mode information                                |        |  
//! | 4f02 | [Set VBE mode](set_video_vesa)                          |   ~    |  
//! | 4f03 | Get VBE mode                                            |        |  
//! | 4f04 | Save or restore state                                   |        |  
//! | 4f05 | Display window control                                  |        |  
//! | 4f06 | Get or set logical scan line length                     |        |  
//! | 4f07 | Get or set display start                                |        |  
//! | 4f08 | Get or set DAC palette format                           |        |  
//! | 4f09 | Get or set DAC palette data                             |        |  
//! | 4f0A | Return VBE protected mode interface                     |        |  
//! | 4f0B | Get or set pixel clock                                  |        |  
//! 
//! References: 
//! * [BIOS Video Modes](https://www.minuszerodegrees.net/video/bios_video_modes.htm)
//! * [VESA BIOS Extensions (Wikipedia)](https://wiki.osdev.org/VESA_Video_Modes)
//! * [VESA Video Modes (OS Dev)](https://wiki.osdev.org/VESA_Video_Modes)
//! * [VESA Tutorial (OS Dev)](https://wiki.osdev.org/User:Omarrx024/VESA_Tutorial)
//! * [VESA Standard 3.0](https://pdos.csail.mit.edu/6.828/2018/readings/hardware/vbe3.pdf)


use core::arch::asm;

use crate::dos::misc;

/// Various video modes that can be sent to [set_video]. Check that function
/// for usage and warnings.
pub enum VideoMode {
    /// Text mode 40x25, 16 shades of grey
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Text40_25B = 0x00,

    /// Text mode 40x25, 16 colours
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Text40_25C = 0x01,

    /// Text mode 80x25, 16 shades of grey
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Text80_25B = 0x02,

    /// Text mode 80x25, 16 colours
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Text80_25C = 0x03,

    /// Graphics mode 320x200, 4 colours
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Graphics320_200C2 = 0x04,

    /// Graphics mode 320x200, 4 shades of grey
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Graphics320_200B = 0x05,

    /// Graphics mode 640x200, 2 shades of grey
    /// 
    /// Supported adapters: CGA, PCjr, EGA, MCGA, VGA
    Graphics640_200B = 0x06,

    /// Text mode 80x25, 2 shades of grey
    /// 
    /// Supported adapters: MDA
    Text20x25B2 = 0x07,

    /// Graphics mode 640x400, 16 colours
    ///  
    /// Supported adapters: VGA
    Graphics640x480C4 = 0x12,

    /// Graphics mode 320x200, 16 colours
    /// 
    /// Supported adapters: CGA, EGA, VGA
    Graphics320_200C4 = 0x0d,

    /// Graphics mode 320x200, 256 colours
    /// 
    /// Supported adapters: VGA
    Graphics320_200C8 = 0x13,
}

#[repr(C)]
pub struct VesaMode {
    mode: u16,
}

#[derive(Debug)]
pub enum VesaReturnStatus {
    /// Function is supported
    Supported = 0x4f,
    /// Function call successful
    Successful = 0x00,
    /// Function call failed
    Failed = 0x01,
    /// Function not supported in current mode
    ModeFailure = 0x02,
    /// Function call invalide in current mode
    ModeInvalid = 0x03,
    /// Unknown return code
    Unknown = 0xffff,
}

impl From<u16> for VesaReturnStatus {
    fn from(value: u16) -> Self {
        match value {
            0x4f => Self::Supported,
            0x00 => Self::Successful,
            0x01 => Self::Failed,
            0x02 => Self::ModeFailure,
            0x03 => Self::ModeInvalid,
            _ => Self::Unknown,
        }
    }
}

impl VesaMode {
    pub fn new(mode: u16, custom_refresh_rate: bool, linear_memory: bool, preserve_buffers: bool) -> Self {
        let mut value = VesaMode {
            mode: 0,
        };

        value.set_mode(mode);

        if custom_refresh_rate {
            value.mode |= 1 << 11;
        }

        if linear_memory {
            value.mode |= 1 << 14;
        }

        if preserve_buffers {
            value.mode |= 1 << 15;
        }

        value
    }

    pub fn set_mode(&mut self, mode: u16) {
        self.mode |= mode & 0x01FF;
    }
}

#[repr(C)]
pub enum ColorTarget {
    Background = 0,
    Palette = 1,   
}

pub struct VgaDacColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

/// Set the current video mode.
/// 
/// Warning: Not all modes properly set up the display or clear buffers so you
/// can render your console unusable. To reset to the default display mode and
/// reset under DOS you can run the `cls` command.
pub fn set_video(mode: VideoMode) {
    unsafe {
        asm!("mov ah, 0x00",
            "int 0x10",
            in("al") mode as u8
        );
    }
}

/// Implements basic VESA mode setting. Extended attributes are not possibe at
/// the moment. 
pub fn set_video_vesa(mode: VesaMode) -> Result<(), VesaReturnStatus> {
    let result: u16;

    unsafe {
        asm!("mov ax, 0x4f02",
            "int 0x10",
            in("bx") mode.mode,
            out("ax") result
        );
    }

    if result == VesaReturnStatus::Supported as u16 {
        return Ok(())
    }

    Err(VesaReturnStatus::from(result))
}

/// Set the current extended VESA video mode

/// Set the cursor size. Only the bottom nibble is used
/// 
/// Note: This will have no effect while in a graphics mode as there's no
/// cursor
pub fn set_cursor_size(top: u8, bottom: u8) {
    unsafe {
        asm!("mov ah, 0x01",
            "int 0x10",
            in("ch") top,
            in("cl") bottom,
        );
    }
}

/// Set the cursor position. Different display modes can have multiple pages
/// that you can swap between.
/// 
/// See also:
/// * [set_page]
pub fn set_cursor_position(page: u8, column: u8, row: u8) {
    unsafe {
        asm!("mov ah, 0x02",
            "int 0x10",
            in("bh") page,
            in("dl") column,
            in("dh") row,
        );
    }
}

/// Set the current display page buffer
pub fn set_page(page: u8) {
    unsafe {
        asm!("mov ah, 0x05",
            "int 0x10",
            in("al") page,
        );
    }
}

// Set colour pallet
pub fn set_cga_palette(target: ColorTarget, color: u8) {
    unsafe {
        asm!("mov ah, 0x0b",
            "int 0x10",
            in("bh") target as u8,
            in("bl") color,
        );
    }
}

// Write graphics pixel
pub fn set_pixel(page: u8, x: u16, y: u16, colour: u8) {
    unsafe {
        asm!("mov ah, 0x0c",
            "int 0x10",
            in("bh") page,
            in("al") colour,
            in("cx") x,
            in("dx") y
        );
    }
}

// Read graphics pixel
pub fn pixel(page: u8, x: u16, y: u16) -> u8 {
    let colour;

    unsafe {
        asm!("mov ah, 0x0d",
            "int 0x10",
            in("bh") page,
            out ("al") colour,
            in("cx") x,
            in("dx") y
        );
    }

    colour
}

// Set all palette registers
pub fn set_vga_dac(colours: &[VgaDacColour], start: u32) {

    let (segment, offset) = misc::ptr_to_segments(colours.as_ptr() as u32);

    unsafe {
        asm!(
            "mov ax, es",
            "push ax",          // Preserve data extra segment register
            "add ax, di",
            "mov es, ax",       // Offset the segment to where our data is

            "mov ax, 0x1012",
            "int 0x10",         // Call interrupt

            "pop ax",           // Restore data segment register
            "mov es, ax",

            in("di") segment,
            in("dx") offset,
            in("bx") start,
            in("cx") colours.len() as u16
        );
    }
}