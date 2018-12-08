
V := @
OBJ_DIR := build
.PHONY: all

all:$(OBJ_DIR)/xv6-rs.img
	
qemu: $(OBJ_DIR)/xv6-rs.img
	qemu-system-i386 -no-reboot -nographic -serial mon:stdio build/xv6-rs.img
	#qemu-system-i386 -d int -no-reboot -hda $<

clean:
	make -C bootloader V=$(V) clean
	make -C kernel V=$(V) clean
	rm -rf $(OBJ_DIR)

# How to build the kernel disk image
$(OBJ_DIR)/xv6-rs.img:
	@echo + mk $@
	$(V)make -C bootloader V=$(V)
	$(V)make -C kernel V=$(V)
	$(V)dd if=/dev/zero of=$(OBJ_DIR)/xv6-rs.img~ count=10000 2>/dev/null
	$(V)dd if=$(OBJ_DIR)/bootloader/bootloader of=$(OBJ_DIR)/xv6-rs.img~ conv=notrunc 2>/dev/null
	$(V)dd if=$(OBJ_DIR)/kernel/kernel of=$(OBJ_DIR)/xv6-rs.img~ seek=1 conv=notrunc 2>/dev/null
	$(V)mv $(OBJ_DIR)/xv6-rs.img~ $(OBJ_DIR)/xv6-rs.img