
Deassemble binary file
```
aarch64-linux-gnu-objdump -b binary -m aarch64 -D build/aarch64/kernel.bin | less
```

Run kernel in qemu
```
qemu-system-aarch64 -M virt -cpu cortex-a53 -m 128M  -smp 1 -nographic -kernel build/aarch64/kernel.bin
```
