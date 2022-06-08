;; CODE_SEG_OFFSET equ gdt_code - gdt_start
;; DATA_SEG_OFFSET equ gdt_data - gdt_start

gdt_start:
;;; Offset + 0
gdt_null:
    dd 0x0
    dd 0x0

;;; Code Segment (CS register)
;;; Offset + 0x08
gdt_code:
    dw 0xffff                   ; Limit (0-15 bits)
    dw 0x0                      ; Base (0-15 bits)
    db 0x0                      ; Base (16-23 bits)
    db 0x9A                     ; AccessBytes 10011010
    db 0xCF                     ; Limit (16-19), Flags (0-3)
    db 0x0                      ; Base (24-31)

;;; Data segment. (Registers DS, SS, ES, FS, GS)
;;; Ofsset + 0x10
gdt_data:
    dw 0xffff                   ; Limit (0-15 bits)
    dw 0x0                      ; Base (0-15 bits)
    db 0x0                      ; Base (16-23 bits)
    db 0x92                     ; AccessBytes 10010010
    db 0xCF                     ; Limit (16-19) Flags (0-3)
    db 0x0                      ; Base (24-31)

gdt_end:
gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
