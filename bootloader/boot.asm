[ORG 0x7c00]
BITS 16

KERNEL_OFFSET equ 0x0100000
;; KERNEL_OFFSET equ 0x9000
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

    ;; Load kernel in real mode
    ;; mov al, 125                 ; Loads 64KB to not break the DMA boundaries
    ;; mov cl, 2                   ; Start reading from the second sector
    ;; mov bx, KERNEL_OFFSET
    ;; mov dl, [BOOT_DRIVE]
    ;; call load_kernel

    ;; mov si, KERNEL_OFFSET
    ;; call rm_print_string

    mov si, PROTECTED_MODE_START_MSG
    call rm_print_string

    call switch_pmode

    ;; Shouldn't reach this jmp
    jmp $

imports_real_mode:
    %include "bootloader/utils/debug_print.asm"
    ;; %include "src/bootloader/utils/disk.asm"
    %include "bootloader/switch_pmode.asm"

[BITS 32]

after_pmode_switch:
    mov eax, 1
    mov ecx, 255
    mov edi, KERNEL_OFFSET
    call ata_lba_read

    ;; add eax, 200
    ;; add ebx, 200
    ;; mov ecx, 200

    ;; mov edi, ebx
    ;; call ata_lba_read

    ;;  Debug

    mov ebx, 0xB8100
    mov eax, [MSG]
    push eax
    pop ecx
    mov [ebx], ecx

    jmp CODE_SEG:KERNEL_OFFSET
    ;; shouldn't reach here
    jmp $

imports_pmode:
    %include "bootloader/utils/disk32.asm"

[BITS 16]

;; Declare consts
BOOT_DRIVE db 0
REAL_MODE_START_MSG:  db 10, "> Starting real mode (iow things are getting real). ", 0
PROTECTED_MODE_START_MSG: db "> Starting protected mode. ", 0
MSG:    db "4", 4

;; Set magic number
times 510-($-$$) db 0

dw 0xaa55

