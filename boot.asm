ORG 0
BITS 16

jmp 0x7c0:setup

setup:
    cli                         ; Clear Interrupts
    mov ax, 0x7c0
    mov ds, ax
    mov es, ax
    mov ax, 0x0
    mov ss, ax
    ;; Setup stack
    mov ebp, 0x8000
    mov esp, ebp
    sti                         ; Enble interrupts back

start:
    mov si, hello
    call print_string

    mov si, 0xFFF2
    call debug_hex
    jmp end_prog

%include "utils/debug_print.asm"

end_prog:
;; Infinite loop
jmp $

;; Declare const
hello:  db "Hello4! ", 0
world:  db "World!", 0
;; magic number
times 510-($-$$) db 0

dw 0xaa55

