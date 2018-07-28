#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(global_asm)]
//#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn bootmain()  {
    return
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
