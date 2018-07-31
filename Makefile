OBJCOPY	:= llvm-objcopy
OBJDUMP := llvm-objdump

OBJ_DIR := obj
obj/bootloader.bin:
	make -C bootloader 

$(OBJ_DIR):
	mkdir -p $@
image: $(OBJ_DIR) $(OBJ_DIR)/bootloader.bin

