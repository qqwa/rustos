ARCH=aarch64
TARGET=$(ARCH)-unknown-none
LD=ld.lld

CARGO_OPTS=--target=$(TARGET) --release
RUST_TARGET_PATH=$(PWD)/targets
RUSTFLAGS=-C soft-float

export RUSTFLAGS
export RUST_TARGET_PATH
export RUSTDOC

all: build/$(ARCH)/kernel.bin

build/$(ARCH)/kernel: $(shell find kernel) config.toml targets/$(TARGET).json
	mkdir -p build/$(ARCH)
	cargo xrustc $(CARGO_OPTS) --manifest-path kernel/Cargo.toml \
		-- --emit=link=$@ --emit=asm=$@.s \
		-Z pre-link-arg=--script=kernel/src/arch/$(ARCH)/link.ld

build/$(ARCH)/kernel.bin: build/$(ARCH)/kernel
	$(LD) --oformat binary -m aarch64elf --script kernel/src/arch/$(ARCH)/link.ld --output $@ $<

clean:
	cargo clean
	rm -rf build

.PHONY: clean doc
