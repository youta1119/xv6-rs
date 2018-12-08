V := @
OBJ_DIR := ../build

OBJCOPY := llvm-objcopy-6.0
OBJDUMP := llvm-objdump-6.0

BUILD_FRAVER := release
RUSTC_TERGET := xv6-rs-bootloader
CARGO_FLAG := 
ifeq ($(BUILD_FRAVER),release)
	CARGO_FLAG += --release
endif	
.PHONY: all

all: $(OBJ_DIR)/bootloader/bootloader;

$(OBJ_DIR)/bootloader/bootloader: target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader
	$(V)mkdir -p $(OBJ_DIR)/bootloader
	$(V)$(OBJDUMP) -S $< > $@.asm
	$(V)$(OBJCOPY) -O binary $< $@

target/$(RUSTC_TERGET)/$(BUILD_FRAVER)/bootloader: src/*.rs src/*.S $(RUSTC_TERGET).json
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(RUSTC_TERGET) $(CARGO_FLAG)

clean:
	$(V)rm -rf $(OBJ_DIR)/bootloader
	$(V)xargo clean