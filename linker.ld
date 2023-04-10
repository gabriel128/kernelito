OUTPUT_FORMAT(binary)
/* OUTPUT_FORMAT(elf32-i386) */

ENTRY(kmain)
SECTIONS
{
    .boot 0x7c00 : {
        *(.boot.*);
    }

    . = 0x7e00;
    .text : ALIGN(512)
    {
        *(.text)
    }

    .rodata : ALIGN(512)
    {
        *(.rodata)
    }

    .data : ALIGN(512)
    {
        *(.data)
    }

    .bss : ALIGN(512)
    {
        *(COMMON)
        *(.bss)
    }

    .asm : ALIGN(512)
    {
        *(.asm)
    }

    /* /DISCARD/ : { */
    /*     *(.comment) */
    /*     *(.eh_frame) */
    /*     *(.rel.eh_frame) */
    /* } */
}
