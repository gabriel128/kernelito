ORG 0x7c00
BITS 16
;; Bios scrolling teletype
mov ah, 0x0E

;; Setup stack
mov ebp, 0x8000
mov esp, ebp

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

