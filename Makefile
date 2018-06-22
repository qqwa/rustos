ARCH=aarch64
TARGET=$(ARCH)-unknown-none
LD=ld.lld

CARGO_OPTS=--target=$(TARGET) --release --features "verbose-exception-handler"
RUST_TARGET_PATH=$(PWD)
RUSTFLAGS=-C soft-float
RUSTDOC=./rustdoc.sh

export RUSTFLAGS
export RUST_TARGET_PATH
export RUSTDOC

all: build/$(ARCH)/imx8/kernel.bin

build/$(ARCH)/imx8/kernel: */src/*.rs */src/*.s Cargo.toml
	mkdir -p $(@D)
	echo $$RUSTFLAGS
	cargo xrustc $(CARGO_OPTS) --manifest-path kernel/Cargo.toml \
		-- --emit=link=$@ --emit=asm=$@.s --cfg device=\"imx8\"

build/$(ARCH)/imx8/kernel.bin: build/$(ARCH)/imx8/kernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

build/$(ARCH)/qemu-virt/kernel: */src/*.rs */src/*.s Cargo.toml
	mkdir -p $(@D)
	echo $$RUSTFLAGS
	cargo xrustc $(CARGO_OPTS) --manifest-path kernel/Cargo.toml \
		-- --emit=link=$@ --emit=asm=$@.s --cfg device=\"qemu-virt\"

build/$(ARCH)/qemu-virt/kernel.bin: build/$(ARCH))/qemu-virt/kernel link.ld
	$(LD) --oformat binary -m aarch64elf --script link.ld --output $@ $<

doc:
	cargo doc --manifest-path kernel/Cargo.toml --target=aarch64-unknown-linux-gnu

doc-open:
	cargo doc --manifest-path kernel/Cargo.toml --target=aarch64-unknown-linux-gnu --open

clean:
	cargo clean
	rm -rf build

.PHONY: clean doc
