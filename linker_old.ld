OUTPUT_FORMAT(binary)
/* OUTPUT_FORMAT(elf32-i386) */

ENTRY(kmain)
SECTIONS
{
    .boot 0x7c00 : {
        *(.boot.*);
    }

    . = 0x7e00;
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

    /* Without this kernel end is not incremented to avoid colliding with bss */
    .phony : {

    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.rel.eh_frame)
    }
}
