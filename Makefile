FEATURES ?= default

all: build run

build: clean
	nasm -g bootloader/main.asm -f bin -o bin/boot.bin

	# Build Rust
	cargo build --release --features $(FEATURES)
	# cp target/i686/release/kernelin bin/kernel.bin

	cp target/i686/release/libkernelito.a build/libkernelito.a
# i686-elf-gcc -g -n -ffreestanding -nostdlib -nostartfiles -o ./bin/kernel.bin -Tlinker.ld build/libkernelito.a
	i686-elf-ld -g -m elf_i386 -o ./bin/kernel.bin -Tlinker.ld build/libkernelito.a

	# i686-elf-ld -o ./bin/kernel.bin -Tlinker.ld ./target/i686/release/kernelito.a
	rm -rf ./bin/kernel.img
	dd if=./bin/boot.bin >> ./bin/kernel.img
	dd if=./bin/kernel.bin >> ./bin/kernel.img
	dd if=/dev/zero bs=512 count=4096 >> ./bin/kernel.img

	du -h ./build/libkernelito.a
	du -h ./bin/kernel.img
	du -h ./bin/kernel.bin

run-checks:
	FEATURES="checks-mode" make


build-debug: clean
	nasm -g bootloader/main.asm -f bin -o bin/boot.bin

	cargo build
	cp target/i686/debug/libkernelito.a build/libkernelito.a
# ld -g --gc-sections -n -m elf_i386 -o ./bin/kernel.bin -Tlinker.ld build/libkernelito.a
# i686-elf-ld -g -n --gc-sections -m elf_i386 -o ./bin/kernel.bin -Tlinker.ld build/libkernelito.a
# i686-elf-ld -m elf_i386 -o ./bin/kernel.bin -Tlinker.ld build/libkernelito.a

	dd if=./bin/boot.bin >> ./bin/kernel.img
	dd if=./bin/kernel.bin >> ./bin/kernel.img
	dd if=/dev/zero bs=512 count=4000 >> ./bin/kernel.img
	du -h ./bin/kernel.bin
	du -h ./bin/kernel.img

debug-run: build-debug
	make run

run:
	# qemu-system-x86_64 -s -S -no-reboot -drive format=raw,file=bin/kernel.img
	# qemu-system-i386 -accel tcg -d int,cpu_reset -D ./log.txt -cpu core2duo -m 128 -no-reboot -drive format=raw,file=bin/kernel.img -monitor pty -smp 1 -vga etd
	qemu-system-i386 -no-reboot -drive format=raw,file=bin/kernel.img

# DEPRECATED
# c-kernel: clean
# 	gcc -T linker.ld -ffreestanding -c ./src/c_kernel/kernel.c -o ./bin/c_kernel/kernel.o
# 	ld -m elf_i386 -o ./bin/c_kernel/kernel.bin -Ttext 0x0100000 ./bin/c_kernel/kernel.o --oformat binary
# 	nasm src/bootloader/boot.asm -f bin -o bin/boot.bin

# 	dd if=./bin/boot.bin >> ./bin/kernel.img
# 	dd if=./bin/c_kernel/kernel.bin >> ./bin/kernel.img
# 	truncate --size 1M ./bin/kernel.img

vb: build
	VBoxManage convertfromraw bin/kernel.img bin/kernelito.vdi --format VDI

clean:
	rm -rf build/*
	rm -rf bin/*

gdb: build
	gdb -ex 'target remote | qemu-system-i386 -hda ./bin/kernel.img -S -gdb stdio' \
        -ex 'set architecture i386' \
		-ex 'continue'
		# -ex 'layout src'
		# -ex 'hbreak *0x100000' \

test:
	cross +stable test --target=i686-unknown-linux-gnu -- --nocapture

test-mutex:
	cargo +stable watch -x "test sync --target=i686-unknown-linux-gnu -- --color=always --nocapture --ignored"
