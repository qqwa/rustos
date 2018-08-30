ENTRY(_head)

SECTIONS
{
    . = 0x40080000;
    __kernel_start = .;
    .text : { *(.text*) . = ALIGN(8); }
    .rodata : { *(.rodata*) . = ALIGN(8); }
    .data : { *(.data*) . = ALIGN(8); }
    .tdata : { *(.tdata*) . = ALIGN(8); }
    .bss : {
        __bss_start = .;
        *(.bss*) . = ALIGN(8);
        __bss_end = .;
    }
    .tbss : {
        __tbss_start = .;
        *(.tbss*) . = ALIGN(8);
        __tbss_end = .;
    }
    __kernel_end = .;
    . = . + 0x4096;
    . = ALIGN(16);
    __stack_top = .;
}
