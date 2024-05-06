FEATURES ?= default

all: build run

build: clean
	nasm -f elf32 -o build/boot.o mb-bootloader/boot.asm
	cargo build --release --features $(FEATURES)
# cp target/i686/release/kernelito bin/kernel.bin
	i686-elf-gcc -T linker.ld -o bin/kernel.bin -ffreestanding -O2 -nostdlib build/boot.o target/i686/release/libkernelito.a -lgcc
	du -h ./bin/kernel.bin

run-checks:
	FEATURES="checks-mode" make

build-debug: clean
	nasm -f elf32 -o build/boot.o mb-bootloader/boot.asm
	cargo build --features $(FEATURES)
# cp target/i686/debug/kernelito bin/kernel.bin
	i686-elf-gcc -T linker.ld -o bin/kernel.bin -ffreestanding -O2 -nostdlib build/boot.o target/i686/debug/libkernelito.a -lgcc
	du -h ./bin/kernel.bin

debug-run: build-debug
	make run

run:
# qemu-system-x86_64 -s -S -no-reboot -drive format=raw,file=bin/kernel.img
# qemu-system-i386 -accel tcg -d int,cpu_reset -D ./log.txt -cpu core2duo -m 128 -no-reboot -kernel bin/kernel.bin -monitor pty -smp 1
	qemu-system-i386 -no-reboot -kernel bin/kernel.bin

clean:
	rm -rf build/*
	rm -rf bin/*

gdb: build
	gdb -ex 'target remote | qemu-system-i386 -hda ./bin/kernel.img -S -gdb stdio' \
        -ex 'set architecture i386' \
		-ex 'continue'
		# -ex 'layout src'
		# -ex 'hbreak *0x100000' \
		#
test:
	cross +stable test --target=i686-unknown-linux-gnu -- --nocapture

test-mutex:
	cargo +stable watch -x "test sync --target=i686-unknown-linux-gnu -- --color=always --nocapture --ignored"
