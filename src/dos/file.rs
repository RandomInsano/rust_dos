use bitflags::bitflags;
use core::arch::asm;
use core::cmp::min;
use crate::dos::error_code::ErrorCode;

use super::datetime::{Date, Time};

extern crate rlibc;

#[allow(dead_code)]
pub struct File {
    handle: u16,
}
#[allow(dead_code)]
pub enum SeekFrom {
    Start(u32),
    End(u32),
    Current(u32),
}

impl SeekFrom {
    fn to_dos_seek_code(&self) -> u8 {
        match self {
            SeekFrom::Start(_) => 0,
            SeekFrom::End(_) => 2,
            SeekFrom::Current(_) => 1,
        }
    }

    fn to_seek_offset(&self) -> u32 {
        match self {
            SeekFrom::Start(offset) => *offset,
            SeekFrom::End(offset) => *offset,
            SeekFrom::Current(offset) => *offset,
        }
    }
}

bitflags! {
    pub struct FileAttributes: u16 {
        const SHARABLE     = 1 << 7;
        const RESERVED     = 1 << 6;
        const ARCHIVE      = 1 << 5;
        const DIRECTORY    = 1 << 4;
        const VOLUME_LABEL = 1 << 3;
        const SYSTEM       = 1 << 2;
        const HIDDEN       = 1 << 1;
        const READ_ONLY    = 1;
    }
}


/// Most operations on files and folders are similar except that the interrupt
/// routine differs. This abstracts all of the common code in one spot for
/// easier usage and maintenance
/// 
/// Returns (ax, cx) registers or an ErrorCode
pub fn file_folder_helper(filename: &str, mode: u8, operation: u8) -> Result<(u16, u16), ErrorCode> {
    let mut error_result: u8;
    let mut error_code: u16;
    let mut result: u16;

    // DOS PATH length limit is 66 bytes.
    let mut filename_array: [u8; 70] = [0; 70]; // To be sure of the segment
    for i in 0..min(filename_array.len(), filename.len()) {
        filename_array[i] = filename.as_bytes()[i];
    }
    let filename_ptr = filename_array.as_ptr();
    unsafe {
        asm!(
            "int 0x21",
            "setc dl",
            "movzx cx, dl",
            in("ah") operation,
            in("al") mode,
            in("dx") filename_ptr as u16,
            lateout("dl") error_result,
            lateout("ax") error_code,
            lateout("cx") result);
    }

    if error_result != 0 {
        return Err(ErrorCode::from_u8(error_code as u8).unwrap_or(ErrorCode::UnknownError));
    }

    Ok((error_code, result))
}

#[allow(dead_code)]
#[allow(unused_assignments)]
impl File {
    pub fn open(filename: &str) -> Result<Self, ErrorCode> {
        let mode = 0x40; // Access and sharing modes? Not sure what this is yet
        let (handle, _) = file_folder_helper(filename, mode, 0x3d)?;
        
        Ok(Self {
            handle,
        })
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, ErrorCode> {
        let mut total_bytes_read: usize = 0;
        for buffer_write_pos in 0..buffer.len() {
            let mut is_read_success: u16 = 1; // 0: success, 1: fail
            let mut error_code_or_bytes_read: u16 = 0;
            let mut tmp_stack_buffer: [u8; 1] = [0; 1]; // To be sure of the segment
            let tmp_buffer_ptr = tmp_stack_buffer.as_mut_ptr();
            unsafe {
                asm!("push ax", "push bx", "push cx", "push dx");
                asm!("mov cx, 1", "mov ah, 0x3f", "int 0x21", "setc  dl", "movzx cx, dl", in("bx") self.handle, in("dx") tmp_buffer_ptr, lateout("cx") is_read_success, lateout("ax") error_code_or_bytes_read);
                asm!("pop dx", "pop cx", "pop bx", "pop ax");
            }
            if is_read_success == 1 {
                return Err(ErrorCode::from_u8(error_code_or_bytes_read as u8).unwrap_or(ErrorCode::UnknownError));
            }
            if error_code_or_bytes_read == 0 {
                // End of file
                break;
            }
            //total_bytes_read += error_code_or_bytes_read as usize; // = 1
            total_bytes_read += 1 as usize;
            buffer[buffer_write_pos] = tmp_stack_buffer[0];
        }

        // fill the rest of the buffer with 0s
        for buffer_write_pos in total_bytes_read..buffer.len() {
            buffer[buffer_write_pos] = 0;
        }
        Ok(total_bytes_read)
    }

    // TODO check
    pub fn write(&self, buffer: &[u8]) -> Result<usize, ErrorCode> {
        let mut total_bytes_written: usize = 0;
        for buffer_read_pos in 0..buffer.len() {
            let mut is_write_success: u16 = 1; // 0: success, 1: fail
            let mut error_code_or_bytes_written: u16 = 0;
            let mut tmp_stack_buffer: [u8; 1] = [0; 1]; // To be sure of the segment
            tmp_stack_buffer[0] = buffer[buffer_read_pos];
            let tmp_buffer_ptr = tmp_stack_buffer.as_ptr();
            unsafe {
                asm!("push ax", "push bx", "push cx", "push dx");
                asm!("mov cx, 1", "mov ah, 0x40", "int 0x21", "setc  dl", "movzx cx, dl", in("bx") self.handle, in("dx") tmp_buffer_ptr, lateout("cx") is_write_success, lateout("ax") error_code_or_bytes_written);
                asm!("pop dx", "pop cx", "pop bx", "pop ax");
            }
            if is_write_success == 1 {
                return Err(ErrorCode::from_u8(error_code_or_bytes_written as u8).unwrap_or(ErrorCode::UnknownError));
            }
            //total_bytes_written += error_code_or_bytes_written as usize; // = 1
            total_bytes_written += 1 as usize;
        }
        Ok(total_bytes_written)
    }

    pub fn close(self) -> Result<(), ErrorCode> {
        self.close_with_ref()
    }

    fn close_with_ref(&self) -> Result<(), ErrorCode> {
        let mut is_close_success: u16 = 1; // 0: success, 1: fail
        let mut error_code: u16 = 0; // 6 = unknown handle
        unsafe {
            asm!("push ax", "push bx", "push cx", "push dx");
            asm!("mov ah, 0x3e", "int 0x21", "setc  dl", "movzx cx, dl", in("bx") self.handle, lateout("cx") is_close_success, lateout("ax") error_code);
            asm!("pop dx", "pop cx", "pop bx", "pop ax");
        }
        if is_close_success == 1 {
            return Err(ErrorCode::from_u8(error_code as u8).unwrap_or(ErrorCode::UnknownError));
        }
        Ok(())
    }

    /// Seek to an offset, in bytes, in a stream.
    /// Returns number of bytes from the start of the stream if success, or an error code otherwise.
    pub fn seek(&self, pos: SeekFrom) -> Result<u32, ErrorCode> {
        let mut is_seek_success: u16 = 1; // 0: success, 1: fail
        let mut error_code_or_new_pos_low_from_start: u16 = 0;
        let mut new_pos_high_from_start: u16 = 0;
        let requested_relative_new_pos: u32 = pos.to_seek_offset();
        let requested_relative_new_pos_low = (requested_relative_new_pos & 0xffff) as u16;
        let requested_relative_new_pos_high = ((requested_relative_new_pos >> 16) & 0xffff) as u16;
        let seek_from: u8 = pos.to_dos_seek_code();
        unsafe {
            asm!("push ax", "push bx", "push cx", "push dx");
            asm!("mov ah, 0x42", "int 0x21", "setc  dl", "movzx cx, dl", in("bx") self.handle, in("cx") requested_relative_new_pos_high as u16, in("dx") requested_relative_new_pos_low, in("al") seek_from, lateout("cx") is_seek_success, lateout("ax") error_code_or_new_pos_low_from_start, lateout("dx") new_pos_high_from_start);
            asm!("pop dx", "pop cx", "pop bx", "pop ax");
        }
        if is_seek_success == 1 {
            return Err(ErrorCode::from_u8(error_code_or_new_pos_low_from_start as u8).unwrap_or(ErrorCode::UnknownError));
        }
        Ok((new_pos_high_from_start as u32) << 16 | (error_code_or_new_pos_low_from_start as u32))
    }

    pub fn attributes(filename: &str) -> Result<FileAttributes, ErrorCode> {
        let (_, attributes) = file_folder_helper(filename,  0x00, 0x43)?;
        Ok(FileAttributes::from_bits_truncate(attributes))
    }

    pub fn last_write(&self) -> Result<(Date, Time), ErrorCode> {
        let mut date = Date::default();
        let mut time = Time::default();
        let date_value: u16;
        let time_value: u16;
        let error_result: u8;
        let error_code: u16;

        unsafe {
            asm!(
                "mov al, 0x00",
                "mov ah, 0x57",
                "int 0x21",
                "setc bh",
                in ("bx") self.handle,
                out ("ax") error_code,
                lateout ("bh") error_result,
                out ("cx") time_value,
                out ("dx") date_value,
            );
        }

        time.second = ((time_value & 0b00000000_00011111) >> 0) as u8 * 2;
        time.minute = ((time_value & 0b00000111_11100000) >> 5) as u8;
        time.hour =   ((time_value & 0b11111000_00011111) >> 11) as u8;

        date.day   = ((date_value & 0b00000000_00011111) >> 0) as u8;
        date.month = ((date_value & 0b00000001_11100000) >> 5) as u8;
        date.year  = ((date_value & 0b11111110_00000000) >> 9) + 1980;

        if error_result != 0 {
            return Err(ErrorCode::from_u8(error_code as u8).unwrap_or(ErrorCode::UnknownError));
        }

        Ok((date, time))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = self.close_with_ref();
    }
}

pub struct Directory {}

impl Directory {
    pub fn make(path: &str) -> Result<(), ErrorCode> {
        file_folder_helper(path, 0x00, 0x39)?;

        Ok(())
    }

    pub fn change_current(path: &str) -> Result<(), ErrorCode> {
        file_folder_helper(path, 0x00, 0x3b)?;

        Ok(())
    }

    pub fn remove(path: &str) -> Result<(), ErrorCode> {
        file_folder_helper(path, 0x00, 0x3a)?;

        Ok(())
    }
}