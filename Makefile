ARCH=aarch64
TARGET=$(ARCH)-unknown-none
LD=ld.lld

CARGO_OPTS=--target=$(TARGET) --release
RUST_TARGET_PATH=$(PWD)

# RUSTFLAGS+=--cfg device="imx8"
export RUSTFLAGS

all: build/$(ARCH)/kernel.bin build/$(ARCH)/qernel.bin

build/$(ARCH)/kernel: RUSTFLAGS=--cfg device="imx8" --emit=asm=$@.s -C soft-float
build/$(ARCH)/kernel: src/*.rs src/*.s Cargo.toml
	mkdir -p $(@D)
	echo $$RUSTFLAGS
	cargo xbuild $(CARGO_OPTS)
	cp target/aarch64-unknown-none/release/rustos build/$(ARCH)/kernel
	# xargo rustc $(CARGO_OPTS) -- -C soft-float --emit=link=$@ --emit=asm=$@.s

build/$(ARCH)/qernel: RUSTFLAGS+=--cfg device="qemu-virt" --emit=asm=$@.s
build/$(ARCH)/qernel: src/*.rs src/*.s Cargo.toml
	mkdir -p $(@D)
	echo "$$RUSTFLAGS"
	cargo xbuild $(CARGO_OPTS)
	cp target/aarch64-unknown-none/release/rustos build/$(ARCH)/qernel
	# cp target/aarch64-unknown-none/release/rustos build/$(ARCH)/qernel

build/$(ARCH)/kernel.bin: build/$(ARCH)/kernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

build/$(ARCH)/qernel.bin: build/$(ARCH)/qernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

clean:
	cargo clean
	rm -rf build

.PHONY: clean
