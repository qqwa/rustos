
```bash
export RUST_TARGET_PATH=$PWD
xargo build --target=aarch64-none-efi --release
aarch64-linux-gnu-objcopy -O binary -S target/aarch64-unknown-none/release/phyos phyos.img
```

inspect generated executable:
`aarch64-linux-gnu-objdump -D -b elf64-littleaarch64 -m aarch64`
