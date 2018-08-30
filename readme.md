# poc-kernel

Small kernel written as part of my bachelor thesis.

## Compiling

To compile the project just run `make`. The subsections Ubuntu and Arch Linux
describe how to install all dependencies (qemu is only needed if you want to run
the kernel with qemu).

Compiling may fail on the first try requiring to execute `rustup component add
rust-src`

The kernel image will be located at build/aarch64/kernel.bin

### Ubuntu

Install rustup by following the instructions on https://rustup.rs/

```
apt install build-essential
apt install lld-6.0
apt install qemu-system-aarch64
cargo install cargo-xbuild
```

Because the lld executable has the name lld-6.0 we either need to change the
linker name in the make file or crate a symbolic link
```
ln -s /usr/bin/lld-6.0 /usr/bin/ld.lld
```

### Arch Linux

```
pacman -S rustup
pacman -S lld
pacman -S qemu-arch-extra
cargo install cargo-xbuild
```

## Run

Depending on the target hardware you may need to change the configuration in the
file `config.toml`

### qemu

Execute the `qemu.sh` script.

### hardware

To run it on real hardware just replace the Linux image with kernel.bin on a
working boot loader + Linux setup. It was only tested with the phyCORE-i.MX 8M.
