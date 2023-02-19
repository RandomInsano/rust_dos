//! 

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;

use crate::dos::error_code::ErrorCode;

pub struct DosAllocator {}

impl DosAllocator {
    const fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {
        /*
        unsafe {
            asm!(
                "mov bx, 1FFF",
                "mov cl, 4",
                "shr bx, cl",
                "add bx, 17",
                "mov ah, 0x4a",
                "int 0x21",
                "mov ax, bx",
                "shl ax, cl",
                "dec ax",
                "mov sp, ax",
                "mov bp, sp"
            );
        };
         */
    }
}

unsafe impl GlobalAlloc for DosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        print!("Alloc [{},", layout.size());
        let max_block: u16;
        let result: u16;
        let error: u16;

        // Paragraphs are 16 bytes
        let alloc_size = (layout.size() / 16) + 1;

        asm!(
            "mov ah, 0x48",
            "int 0x21",

            "setc dl",
            "movzx cx, dl",

            in("bx") alloc_size,
            lateout("bx") max_block,
            lateout("ax") result,
            lateout("cx") error,
        );

        if error != 0 {
            panic!("Failed to allocate! Biggest block can be {}. Error was {:?}", max_block, ErrorCode::from_u8(result as u8));
        }

        print!("{}]", result * 16);
        (result * 16) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        println!("Dealloc");
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        println!("Realloc [{},{}]", ptr as u32, layout.size());
        let max_block: u16;
        let result: u16;
        let error: u16;

        let alloc_size = (new_size / 16) + 1;
        let old_ptr = ptr;

        println!("Beep");

        asm!(
            "mov es, di",                      // Load pointer address into es

            "mov ah, 0x4a",
            "int 0x21",

            "setc dl",
            "movzx cx, dl",

            in("di") old_ptr as u16, // Paragraphs are 16 byte blocks
            in("bx") alloc_size as u16,
            lateout("bx") max_block,
            lateout("ax") result,
            lateout("cx") error,
        );
        
        println!("Boop");

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