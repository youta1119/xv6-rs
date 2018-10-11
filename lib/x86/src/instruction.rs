
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("inb %dx, %al" : "={ax}" (data) : "{dx}"(port) :: "volatile");
    return data;
}

#[inline(always)]
pub unsafe fn insl(port: u16, addr: u32, cnt: u32) {
    asm!("cld; rep insl %dx, (%edi)"
        :
        : "{ecx}"(cnt), "{dx}"(port), "{edi}"(addr)
        : "ecx", "edi", "memory", "cc"
        : "volatile");
}

#[inline(always)]
pub unsafe fn outb(port: u16, data: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(data) :: "volatile");
}

