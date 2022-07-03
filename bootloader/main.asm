[ORG 0x7c00]
BITS 16

;; KERNEL_OFFSET equ 0x0100000
KERNEL_OFFSET equ 0x7e00

setup_real_mode:
    ;; Just in case dl gets overriden
    mov [BOOT_DRIVE], dl
    cli ; Clear Interrupts
    mov ax, 0x00
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov ebp, 0x7c00
    mov sp, bp
    sti ; Enables Interrupts

start_real_mode:
    mov si, REAL_MODE_START_MSG
    call rm_print_string

    ;; Hide cursor
    mov ah, 0x1
    mov ch, 0x3F
    int 0x10

start_loading_kernel:
    mov si, LOADING_MESSAGE
    call rm_print_string

    ;; Loads first sector
    mov al, 125                 ; Loads 64KB to not break the DMA boundaries
    mov cl, 2                   ; Start reading from the second sector
    mov dh, 0
    mov bx, 0x7e0
    mov es, bx
    mov bx, 0
    mov dl, [BOOT_DRIVE]
    call load_kernel

load_more_sectors:
    cmp dh, 7
    jge stop_loading_sectors
    mov al, 125                 ; Loads 64KB to not break the DMA boundaries
    mov cl, 1                   ; Start reading from the second sector
    inc dh
    mov bx, es
    add bx, 4000
    mov es, bx
    mov bx, 0
    mov dl, [BOOT_DRIVE]
    call load_kernel
    jmp load_more_sectors
stop_loading_sectors:
    mov si, FINISHED_LOADING_MESSAGE
    call rm_print_string


    ;; mov si, KERNEL_OFFSET
    ;; call rm_print_string

    mov si, PROTECTED_MODE_START_MSG
    call rm_print_string

    call switch_pmode

    ;; Shouldn't reach this jmp
    jmp $

imports_real_mode:
    %include "bootloader/utils/debug_print.asm"
    %include "bootloader/utils/disk.asm"
    %include "bootloader/switch_pmode.asm"

[BITS 32]

after_pmode_switch:
    ;; Should load a 130K kernel
    ;; mov eax, 1
    ;; mov ecx, 255
    ;; mov edi, KERNEL_OFFSET
    ;; call ata_lba_read

    ;; Debug - with smiley
    mov bx, 0x0f01
    mov eax, 0x0b8000
    mov word [ds:eax], bx

    jmp CODE_SEG:KERNEL_OFFSET
    ;; shouldn't reach here
    jmp $

imports_pmode:
    ;; %include "bootloader/utils/disk32.asm"

[BITS 16]

;; Declare consts
BOOT_DRIVE db 0
REAL_MODE_START_MSG:  db 0xa, "> Starting real mode (iow things are getting real). ", 0xa, 0xd, 0
PROTECTED_MODE_START_MSG: db "> Starting protected mode. ", 0
MSG:    db "H", 4

DAPACK:
        db  0x10
        db  0
blkcnt: dw  1       ; number of sector/blocks, int 13 resets this to # of blocks actually read/written
db_off: dw  0       ; memory buffer destination address (0:7c00)
db_seg: dw  0       ; in memory page zero
d_lba:  dd  0       ; put the lba to read in this spot
        dd  0       ; more storage bytes only for big lba's ( > 4 bytes )

;; Set magic number
times 510-($-$$) db 0

dw 0xaa55

