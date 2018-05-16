ENTRY(RESET)
OUTPUT_FORMAT(elf64-aarch64)

SECTIONS
{
    . = 0x80000;
    .vector_table.interrupts . : { *(.text) }
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss COMMON) }
    . = ALIGN(8);
    . = . + 0x4000;
    stack_top = .;
}
