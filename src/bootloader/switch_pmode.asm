;;; Switchs to protected mode.
;;;
;;; Expects a label `after_pmode_switch` as a continuation
%include "src/bootloader/gdt.asm"

switch_pmode:
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 0x01
    mov cr0, eax
    jmp CODE_SEG:init_prot_mode

[BITS 32]

init_prot_mode:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax
    ;; Setup stack
    mov ebp, 0x00200000
    mov esp, ebp

    ;; Enable A20 mode
    in al, 0x92
    or al, 2
    out 0x92, al

    jmp after_pmode_switch
