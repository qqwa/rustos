ENTRY(_head)

SECTIONS
{
    . = 0x40480000;
    /* .vector_table.interrupts . : { *(.text) } */
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss COMMON) }
    .rodata : { *(.rodata) }
    . = ALIGN(8);
    . = . + 0x4000;
    stack_top = .;
}
