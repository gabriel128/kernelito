ORG 0x7c00
BITS 16

setup_real_mode:
    cli                         ; Clear Interrupts
    mov ax, 0x00
    mov ds, ax
    mov es, ax
    mov ss, ax
    ;; Setup stack
    mov ebp, 0x8000
    mov esp, ebp
    sti                         ; Enble interrupts back

start_real_mode:
    mov si, REAL_MODE_START_MSG
    call rm_print_string

    mov si, PROTECTED_MODE_START_MSG
    call rm_print_string

    call switch_pmode

    ;; Shouldn't reach this jmp
    jmp $

imports_real_mode:
    %include "src/bootloader/utils/debug_print.asm"
    %include "src/bootloader/switch_pmode.asm"

[BITS 32]

after_pmode_switch:
    jmp end_prog

end_prog:
   jmp $
   nop
   nop

[BITS 16]

;; Declare consts
REAL_MODE_START_MSG:  db 10, "Starting real mode (iow things are getting real). ", 0
PROTECTED_MODE_START_MSG: db "Starting protected mode. ", 0


;; Set magic number
times 510-($-$$) db 0

dw 0xaa55

