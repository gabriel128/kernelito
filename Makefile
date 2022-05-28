all: build run

build:
	nasm src/bootloader/boot.asm -f bin -o bin/boot.bin

run: 
	qemu-system-x86_64 bin/boot.bin

clean:
	rm -rf bin/boot.bin

debug: build
	gdb -q
