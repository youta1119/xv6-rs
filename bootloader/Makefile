TOOLCHAIN := i386-jos-elf-
OBJCOPY	:= $(TOOLCHAIN)objcopy

BUILD_FRAVER := release
RUSTC_TERGET := i386-unknown-none
CARGO_FLAG := 
ifeq ($(BUILD_FRAVER),release)
    CARGO_FLAG += --release
endif
OBJ_DIR := obj

.PHONY: clean

target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader: src/*.rs src/*.S $(RUSTC_TERGET).json
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)

$(OBJ_DIR)/bootloader.bin: target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader
	@mkdir -p $(OBJ_DIR)
	$(OBJCOPY) -O binary -S $< $@
clean:
	rm -rf obj
	rm -rf target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader

	