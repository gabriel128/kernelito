# Kernelito

WIP kernel (most likely will end up as a micro kernel)

# Current State
- 2 stage x86 handmade Bootloader 
- 32 bits in protected mode
- Vga driver 
- Cool presentation screen
- Basic IDT (mostly panics for now)
- PIC set => Timer and Keybard IRQ set
- Kernel locks (Basic spinlock Mutex and RwLock)
[WIP] Keybard scancodes mapping
[WIP] FrameAllocator 
[WIP] Paging (Basics only, Identity mapped for now, Maybe recursive page directory?) 


![screen](https://user-images.githubusercontent.com/2847315/181233304-8e7e9cd2-cda7-44e2-9a9a-d9fbdd001b53.png)

# Debug with gdb
```
make debug
```

# Physical Memory mapping

```
----  0xFFFFFFFF (4GB)

Dynamically managed by frame allocator

---- 0x400000 (4MB)

Frame allocator

---- 0x200000 (2MB)

Protected mode Kernel stack

---- 0x110000

Stack Guard (WIP)

---- 0x100000 (1MB)

Bios Stuff

---- 0xB8FA0

VGA IO mapped mem

---- 0xB8000

Bios Stuff

---- 0x7FFFF

Kernel Code (Max ~480KB)

---- 0x7E00

Bootloader 

---- 0x7C00

Real mode Kernel stack 

---- 0x4FF

Bios Stuff

---- 0
```

# Virtual Memory mapping

WIP
