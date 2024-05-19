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

<img width="717" alt="image" src="https://github.com/gabriel128/kernelito/assets/2847315/0f101923-edcc-4e67-9aec-b3237f2a8bfc">

# Debug with gdb
```
make debug
```

# Virtual Memory mapping

WIP
