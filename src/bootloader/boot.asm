[ORG 0x7c00]
BITS 16

KERNEL_OFFSET equ 0x9000
;; Just in case dl gets overriden

setup_real_mode:
    mov [BOOT_DRIVE], dl
    cli ; Clear Interrupts
    mov ax, 0x00
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov ebp, 0x8000
    mov sp, bp
    sti ; Enables Interrupts

start_real_mode:
    mov si, REAL_MODE_START_MSG
    call rm_print_string

    ;; Load kernel
    mov al, 125                 ; Loads 64KB to not break the DMA boundaries
    mov cl, 2                   ; Start reading from the second sector
    mov bx, KERNEL_OFFSET
    mov dl, [BOOT_DRIVE]

    call load_kernel

    ;; mov si, KERNEL_OFFSET
    ;; call rm_print_string

    mov si, PROTECTED_MODE_START_MSG
    call rm_print_string

    call switch_pmode

    ;; Shouldn't reach this jmp
    jmp $

imports_real_mode:
    %include "src/bootloader/utils/debug_print.asm"
    %include "src/bootloader/utils/disk.asm"
    %include "src/bootloader/switch_pmode.asm"

[BITS 32]

after_pmode_switch:
    jmp CODE_SEG:KERNEL_OFFSET
    jmp end_prog

end_prog:
   jmp $
   nop
   nop

[BITS 16]

;; Declare consts
BOOT_DRIVE db 0
REAL_MODE_START_MSG:  db 10, "> Starting real mode (iow things are getting real). ", 0
PROTECTED_MODE_START_MSG: db "> Starting protected mode. ", 0


;; Set magic number
times 510-($-$$) db 0

dw 0xaa55

