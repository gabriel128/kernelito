build:
	nasm boot.asm -f bin -o boot.bin

run: build
	qemu-system-x86_64 boot.bin
