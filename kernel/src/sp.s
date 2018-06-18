.weak   _RESET
.extern __stack_top
.section ".text.RESET","aw"
_RESET:
    adrp    x8, __stack_top		// defined by the linker script
    msr SP_EL1, x8

    // Check if we booted into el2
    mrs x0, CurrentEL
    cmp x0, 0x8
    b.eq el2
    b init

el2:
    // Set Execution state of EL1 to aarch64
    mov x0, xzr
    orr x0, x0, #(1 << 41) // dont trap instructions related to pointer authenication
    orr x0, x0, #(1 << 40) // dont trap access to registers holding key values for pa

    orr x0, x0, #(1 << 31) // set RW bit
    msr HCR_EL2, x0

    // Set Context for EL1
    mov x0, 0x3c5
    msr SPSR_EL2, x0

    ldr x0, =init
    msr ELR_EL2, x0

    eret

init:
    ldr x0, =Vector_table
    msr VBAR_EL1, x0
    isb

    bl rust_entry

// https://developer.arm.com/products/architecture/a-profile/docs/100933/latest/aarch64-exception-vector-table
.balign 0x800
Vector_table:
// Exceptions el1 -> el1
.balign 0x80
b sp0_sync
.balign 0x80
b sp0_irq
.balign 0x80
b sp0_fiq
.balign 0x80
b sp0_serror
.balign 0x80
b spx_sync
.balign 0x80
b spx_irq
.balign 0x80
b spx_fiq
.balign 0x80
b spx_serror
// Exceptions el0 -> el1
.balign 0x80
b us64_sync
.balign 0x80
b us64_irq
.balign 0x80
b us64_fiq
.balign 0x80
b us64_serror
.balign 0x80
b us32_sync
.balign 0x80
b us32_irq
.balign 0x80
b us32_fiq
.balign 0x80
b us32_serror
