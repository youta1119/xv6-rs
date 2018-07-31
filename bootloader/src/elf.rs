pub const ELF_MAGIC: u32 = 0x464C457F;  // "\x7FELF" in little endian

#[repr(C)]
pub struct ElfHeader {
    pub e_magic: u32,
    pub e_elf: [u8; 12],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_va: u32,
    pub p_pa: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}