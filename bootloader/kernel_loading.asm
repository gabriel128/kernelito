;;; Loads 512kB worth of sectors
load_kernel:
    pusha

    mov ch, 0
    ;; Loads first sector
    mov al, 62                 ; Loads 64KB to not break the DMA boundaries
    mov cl, 2                   ; Start reading from the second sector
    mov dh, 0
    mov bx, KERNEL_OFFSET
    shr bx, 4                   ; 0x7e00 -> 0x7e0 since we are using the segment
    mov es, bx
    mov bx, 0                   ; no offset
    mov dl, [BOOT_DRIVE]
    call do_load_kernel

    mov bx, es
    add bx, 1984
    mov es, bx
load_more_sectors:
    cmp dh, 14
    jg stop_loading_sectors
    mov al, 63                 ; Loads 64KB to not break the DMA boundaries
    mov cl, 1                   ; Start reading from the second sector
    inc dh
    mov bx, 0
    mov dl, [BOOT_DRIVE]
    call do_load_kernel

    mov bx, es
    add bx, 2016
    mov es, bx
    jmp load_more_sectors
stop_loading_sectors:
load_last_bunch:
    mov al, 56                 ; Loads last 56 sectors to fill in 512
    mov cl, 1                  ; Start reading from the first sector
    mov dh, 0                     ; Use next head
    mov ch, 1                     ; Use next cylinder
    mov bx, 0                  ; set offset to 0 since we are using the segment
    mov dl, [BOOT_DRIVE]
    call do_load_kernel

    mov si, FINISHED_LOADING_MESSAGE
    call rm_print_string
    popa
    ret

do_load_kernel:
    mov ah, 0x2
    int 0x13
    jc error

    xor ah, ah      ; reset disk system (int 0x13, ah = 0x00)
    int 0x13
    ret

error:
    mov si, ERROR_MESSAGE
    call rm_print_string
    jmp $

ERROR_MESSAGE:  db "Failed to load sector ", 0
LOADING_MESSAGE:  db "> Loading kernel ", 0xa, 0xd, 0
FINISHED_LOADING_MESSAGE:  db "> Finished Loading kernel ", 0xa, 0xd, 0
