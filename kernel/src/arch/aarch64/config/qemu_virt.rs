// uart device starts at 0x9000000 but because qemu uses a different uart device
// we change it so that the offset is correct for writing to the serial port
pub const UART: *mut u8 = (0x9000000 - 0x40) as *mut u8;
