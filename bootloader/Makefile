OBJ_DIR = obj
OBJCOPY	:= llvm-objcopy-6.0
OBJDUMP := llvm-objdump-6.0
BUILD_FRAVER := release
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG := 
ifeq ($(BUILD_FRAVER),release)
    CARGO_FLAG += --release
endif

obj/bootloader.bin: target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader
	mkdir -p $(OBJ_DIR)
	$(OBJDUMP) -S $< > $(OBJ_DIR)/bootloader.asm
	$(OBJCOPY) -O binary $< $@

target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader: src/*.rs src/*.S $(RUSTC_TERGET).json
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)


clean:
	rm -rf obj
	cargo clean
	xargo clean