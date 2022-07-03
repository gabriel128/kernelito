;; AH = 02h
;; AL = number of sectors to read (must be nonzero)
;; CH = low eight bits of cylinder number
;; CL = sector number 1-63 (bits 0-5)
;; high two bits of cylinder (bits 6-7, hard disk only)
;; DH = head number
;; DL = drive number (bit 7 set for hard disk)
;; ES:BX -> data buffer
;; CF set on error
;; if AH = 11h (corrected ECC error), AL = burst length
;; CF clear if successful
;; AH = status (see #00234)
;; AL = number of sectors transferred (only valid if CF set for some
;; BIOSes)
;;
;;; Takes
;;; CL as the sector where to start reading
;;; AL as the quantity of sectors to read
;;; BX where to put the data in memory
load_kernel:
    mov ah, 0x2
    mov ch, 0
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
