//! 

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;

use crate::dos::error_code::ErrorCode;
use crate::dos::misc;

pub struct DosAllocator {}

impl DosAllocator {
    const fn new() -> Self {
        Self {}
    }

    /// Prep the allocator for use
    pub fn init(&self) {
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
            );

            if result == 1 {
                panic!("Failed to free memory for allocator use. Error {:02x}", error);
            }
        };
    }
}

unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        print!("Alloc [S:{},", layout.size());
        let max_block: u16;
        let result: u16;
        let error_or_block: u16;
        let data_segment: u16;

        // Paragraphs are 16 bytes
        let alloc_size = (layout.size() / 16) + 1;

        asm!(
            "mov ah, 0x48",
            "int 0x21",

            "setc dl",
            "movzx cx, dl",

            "mov di, ax",
            "mov ax, ds",
            "sub ax, di",                   // Offset segment ptr

            in("bx") alloc_size,
            lateout("bx") max_block,
            lateout("ax") error_or_block,
            lateout("cx") result,
            lateout("di") data_segment,
        );

        //println!("\n\nDS:{:04x}, Loc:{:04x}", data_segment, error_or_block);

        if result != 0 {
            panic!("Failed to allocate! Biggest block can be {}. Error was {:?}", max_block, ErrorCode::from_u8(error_or_block as u8));
        }

        let data_ptr = (error_or_block * 16 - 16) as *mut u8;
        let mut buffer = [0u8; 16];
        data_ptr.copy_to(buffer.as_mut_ptr(), buffer.len());
        data_ptr.copy_from([1, 2, 3, 4, 5, 6, 7, 8, 9, 0].as_ptr(), 10);
        //println!("Buffer: {:?}", buffer);
        //misc::dump_registers();


        print!("A:{:04X}]", error_or_block * 16);
        (error_or_block * 16) as *mut u8
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