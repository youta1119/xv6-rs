#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(global_asm)]
#![feature(asm)]

extern crate x86;

mod elf;
use elf::{ElfHeader, ProgramHeader, ELF_MAGIC};
use x86::instruction::{inb, insl, outb};
use core::panic::PanicInfo;

global_asm!(include_str!("boot.S"));

const SECTOR_SIZE: u32 = 512;

#[no_mangle]
pub unsafe extern "C" fn bootmain() {
    let elf_header = 0x10000 as *const ElfHeader;// scratch space
    read_segment(elf_header as u32, SECTOR_SIZE * 8, 0);
    if (*elf_header).magic != ELF_MAGIC {
        //outw(0x8A00, 0x8A00);
        //outw(0x8A00, 0x8E00);
        return;
    }
    // load each program segment (ignores ph flags)
    let mut ph = ((elf_header as *const u8).offset((*elf_header).phoff as isize)) as *const ProgramHeader;
    let eph = ph.offset((*elf_header).phnum as isize);
    while ph < eph {
        read_segment((*ph).paddr, (*ph).memsz, (*ph).off);
        ph = ((ph as u32) + 32) as *const ProgramHeader; //ph.offset(1);
    }
    // call the entry point from the ELF header
    // note: does not return!
    let entry: extern "C" fn() -> ! = core::mem::transmute((*elf_header).entry);
    entry();
}

// Read 'count' bytes at 'offset' from kernel into physical address 'pa'.
// Might copy more than asked
#[inline(never)]
unsafe fn read_segment(pa: u32, count: u32, offset: u32) {
    let epa = pa + count;
    // round down to sector boundary
    let mut bpa = pa & !(SECTOR_SIZE - 1);

    // translate from bytes to sectors, and kernel starts at sector 1
    let mut offset = (offset / SECTOR_SIZE) + 1;
    // If this is too slow, we could read lots of sectors at a time.
    // We'd write more to memory than asked, but it doesn't matter --
    // we load in increasing order.
    while bpa < epa {
        // Since we haven't enabled paging yet and we're using
        // an identity segment mapping (see boot.S), we can
        // use physical addresses directly.  This won't be the
        // case once JOS enables the MMU.
        read_sector(bpa , offset);
        offset += 1;
        bpa += SECTOR_SIZE;
    }
}

unsafe fn wait_disk() {
    // wait for disk reaady
    while (inb(0x1F7) & 0xC0) != 0x40 {};
}

unsafe fn read_sector(dst: u32, offset: u32) {
    // wait for disk to be ready
    wait_disk();
    outb(0x1F2, 1); // count = 1
    outb(0x1F3, offset as u8);
    outb(0x1F4, (offset >> 8) as u8);
    outb(0x1F5, (offset >> 16) as u8);
    outb(0x1F6, ((offset >> 24) | 0xE0) as u8);
    outb(0x1F7, 0x20); // cmd 0x20 - read sectors
    // wait for disk to be ready
    wait_disk();
    // read a sector
    insl(0x1F0, dst, SECTOR_SIZE / 4);
}

#[panic_implementation]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}
