// .extern stack_pointer
.weak   _RESET
.section ".text.RESET","aw"
_RESET:
    adrp    x8, :got:stack_top		// defined by the linker script
    mov sp, x8
    bl rust_entry
    b .
