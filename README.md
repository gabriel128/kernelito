# Kernelito

WIP kernel (most likely will endaup as a micro kernel)

# Current State
- 2 stage x86 handmade Bootloader 
- 32 bits in protected mode
- Vga driver 
- Basic IDT (only panics for now)
- HW interrupts
- Cool presentation screen


![fornow](https://user-images.githubusercontent.com/2847315/176840794-6c7bebea-fd72-43f2-9351-afaea22e5efa.png)

# Debug with gdb

```
make debug
```

# Memory mapping

----  0xF4240000 (4GB)

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

