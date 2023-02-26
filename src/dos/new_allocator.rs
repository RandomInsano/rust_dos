//! 

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;

use crate::dos::error_code::ErrorCode;
use crate::dos::misc;

pub struct DosAllocator {
    program_segment_offset: u16,
}

impl DosAllocator {
    const fn new() -> Self {
        Self {
            program_segment_offset: 0x01dd
        }
    }

    /// Prep the allocator for use
    pub fn init(&mut self) {
        // We need to free up the memory DOS gave us so we can ask it for more.
        // It's a little weird, but it's for backward compatibility.

        // The data we need is stored in the Program Segment Prefix structure

        let mut debug: u16;
        let mem_available: u16;
        let mut result: u16;
        let mut error: u16;

        unsafe {
            asm!(                
                // This will store the last data segment that our program is
                // using (the second field of the PSP) into BX and the segment
                // start of our program into ES
                "push es",
                "mov ah, 0x62",
                "int 0x21",             // Get PSP addres in BX
                "mov es, bx",
                "mov cx, es:[0x01]",    // Next segment address after program
                "mov di, bx",           // Save this for later
                "sub bx, cx",
                
                // Resize memory now where BX has been set to the last paragraph
                // of program memory and ES is set to the beginning of the
                // program
                "mov ah, 0x4a",
                "int 0x21",             // Resize memory allocation
                "setc dl",
                "movzx cx, dl",

                "pop es",

                out("cx") result,
                out("ax") error,
                out("di") self.program_segment_offset
            );

            println!("Init?");
            println!("Blah: {:04X}", self.program_segment_offset);

            if result == 1 {
                panic!("Failed to free memory for allocator use. Error {:02x}", error);
            }
        };
    }
}

unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let max_block: u16;
        let result: u16;
        let error_or_block: u16;
        let code_segment: u16;

        // Paragraphs are 16 bytes
        let alloc_size = (layout.size() / 16) + 1;

        asm!(
            "mov ah, 0x48",
            "int 0x21",

            "setc dl",
            "movzx cx, dl",

            "mov di, cs",

            in("bx") alloc_size,
            lateout("bx") max_block,
            lateout("ax") error_or_block,
            lateout("cx") result,
            lateout("di") code_segment,
        );

        if result != 0 {
            panic!("Failed to allocate! Biggest block can be {}. Error was {:?}", max_block, ErrorCode::from_u8(error_or_block as u8));
        }

        // Pointer seems to be at 0:34f0 for some reason
        // It *should* be pointed 0:1720
        let pointer = ((self.program_segment_offset - error_or_block) * 16) as *mut u8;
        let text = "        OH PLEASE FIND THIS TEXT I WOULD REALLY ENJOY THAT";
        let mut buffer = [0u8; 16];

        pointer.copy_to(buffer.as_mut_ptr(), buffer.len());
        print!("Buffer: ");
        for byte in buffer {
            print!("{:02X} ", byte);
        }
        println!("");

        pointer.copy_from(text.as_ptr(), text.len());
        pointer.copy_from([0xde, 0xad, 0xbe, 0xef].as_ptr(), 4);

        println!("Code offset: {:04X}, EOB: {:04X}", code_segment, error_or_block);
        println!("PSP: {:04X}", self.program_segment_offset);
        println!("Alloc [S:{},A:{:04X}]", layout.size(), (self.program_segment_offset - error_or_block) * 16);

        panic!("Dead again!");

        pointer
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        println!("Dealloc");
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let max_block: u16;
        let result: u16;
        let error: u16;

        let alloc_size = (new_size / 16) + 1;
        let old_ptr = ptr;

        println!("Realloc [S:{},A:{:04X}]", new_size, ptr as u32);

        asm!(
            "push es",
            "mov ax, ds",
            "add di, ax",                   // Offset segment ptr

            "mov es, di",                   // Load pointer address into es

            "mov ah, 0x4a",
            "int 0x21",

            "setc dl",
            "movzx cx, dl",

            "pop es",

            in("di") old_ptr as u16, // Paragraphs are 16 byte blocks
            in("bx") alloc_size as u16,
            lateout("bx") max_block,
            lateout("ax") result,
            lateout("cx") error,
        );

        //println!("Ask: {:#?}, Got: {}", layout, result);

        if error != 0 {
            panic!("Failed to re-allocate! Biggest block can be {}. Error was {:?}", max_block, ErrorCode::from_u8(result as u8));
        }

        ptr
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

#[global_allocator]
pub(crate) static mut GLOBAL_ALLOCATOR: DosAllocator = DosAllocator::new();