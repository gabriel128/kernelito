all: build run

build: clean
	nasm -g bootloader/boot.asm -f bin -o bin/boot.bin

	cargo build --release
	cp target/i686/release/libkernelito.a build/libkernelito.a
	i686-elf-ld -n -gc-section -o ./bin/kernel.bin -Tlinker.ld build/libkernelito.a
	# i686-elf-gcc -o ./bin/kernel.bin -Tlinker.ld -O0 -nostdlib -ffreestanding build/libkernelito.a

	dd if=./bin/boot.bin >> ./bin/kernel.img
	dd if=./bin/kernel.bin >> ./bin/kernel.img
	truncate --size 10M ./bin/kernel.img

run:
	qemu-system-x86_64 -drive format=raw,file=bin/kernel.img

# DEPRECATED
# c-kernel: clean
# 	gcc -T linker.ld -ffreestanding -c ./src/c_kernel/kernel.c -o ./bin/c_kernel/kernel.o
# 	ld -m elf_i386 -o ./bin/c_kernel/kernel.bin -Ttext 0x0100000 ./bin/c_kernel/kernel.o --oformat binary
# 	nasm src/bootloader/boot.asm -f bin -o bin/boot.bin

# 	dd if=./bin/boot.bin >> ./bin/kernel.img
# 	dd if=./bin/c_kernel/kernel.bin >> ./bin/kernel.img
# 	truncate --size 1M ./bin/kernel.img

clean:
	rm -rf build/*
	rm -rf bin/*

debug: build
	gdb -q
