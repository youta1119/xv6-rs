OUTPUT_FORMAT("elf32-i386", "elf32-i386", "elf32-i386")
OUTPUT_ARCH(i386)
ENTRY(start)
IPLBASE = 0x7c00;

SECTIONS {
    . = IPLBASE;
    .text : AT(IPLBASE) {
      *(.text)
      *(.data)
      *(.bbs)
    }
    .sign : AT(IPLBASE + 510) {
      SHORT(0xaa55)
    }
}