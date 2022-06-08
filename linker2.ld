ENTRY(_start)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x0100000, l = 12K
}

SECTIONS {
    . = 0x0100000;

    .text : {
        *(.text)
    } >ram

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
