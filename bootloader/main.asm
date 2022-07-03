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

    call load_kernel

    ;; mov si, KERNEL_OFFSET
    ;; call rm_print_string

    mov si, PROTECTED_MODE_START_MSG
    call rm_print_string

    call switch_pmode

    ;; Shouldn't reach this jmp
    jmp $

imports_real_mode:
    %include "bootloader/utils/debug_print.asm"
    %include "bootloader/kernel_loading.asm"
    %include "bootloader/switch_pmode.asm"

[BITS 32]

after_pmode_switch:
    ;; Should load a 130K kernel
    ;; mov eax, 1
    ;; mov ecx, 255
    ;; mov edi, KERNEL_OFFSET
    ;; call ata_lba_read

    ;; Show smiley
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

;; Set magic number
times 510-($-$$) db 0

dw 0xaa55

