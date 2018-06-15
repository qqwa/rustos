.weak   _RESET
.extern __stack_top
.section ".text.RESET","aw"
_RESET:
    msr spsel, 0
    msr spsel, 1
    adrp    x8, __stack_top		// defined by the linker script
    mov sp, x8
    bl rust_entry
    b .
