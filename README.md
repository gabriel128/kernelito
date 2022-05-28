# Kernelito

WIP kernel (most likely a micro kernel)

# Debug with gdb

```
(gdb)
target remote | qemu-system-x86_64 -hda ./bin/boot.bin -S -gdb stdio
```

