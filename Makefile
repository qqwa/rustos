CARGO = cargo
RUSTFLAGS+=-Z pre-link-arg=-nostartfiles
TARGET=aarch64-unknown-none-elf
TARGET_FILE=.cargo/$(TARGET).json

CARGO_OPTS=--target=$(TARGET_FILE)
ifdef emit
	# emit generated code eg.: emit=asm
	RUSTFLAGS+=--emit $(emit)
endif

all:
	$(MAKE) build
	$(MAKE) doc

build:
	$(CARGO) build $(CARGO_OPTS)

clean:
	$(CARGO) clean $(CARGO_OPTS)

check:
	$(CARGO) check $(CARGO_OPTS)

test:
	$(CARGO) test $(CARGO_OPTS)

doc:
	$(CARGO) doc $(CARGO_OPTS)

.PHONY: all build clean check test doc
