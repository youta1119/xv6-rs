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
    . = IPLBASE + 510;
    .sign : {
      SHORT(0xaa55)
    }
}