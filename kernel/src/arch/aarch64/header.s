// Linux ARM64 Header
// bootloader will use information specified here to load the kernel
// we should only use the first two longs for custom insturctions,
// we jump into _RESET and leave everything else 0, unless the
// magic number, which is required so that we can use a bootloader
// which expects a linux image, without anychanges to the bootlaoder
.weak   _head
.section ".text.head","aw"
_head:
    b      _RESET  // branch to kernel start, magic
    .word   0       // reserved
    .quad   0       // image load offset is 0x80000 because iamge size is 0
    .quad   0       // image size
    .quad   0       // information flags
    .quad   0       // reserved
    .quad   0       // reserved
    .quad   0       // reserved
    .byte   0x41    // magic number, "ARM\x64"
    .byte   0x52
    .byte   0x4d
    .byte   0x64
    .word   0
