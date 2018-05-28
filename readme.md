`make` will produce three files in build/aarch64 `kernel` is the linked
executable in elf format, `kernel.bin` is the linked executable in binary
format and `kernel.s` is the generated assembly code of the kernel, which
can be used for debugging.

`kernel.bin` can be either runned with qemu, a bash script with the correct
parameters is provided or on real hardware only the imx8m is supported.
To run it on hardware you first need a functioning bootloader for linux
and then replace the linux image, with `kernel.bin`
