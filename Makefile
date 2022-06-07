all: build run

build: clean
	nasm src/bootloader/boot.asm -f bin -o bin/boot.bin
	dd if=./bin/boot.bin >> ./bin/kernel.img
	truncate --size 10M ./bin/kernel.img

run:
	qemu-system-x86_64 -hda bin/kernel.img

c-kernel: clean
	gcc -T linker.ld -ffreestanding -c ./src/c_kernel/kernel.c -o ./bin/c_kernel/kernel.o
	ld -o ./bin/c_kernel/kernel.bin -Ttext 0x0100000 ./bin/c_kernel/kernel.o --oformat binary
	nasm src/bootloader/boot.asm -f bin -o bin/boot.bin

	dd if=./bin/boot.bin >> ./bin/kernel.img
	dd if=./bin/c_kernel/kernel.bin >> ./bin/kernel.img
	truncate --size 1M ./bin/kernel.img

clean:
	rm -rf bin/boot.bin
	rm -rf bin/kernel.img
	rm -rf bin/c_kernel/*.o
	rm -rf bin/c_kernel/*.bin

debug: build
	gdb -q
