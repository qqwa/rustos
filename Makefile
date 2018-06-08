ARCH=aarch64
TARGET=$(ARCH)-unknown-none
LD=ld.lld

CARGO_OPTS=--target=$(TARGET) --release
RUST_TARGET_PATH=$(PWD)

all: build/$(ARCH)/kernel.bin

build/$(ARCH)/kernel: src/*.rs src/*.s Cargo.toml
	mkdir -p $(@D)
	xargo rustc $(CARGO_OPTS) --cfg feature="imx8" -- -C soft-float --emit=link=$@ --emit=asm=$@.s

build/$(ARCH)/kernel.bin: build/$(ARCH)/kernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

clean:
	cargo clean
	rm -rf build

.PHONY: clean
