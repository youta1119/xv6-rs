all: bootloader.bin:

qemu:
	target/i386-unknown-none/debug/bootloader

bootloader.bin: src/**/*.rs
	RUST_TARGET_PATH=$(shell pwd) xargo build --target i386-unknown-none
	i386-jos-elf-objcopy -O binary -S target/i386-unknown-none/debug/bootloader bootloader.bin

