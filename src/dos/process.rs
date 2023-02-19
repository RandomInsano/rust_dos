//! Functions related to process management

use core::arch::asm;

use alloc::{
    vec::Vec,
    slice,
    str
};


#[derive(Debug)]
#[repr(C,packed(2))]
pub struct ProgramSegmentPrefix {
    /// Address of INT20 instruction to call on program termination in CP/M
    cpm_terminate_instruction: u16,
    /// First segment after memory allocated to this program
    next_free_segment: u16,
    /// Reserved data
    _reserved1: u8,
    /// TODO: Actually fill this out
    _various1: [u8; 5],
    _various2: u32,
    _various3: u32,
    _various4: u32,
    parent_psp: u16,
    _reserved2: [u8; 20],
    environment_segment: u16,
    /// Stack pointer SS:SP (TODO: Just split this in two?)
    stack_pointer: u32,
    /// Job file table size
    _jft_size: u16,
    /// Job file table pointer
    _jft_pointer: u32,


    /// More junk to pad things out (128 bytes total)
    junk: [u8; 72],
    command_line_len: u8,
    command_line: [u8; 127],

    // More to come
}

impl ProgramSegmentPrefix {
    pub fn environment(&self) -> Vec<(&str, &str)> {
        let mut env_list = Vec::new();
        let mut env_buffer: *mut u8;
        let mut start;
        let mut end;

        let segment = self.environment_segment;
        env_buffer = (segment * 16) as *mut u8;
        
        println!("Environment segment: {}", self.environment_segment);
        println!("Address: {}", segment);
        println!("Current segment: {}", program_segment());
        println!("Command line: {}", self.command_line());
        println!("Junk:");

        for _ in 0 .. 5 {
            for _ in 0 .. 50 {
                unsafe {
                    print!("{}", *env_buffer as char);
                    env_buffer = env_buffer.add(1);
                }
            }

            println!("");
        }

        unsafe {
            start = env_buffer;
            end = env_buffer.add(1);

            while *start != 0 && *end != 0 {
                end = end.add(1);

                if *end == 0 {
                    let diff = end as usize - start as usize;
                    print!("Len: {} ", diff);
                    let string = str::from_utf8(slice::from_raw_parts(start, 3)).unwrap();
                    print!("String: {}", string);
                }
            }
        }

        env_list
    }

    pub fn command_line(&self) -> &str {
        str::from_utf8(&self.command_line[0..self.command_line_len as usize]).unwrap()
    }
}

pub fn program_segment() -> u16 {
    let segment: u16;

    unsafe {
        asm!(
            "mov ah, 0x62",
            "int 0x21",
            out("bx") segment,
        );
    }

    segment
}

/// Read the current Program Segment Prefix for the running program
/// 
/// This function copies the 256 byte structure to the stack to avoid the
/// pointers breaking when the memory segment registers change.
/// 
/// Originally implemented as just a static pointer, when moving around larger
/// parts of the program the pointer would no longer point to valid data. If
/// you know how to fix that, please submit a PR!
pub fn current_psp() -> ProgramSegmentPrefix {    
    unsafe {
        core::ptr::read(0 as *const ProgramSegmentPrefix)
    }
}