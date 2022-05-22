build:
	nasm boot.asm -f bin -o bin/boot.bin

run: build
	qemu-system-x86_64 bin/boot.bin
