#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(asm)]
extern crate x86;
use core::panic::PanicInfo;
mod io;
use io::uart::Uart;

//#[macro_use]
//mod console;
//global_asm!(include_str!("entry.S"));
static HELLO: &[u8] = b"\nHello World!!!!!";

#[no_mangle]
pub extern "C" fn i386_init() {
    let uart: Uart = Uart::new().ok().expect("uart error");
    for &byte in HELLO.iter() {
        uart.write_byte(byte);
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