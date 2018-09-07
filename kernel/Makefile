V=@
OBJDIR = obj

OBJCOPY:=i386-jos-elf-objcopy
OBJDUMP:=i386-jos-elf-objdump
CC := i386-jos-elf-gcc -pipe
LD := i386-jos-elf-ld
NM := i386-jos-elf-nm
GCC_LIB := $(shell $(CC) $(CFLAGS) -print-libgcc-file-name)

# compiler flags
# -fno-builtin is required to avoid refs to undefined functions in the kernel.
# Only optimize to -O1 to discourage inlining, which complicates backtraces.
CFLAGS :=  -O1 -fno-builtin -I. -MD
CFLAGS += -fno-omit-frame-pointer
CFLAGS += -std=gnu99
CFLAGS += -static
CFLAGS += -Wall -Wno-format -Wno-unused -Werror -gstabs -m32
# -fno-tree-ch prevented gcc from sometimes reordering read_ebp() before
# mon_backtrace()'s function prologue on gcc version: (Debian 4.7.2-5) 4.7.2
CFLAGS += -fno-tree-ch
# Add -fno-stack-protector if the option exists.
CFLAGS += $(shell $(CC) -fno-stack-protector -E -x c /dev/null >/dev/null 2>&1 && echo -fno-stack-protector)
#kernel option
CFLAGS += -DJOS_KERNEL -gstabs
# linker flags
LDFLAGS := -m elf_i386
LDFLAGS += -T kernel.ld -nostdlib
# rustc option
BUILD_FRAVER := debug
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG := 
ifeq ($(BUILD_FRAVER),release)
	CARGO_FLAG += --release
endif
RUST_LIB := target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/libkernel.a
KERN_SRCFILES := lib/entry.S lib/entrypgdir.c
# Only build files if they exist.
KERN_SRCFILES := $(wildcard $(KERN_SRCFILES))
# Binary program images to embed within the kernel.
KERN_OBJFILES := $(patsubst %.c, $(OBJDIR)/%.o, $(KERN_SRCFILES))
KERN_OBJFILES := $(patsubst %.S, $(OBJDIR)/%.o, $(KERN_OBJFILES))
KERN_OBJFILES := $(patsubst $(OBJDIR)/lib/%, $(OBJDIR)/kern/%, $(KERN_OBJFILES))
KERN_OBJFILES +=  $(RUST_LIB)
# How to build kernel object files
$(OBJDIR)/kern/%.o: lib/%.c
	@echo + cc $<
	@mkdir -p $(@D)
	$(V)$(CC) -nostdinc $(CFLAGS) -c -o $@ $<

$(OBJDIR)/kern/%.o: lib/%.S 
	@echo + as $<
	@mkdir -p $(@D)
	$(V)$(CC) -nostdinc $(CFLAGS) -c -o $@ $<

$(RUST_LIB): src/*.rs i386-unknown-none.json
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)

# How to build the kernel itself
$(OBJDIR)/kern/kernel:$(KERN_OBJFILES)
	@echo + ld $@
	$(V)$(LD) -o $@ $(LDFLAGS) $(KERN_OBJFILES) $(GCC_LIB) -b binary
	$(V)$(OBJDUMP) -S $@ > $@.asm
	$(V)$(NM) -n $@ > $@.sym

# How to build the kernel disk image
$(OBJDIR)/kern/kernel.img: $(OBJDIR)/kern/kernel
	@echo + mk $@
	$(V)dd if=/dev/zero of=$(OBJDIR)/kern/kernel.img~ count=10000 2>/dev/null
	$(V)dd if=$(OBJDIR)/boot/boot of=$(OBJDIR)/kern/kernel.img~ conv=notrunc 2>/dev/null
	$(V)dd if=$(OBJDIR)/kern/kernel of=$(OBJDIR)/kern/kernel.img~ seek=1 conv=notrunc 2>/dev/null
	$(V)mv $(OBJDIR)/kern/kernel.img~ $(OBJDIR)/kern/kernel.img

qemu: $(OBJDIR)/kern/kernel.img
	qemu-system-i386 -d int -no-reboot -hda $<
clean:
	rm -rf obj/kern
	cargo clean
	xargo clean
