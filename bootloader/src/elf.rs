pub const ELF_MAGIC: u32 = 0x464C457F;  // "\x7FELF" in little endian

#[repr(C)]
pub struct ElfHeader {
    pub magic: u32,
    pub elf: [u8; 12],
    pub typ: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u32,
    pub phoff: u32, // physical header offset
    pub shoff: u32,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shntsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[repr(C)]
pub struct ProgramHeader {
    pub typ: u32,
    pub off: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub filesz: u32,
    pub memsz: u32,
    pub flags: u32,
    pub align: u32,
}

