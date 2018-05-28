#!/bin/bash

qemu-system-aarch64 -M virt -cpu cortex-a53 -m 128M  -smp 1 -nographic -kernel build/aarch64/kernel.bin
