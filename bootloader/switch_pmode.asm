;;; Switchs to protected mode.
;;;
;;; Expects a label `after_pmode_switch` as a continuation
%include "bootloader/gdt.asm"

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
    mov esp,  0x9FFF0
    mov ebp, esp

    ;; Enable A20 mode
    in al, 0x92
    test al, 2
    jnz after
    or al, 2
    and al, 0xFE
    out 0x92, al
    after:
        jmp after_pmode_switch
