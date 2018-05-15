#!/bin/bash

qemu-system-aarch64 \
	-M virt \
	-cpu cortex-a53 \
	-m 512 \
	-drive format=raw,file=target/aarch64-unknown-none/release/libkernel.a \
	--nographic
