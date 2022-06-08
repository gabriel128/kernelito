/* OUTPUT_FORMAT(binary) */
OUTPUT_FORMAT(elf32-i386)
__max_stack_size = 0x200;

ENTRY(_start)
SECTIONS
{
    /* .boot 0x7c00 : { */
    /*     *(.boot.*); */
    /* } */

    . = 1M;
    .text : ALIGN(4K)
    {
        *(.text)
    }

    .rodata : ALIGN(4K)
    {
        *(.rodata)
    }

    .data : ALIGN(4K)
    {
        *(.data)
    }

    .bss : ALIGN(4K)
    {
        *(COMMON)
        *(.bss)
    }

    .asm : ALIGN(4K)
    {
        *(.asm)
    }

    /* /DISCARD/ : { */
    /*    *(.comment) */
    /*    *(.eh_frame) */
    /*    *(.eh_frame_hdr) */
    /*    *(.rel.eh_frame) */
    /* } */
}
