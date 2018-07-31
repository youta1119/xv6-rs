#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(global_asm)]
#![feature(asm)]

extern crate x86;

//mod elf;

//use x86::instruction;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.S"));


//const SECTOR_SIZE: usize = 512;
static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
pub extern "C" fn bootmain() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
    //let elf = 0x10000 as *const elf::ElfHeader;// scratch space
    //readseg(elf as u32, SECTOR_SIZE * 8, 0);
    /*if (*elf).e_magic != elf::ELF_MAGIC {
        return;
    }*/
   /* // load each program segment (ignores ph flags)
    let mut ph = (ELFHDR as *mut u8).offset(ELFHDR.e_phoff) as *mut Proghdr;
    let mut eph = ph.offset(ELFHDR.e_phnum);
    while ph < eph
        // p_pa is the load address of this segment (as well
        // as the physical address)
        {
            read_segment(ph.p_pa, ph.p_memsz, ph.p_offset);
            ph = ph.offset(1)
        }
    // call the entry point from the ELF header
    // note: does not return!
    loop {}*/
}

/*
// Read 'count' bytes at 'offset' from kernel into physical address 'pa'.
// Might copy more than asked
#[no_mangle]
pub unsafe extern "C" fn read_segment(pa: u32, count: u32, offset: u32) {
    let end_pa = pa + count;
    // round down to sector boundary
    let begin_pa = pa & !(SECTOR_SIZE - 1);

    // translate from bytes to sectors, and kernel starts at sector 1
    let mut offset = (offset / SECTOR_SIZE) + 1;
    // If this is too slow, we could read lots of sectors at a time.
    // We'd write more to memory than asked, but it doesn't matter --
    // we load in increasing order.
    let iter = begin_pa..end_pa;
    for pa in (begin_pa..end_pa).step_by(SECTOR_SIZE) {
        // Since we haven't enabled paging yet and we're using
        // an identity segment mapping (see boot.S), we can
        // use physical addresses directly.  This won't be the
        // case once JOS enables the MMU.
        read_sector(pa , offset);
        offset += 1;
    };
}

unsafe fn wait_disk() {
    // wait for disk reaady
   // while (inb(0x1f7) & 0xc0) != 0x40 {};
}

unsafe fn read_sector(mut dst: u32, mut offset: u32) {
    /*// wait for disk to be ready
    wait_disk();
    outb(498, 1); // count = 1
    outb(499, offset as u8);
    outb(500, (offset >> 8) as u8);
    outb(501, (offset >> 16) as u8);
    outb(502, ((offset >> 24) | 0xe0) as u8);
    outb(503, 0x20); // cmd 0x20 - read sectors
    // wait for disk to be ready
    wait_disk();
    // read a sector
    insl(496, dst, SECTOR_SIZE / 4);*/
}
*/
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
