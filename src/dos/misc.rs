use core::arch::asm;

#[derive(Default)]
pub struct VersionInfo {
    major: u8,
    minor: u8,
    flags: u8,
}

impl VersionInfo {
    /// Whether or not MS-DOS is running from ROM. DOS older than 5.00 only
    pub fn in_rom(&self) -> bool {
        self.major < 5 && self.flags & 0b00000100 != 0
    }
}

impl core::fmt::Debug for VersionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{:0>2}", self.major, self.minor)?;

        if self.in_rom() {
            write!(f, "(in ROM)")?;
        }

        Ok(())
    }
}

pub fn ptr_to_segments(value: u32) -> (u16, u16) {
    let segment = value / 16;
    let offset = value & 0xf;

    (segment as u16, offset as u16)
}

pub fn dump_registers() {
    let mut ax: u16;
    let mut bx: u16;
    let mut cx: u16;
    let mut di: u16;
    let mut si: u16;

    unsafe {
        asm!(
            "and ax, ax",
            out("ax") ax,
            out("bx") bx,
            out("cx") cx,
            out("di") di,
        )
    }

    println!("AX:{:02x} BX:{:02x} CX:{:02x} DI:{:02x}",
        ax,
        bx,
        cx,
        di,
    );

    unsafe {
        asm!(
            "mov ax, ds",
            "mov bx, cs",
            "mov cx, ss",
            "mov di, es",
            out("ax") ax,
            out("bx") bx,
            out("cx") cx,
            out("di") di,
        )
    }

    println!("DS:{:02x} CS:{:02x} SS:{:02x} ES:{:02x}",
        ax,
        bx,
        cx,
        di,
    );
}

pub fn dos_version() -> VersionInfo {
    let mut version_info = VersionInfo::default();

    unsafe {
        // Push for bh is from a reference that "bh" is preserved. If it doesn't
        // need to be stored, please open a PR
        asm!("mov ch, bh",
            "mov ah, 0x30",
            "int 0x21",
            "mov bh, ch",
            out("al") version_info.major,
            out("ah") version_info.minor,
            out("bh") version_info.flags
        );
    }
    
    version_info
}
