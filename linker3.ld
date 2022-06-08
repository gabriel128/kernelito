ENTRY(_start)
OUTPUT_FORMAT(binary)
/* OUTPUT_FORMAT(elf32-i386) */
phys = 0x0100000;

SECTIONS {
    .text phys : AT(phys) {
        code = .;
        *(.text)
        /* *(.rodata) */
        . = ALIGN(4096);
    }

    end = .;
}
