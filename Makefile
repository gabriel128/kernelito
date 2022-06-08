all: build run

build: clean
	nasm -g src/bootloader/boot.asm -f bin -o bin/boot.bin

	# cd ./src/kernel && cargo clean
	# cd ./src/kernel && cargo build --release
	# cp ./src/kernel/target/i686/release/kernel ./bin/kernel.bin
	# objcopy -O binary ./bin/kernel.o ./bin/kernel.bin
	# ld -m elf_i386 -o ./bin/kernel.bin bin/kernel -oformat binary

	cd ./src/kernel && cargo build --release
	cp ./src/kernel/target/i686/release/kernel ./bin/kernel
	ld --gc-section -n -m elf_i386 -o ./bin/kernel.bin -Tlinker.ld bin/kernel

	# cd ./src/kernel && cargo build
	# cp ./src/kernel/target/i686/debug/kernel ./bin/kernel
	# ld -m elf_i386 -o ./bin/kernel.bin -Tlinker.ld bin/kernel

	# cd ./src/kernel && cargo build --release
	# cp ./src/kernel/target/x86/release/kernel ./bin/kernel
	# ld -n -o ./bin/kernel.bin -Tlinker.ld bin/kernel

	dd if=./bin/boot.bin >> ./bin/kernel.img
	dd if=./bin/kernel.bin >> ./bin/kernel.img
	truncate --size 10M ./bin/kernel.img

run:
	# qemu-system-x86_64 -hda bin/kernel.img
	qemu-system-x86_64 -drive format=raw,file=bin/kernel.img

c-kernel: clean
	gcc -T linker.ld -ffreestanding -c ./src/c_kernel/kernel.c -o ./bin/c_kernel/kernel.o
	ld -m elf_i386 -o ./bin/c_kernel/kernel.bin -Ttext 0x0100000 ./bin/c_kernel/kernel.o --oformat binary
	nasm src/bootloader/boot.asm -f bin -o bin/boot.bin

	dd if=./bin/boot.bin >> ./bin/kernel.img
	dd if=./bin/c_kernel/kernel.bin >> ./bin/kernel.img
	truncate --size 1M ./bin/kernel.img

clean:
	rm -rf bin/boot.bin
	rm -rf bin/kernel
	rm -rf bin/kernel.o
	rm -rf bin/kernel.bin
	rm -rf bin/kernel.img
	rm -rf bin/c_kernel/*.o
	rm -rf bin/c_kernel/*.bin

debug: build
	gdb -q
