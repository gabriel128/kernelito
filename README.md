# Kernelito

WIP kernel (most likely will end up as a micro kernel)

# Current State
- ~2 stage x86 handmade Bootloader~ Fuck that, using grub's multiboot now
- 32 bits in protected mode
- Vga driver 
- Cool presentation screen
- Some Interrupt and exceptions handling
- PIC set => Timer and Keybard IRQ set
- Kernel locks (Basic spinlock Mutex and RwLock)
- [WIP] Keybard scancodes mapping
- [WIP] FrameAllocator 
- [WIP] Paging (Basics only, Identity mapped for now, Maybe recursive page directory?) 

![screen](https://user-images.githubusercontent.com/2847315/181233304-8e7e9cd2-cda7-44e2-9a9a-d9fbdd001b53.png)

# Debug with gdb
```
make debug
```

# Virtual Memory mapping

WIP
