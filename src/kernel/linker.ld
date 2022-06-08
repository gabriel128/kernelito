ENTRY(_start)
OUTPUT_FORMAT(binary)
/* OUTPUT_FORMAT(elf32-i386) */
SECTIONS
{
    . = 1M;
    .text : ALIGN(4096)
    {
        *(.text)
    }

    .rodata : ALIGN(4096)
    {
        *(.rodata)
    }

    .data : ALIGN(4096)
    {
        *(.data)
    }

    .bss : ALIGN(4096)
    {
        *(COMMON)
        *(.bss)
    }

    .asm : ALIGN(4096)
    {
        *(.asm)
    }

    /* /DISCARD/ : { */
    /*    *(.comment)     */
    /*    *(.eh_frame) */
    /*    *(.eh_frame) */
    /*    *(.rel.eh_frame) */
    /* } */
}
