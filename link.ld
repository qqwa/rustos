ENTRY(_head)

SECTIONS
{
    . = 0x40080000;
    /* . = ALIGN(4096); */
    /* .vector_table.interrupts . : { *(.text) } */
    .text : { *(.text) . = ALIGN(8); }
    .rodata : { *(.rodata) . = ALIGN(8); }
    /* .bss : { *(.bss COMMON) . = ALIGN(8); } */
    .data : {
        *(.bss) . = ALIGN(8);
        *(.data) . = ALIGN(8);
    }
    . = ALIGN(8);
    . = . + 0x4096;
    __stack_top = .;

}
