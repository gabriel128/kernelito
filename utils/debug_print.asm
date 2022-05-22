debug_hex:
    push bx

    mov bx, 0x05            ; bx = 5
    xor cl, cl              ; cx = 0

    .loop:
        cmp bx, 1               ;
        je .end_loop            ; if cx == 1 then goto .end_loop

        mov dx, si              ; dx = *esp
        shr dx, cl              ; dx >> cl e.g. 0x0030 -> 0x0003
        and dx, 0x000F          ; dx & 0x000F

        cmp dx, 0xA
        jl .handle_number
        jmp .handle_letter
        .handle_number:
            add dx, 0x30            ; dx = dx + 30
            jmp .after_add
        .handle_letter:
            add dx, 55
        .after_add:

        mov byte [HEX_TEMP + bx], dl ; *(HEX_TEMP + bx) = dl

        ;;  goto Loop
        dec bl                  ; bx--
        add cl, 0x4             ; cl += 4
        jmp .loop               ; goto loop

    .end_loop:
        mov si, HEX_TEMP
        call print_string
        pop bx
        ret

print_string:
    push ax
    ;; Bios scrolling teletype
    mov ah, 0x0E

    .loop:
        ;; mov al, [si]
        ;; inc si
        lodsb
        cmp al, 0
        je .end_loop
        call .print_char
        jmp .loop

    .print_char:
        int 0x10
        ret

    .end_loop:
        pop ax
        ret

HEX_TEMP: db "0x???? ",0
