#![no_std]
#![feature(lang_items)]
#![feature(asm)]
extern crate x86;
#[macro_use]
extern crate lazy_static;
extern crate spin;
mod io;
use core::panic::PanicInfo;
use io::uart::Uart;

#[macro_use]
mod console;
//global_asm!(include_str!("entry.S"));
static HELLO: &[u8] = b"\nHello World!!!!!";

#[no_mangle]
pub extern "C" fn i386_init() {
    println!("Hello world!");
    println!("Hello {}!", "world");
    println!("{} | {} | {1} | {three} | {four}",
         "One",
         "Two",
         three="three",
         four=4);
    loop {}
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}