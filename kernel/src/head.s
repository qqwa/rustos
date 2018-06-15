// Linux ARM64 Header
// bootloader will use information specified here to load the kernel
// we should only use the first two longs for custom insturctions,
// we jump into _RESET and leave everything else 0, unless the
// magic number, which is required so that we can use a bootloader
// which expects a linux image, without anychanges to the bootlaoder
//
// u32 code0;			        /* Executable code */
// u32 code1;			        /* Executable code */
// u64 text_offset;		    /* Image load offset, little endian */
// u64 image_size;		        /* Effective Image size, little endian */
// u64 flags;			        /* kernel flags, little endian */
// u64 res2	= 0;		    /* reserved */
// u64 res3	= 0;		    /* reserved */
// u64 res4	= 0;		    /* reserved */
// u32 magic	= 0x644d5241;	/* Magic number, little endian, "ARM\x64" */
// u32 res5;			        /* reserved (used for PE COFF offset) */

.weak   _head
.section ".text.head","aw"
_head:
    b      _RESET  // branch to kernel start, magic
    .long   0       // reserved
    .quad   0x00080000       // Image load offset
    .quad   0       // Effective size of kernel image
    .quad   0       // Information flags
    .quad   0       // reserved
    .quad   0       // reserved
    .quad   0       // reserved
    .byte   0x41    // Magic number, "ARM\x64"
    .byte   0x52
    .byte   0x4d
    .byte   0x64
    .word   0
