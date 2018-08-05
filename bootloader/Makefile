OBJ_DIR = obj
LLVM_PREFIX:=$(shell brew --prefix llvm)
OBJCOPY:=$(LLVM_PREFIX)/bin/llvm-objcopy
OBJDUMP:=$(LLVM_PREFIX)/bin/llvm-objdump

BUILD_FRAVER := release
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG := 
ifeq ($(BUILD_FRAVER),release)
	CARGO_FLAG += --release
endif	
.PHONY: bootloader-xv6
$(OBJ_DIR)/bootloader.bin: target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader
	mkdir -p $(OBJ_DIR)
	$(OBJDUMP) -S $< > $(OBJ_DIR)/bootloader.asm
	$(OBJCOPY) -O binary $< $@
	hexdump $@

target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader: src/*.rs src/*.S $(RUSTC_TERGET).json
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)


bootloader-xv6: $(OBJ_DIR)/bootloader.bin
	mkdir -p ../lab/obj/boot
	cp $< ../lab/obj/boot/boot
	rm -rf ../lab/obj/kern/kernel.img
	cd ../lab && docker run -it --rm -v `pwd`:/root/work youta1119/xv6-toolchain make BOOT_OBJS= obj/kern/kernel.img
	qemu-system-i386 -d int -no-reboot -hda ../lab/obj/kern/kernel.img

clean:
	rm -rf obj
	cargo clean
	xargo clean