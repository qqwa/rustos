.weak   _RESET
.section ".text.RESET","aw"
_RESET:
    mrs x0, MPIDR_EL1
    and x0, x0, #3
    cbz x0, bss

halt:       // halt secondary cpus
    wfe
    b halt

bss:
    ldr     x1, = __bss_start
    ldr     x2, = __bss_end

bss_loop:
    cmp x2, x1
    ble cont
    stp     xzr, xzr, [x1], #16      // Zero two bytes
    // __bss_end - __bss_start
    // we don't have any data after __bss_start so it doesn't matter if we
    // zero a few bytes to much
    b bss_loop

cont:
    ldr    x8, =__stack_top
    mov sp, x8

    // Check if we booted into el2
    mrs x0, CurrentEL
    cmp x0, 0x8
    b.eq el2
    b init

el2:
    msr SP_EL1, x8
    // Set Execution state of EL1 to aarch64
    mov x0, xzr
    // dont trap instructions related to pointer authenication
    orr x0, x0, #(1 << 41)
    // dont trap access to registers holding key values for pa
    orr x0, x0, #(1 << 40)

    orr x0, x0, #(1 << 31) // set RW bit
    msr HCR_EL2, x0

    // Set Context for EL1
    //mov x0, 0x3c5
    mov x0, 0x105
    msr SPSR_EL2, x0

    ldr x0, =init
    msr ELR_EL2, x0

    eret

init:
    ldr x0, =Vector_table
    msr VBAR_EL1, x0
    isb

    bl rust_entry


/*
            ---------
            | v31   |   context
            |   .   |       .
            |   .   |       .
            |   .   |       .
            | v0    |       .
            | SP    |       .
            | ID    |       .
            | x29   |       .
            |   .   |       .
            |   .   |       .
            |   .   |       .
            | x2    |       .
            | x1    |   context
            |   -   |   return adress (ignore)
            | x0    |   vector
            | lr    |   vector
            ---------

*/

context_save:
    stp     x2,  x1,    [SP, #-16]!
    stp     x4,  x3,    [SP, #-16]!
    stp     x6,  x5,    [SP, #-16]!
    stp     x8,  x7,    [SP, #-16]!
    stp     x10, x9,    [SP, #-16]!
    stp     x12, x11,   [SP, #-16]!
    stp     x14, x13,   [SP, #-16]!
    stp     x16, x15,   [SP, #-16]!
    stp     x18, x17,   [SP, #-16]!
    stp     x20, x19,   [SP, #-16]!
    stp     x22, x21,   [SP, #-16]!
    stp     x24, x23,   [SP, #-16]!
    stp     x26, x25,   [SP, #-16]!
    stp     x28, x27,   [SP, #-16]!
    stp     x30, x29,   [SP, #-16]!

    mrs     x1, sp_el0
    mrs     x2, tpidr_el1
    stp     x2,  x1,    [SP, #-16]!

    stp     v1,   v0,   [SP, #-32]!
    stp     v3,   v2,   [SP, #-32]!
    stp     v5,   v4,   [SP, #-32]!
    stp     v7,   v6,   [SP, #-32]!
    stp     v9,   v8,   [SP, #-32]!
    stp     v11,  v10,  [SP, #-32]!
    stp     v13,  v12,  [SP, #-32]!
    stp     v15,  v14,  [SP, #-32]!
    stp     v17,  v16,  [SP, #-32]!
    stp     v19,  v18,  [SP, #-32]!
    stp     v21,  v20,  [SP, #-32]!
    stp     v23,  v22,  [SP, #-32]!
    stp     v25,  v24,  [SP, #-32]!
    stp     v27,  v26,  [SP, #-32]!
    stp     v29,  v28,  [SP, #-32]!
    stp     v31,  v30,  [SP, #-32]!
    mrs     x1, ESR_EL1
    mov     x2, SP
    bl      exception_handler

context_restore:
    ldp     v31, v30,   [SP], #32
    ldp     v29, v28,   [SP], #32
    ldp     v27, v26,   [SP], #32
    ldp     v25, v24,   [SP], #32
    ldp     v23, v22,   [SP], #32
    ldp     v21, v20,   [SP], #32
    ldp     v19, v18,   [SP], #32
    ldp     v17, v16,   [SP], #32
    ldp     v15, v14,   [SP], #32
    ldp     v13, v12,   [SP], #32
    ldp     v11, v10,   [SP], #32
    ldp     v9,  v8,    [SP], #32
    ldp     v7,  v6,    [SP], #32
    ldp     v5,  v4,    [SP], #32
    ldp     v3,  v2,    [SP], #32
    ldp     v1,  v0,    [SP], #32

    ldp     x2,  x1,    [SP], #16
    msr     sp_el0, x1
    msr     tpidr_el1, x2

    ldp     x30, x29,   [SP], #16
    ldp     x28, x27,   [SP], #16
    ldp     x26, x25,   [SP], #16
    ldp     x24, x23,   [SP], #16
    ldp     x22, x21,   [SP], #16
    ldp     x20, x19,   [SP], #16
    ldp     x18, x17,   [SP], #16
    ldp     x16, x15,   [SP], #16
    ldp     x14, x13,   [SP], #16
    ldp     x12, x11,   [SP], #16
    ldp     x10, x9,    [SP], #16
    ldp     x8,  x7,    [SP], #16
    ldp     x6,  x5,    [SP], #16
    ldp     x4,  x3,    [SP], #16
    ldp     x2,  x1,    [SP], #16
    ret

.macro EXCEPTION, source, kind
    .balign 0x80
    stp     x0, lr, [SP, #-16]!
    mov     x0, \source
    movk    x0, \kind, LSL #16
    bl      context_save
    ldp     x0, lr, [SP], #16
    eret
.endm

// https://developer.arm.com/products/architecture/a-profile/docs/100933/latest/aarch64-exception-vector-table
.balign 0x800
Vector_table:
// CurrentEL SP0
EXCEPTION #0, #0
EXCEPTION #0, #1
EXCEPTION #0, #2
EXCEPTION #0, #3

//CurrentlEL SPx
EXCEPTION #1, #0
EXCEPTION #1, #1
EXCEPTION #1, #2
EXCEPTION #1, #3

//LowerEL AArch64
EXCEPTION #2, #0
EXCEPTION #2, #1
EXCEPTION #2, #2
EXCEPTION #2, #3

//LowerEL AArch32
EXCEPTION #3, #0
EXCEPTION #3, #1
EXCEPTION #3, #2
EXCEPTION #3, #3
