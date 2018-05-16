#!/bin/bash

qemu-system-aarch64 -M virt -cpu cortex-a53 -m 128M  -smp 1 -nographic -kernel target/aarch64-unknown-none/release/phyos
