extern crate alloc;

use rust_dos::dos::error_code::ErrorCode;
use rust_dos::dos::file::{File, SeekFrom, AccessMode};
use core::mem;
use alloc::vec::Vec;

#[derive(Debug)]
#[repr(C, packed)]
pub struct BitmapFileHeader {
    id: [u8; 2],
    size: u32,
    _reserved: [u8; 4],
    image_offset: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct BitmapInfoHeader {
    header_size: u32,
    width: u32,
    height: u32,
    colour_planes: u16,
    bpp: u16,
    compression: u32,
    image_size: u32,
    pixel_width_per_meter: u32,
    pixel_height_per_metere: u32,
    pallette_colour_count: u32,
    important_colour_count: u32,
}

impl BitmapInfoHeader {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bpp(&self) -> u16 {
        self.bpp
    }

    pub fn compression(&self) -> u32 {
        self.compression
    }
}

#[repr(C, packed)]
/// Colours for each index
pub struct BitmapColourEntry {
    blue: u8,
    green: u8,
    red: u8,
    _reserved: u8
}

impl BitmapColourEntry {
    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn red(&self) -> u8 {
        self.red
    }
}

/// In-memory bitmap. Note that this does not follow the Microsoft spec for
/// a device-independent bitmap because parts aren't contiguous in memory
pub struct Bitmap {
    header: BitmapInfoHeader,
    palette: Vec<BitmapColourEntry>,
    image_data: Vec<u8>,
}

impl Bitmap {
    pub fn load(filename: &str) -> Result<Self, ErrorCode> {
        let file_header: BitmapFileHeader;
        let bitmap_header: BitmapInfoHeader;
        let mut palette: Vec<BitmapColourEntry> = Vec::new();

        let file_handle = File::open(filename, AccessMode::default()).unwrap();

        let mut buffer = [0u8; mem::size_of::<BitmapFileHeader>()];
        file_handle.read(&mut buffer)?;
        file_header = unsafe {
            mem::transmute(buffer)
        };

        assert_eq!(file_header.id, [0x42, 0x4d]);

        let mut buffer = [0u8; mem::size_of::<BitmapInfoHeader>()];
        file_handle.read(&mut buffer)?;
        bitmap_header = unsafe {
            mem::transmute(buffer)
        };

        let palette_data_offset = mem::size_of::<BitmapFileHeader>() as u32 + bitmap_header.header_size;
        file_handle.seek(SeekFrom::Start(palette_data_offset))?;

        if bitmap_header.bpp <= 8 {
            let mut buffer = [0u8; mem::size_of::<BitmapColourEntry>()];

            for _ in 0 .. bitmap_header.pallette_colour_count {
                file_handle.read(&mut buffer)?;
                palette.push(unsafe {
                    mem::transmute(buffer)
                });
            }
        }

        // Jump to the start of image data
        file_handle.seek(SeekFrom::Start(file_header.image_offset)).unwrap();

        let mut image_data = Vec::new();
        image_data.resize(bitmap_header.image_size as usize, 0u8);
        file_handle.read(&mut image_data).unwrap();

        Ok(Self{
            header: bitmap_header,
            palette,
            image_data
        })
    }

    pub fn header(&self) -> &BitmapInfoHeader {
        &self.header
    }

    pub fn palette(&self) -> &[BitmapColourEntry] {
        &self.palette
    }

    pub fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}
