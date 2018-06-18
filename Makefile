ARCH=aarch64
TARGET=$(ARCH)-unknown-none
LD=ld.lld

CARGO_OPTS=--target=$(TARGET) --release
RUST_TARGET_PATH=$(PWD)

# RUSTFLAGS+=--cfg device="imx8"
export RUSTFLAGS
export RUST_TARGET_PATH

all: build/$(ARCH)/kernel.bin build/$(ARCH)/qernel.bin

build/$(ARCH)/kernel: RUSTFLAGS=-C soft-float
build/$(ARCH)/kernel: */src/*.rs */src/*.s Cargo.toml
	mkdir -p $(@D)
	echo $$RUSTFLAGS
	cargo xbuild $(CARGO_OPTS) -p rustos -- --emit=link=$@ --emit=asm=$@.s --cfg device=\"imx8\"

build/$(ARCH)/qernel: RUSTFLAGS=-C soft-float
build/$(ARCH)/qernel: */src/*.rs */src/*.s Cargo.toml
	mkdir -p $(@D)
	echo "$$RUSTFLAGS"
	cargo xbuild $(CARGO_OPTS) -p rustos -- --emit=link=$@ --emit=asm=$@.s --cfg device=\"qemu-virt\"

build/$(ARCH)/kernel.bin: build/$(ARCH)/kernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

build/$(ARCH)/qernel.bin: build/$(ARCH)/qernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

clean:
	cargo clean
	rm -rf build

.PHONY: clean
