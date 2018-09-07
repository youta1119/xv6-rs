#![no_std]
//#![no_main]
#![feature(lang_items)]
#![feature(panic_handler)]
//#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
//global_asm!(include_str!("entry.S"));
static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
pub extern "C" fn i386_init() {
    let vga_buffer = (0xf0000000 + 0xb8000) as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}