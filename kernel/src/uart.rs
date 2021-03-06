pub struct Uart {
    base: *mut u8,
}

unsafe impl Send for Uart {}

impl Uart {
    pub fn new(base: *mut u8) -> Uart {
        Uart { base }
    }

    pub unsafe fn print_char(&self, c: char) {
        let uts = self.base.offset(Offset::UTS as isize) as *mut u32;
        // only works on imx8, because qemu has a different uart device
        if cfg!(device = "imx8") {
            // check if tx buffer is full
            while ::core::ptr::read_volatile(uts) & (1 << 4) == 1 {}
        }
        *(self.base.offset(Offset::UTXD as isize)) = c as u8;
    }
}

#[allow(dead_code)]
#[repr(isize)]
enum Offset {
    /// UART Receiver Register
    URXD = 0x0,
    /// UART Transmitter Register
    UTXD = 0x40,
    /// UART Control Register 1
    UCR1 = 0x80,
    /// UART Control Register 2
    UCR2 = 0x84,
    /// UART Control Register 3
    UCR3 = 0x88,
    /// UART Control Register 4
    UCR4 = 0x8c,
    /// UART FIFO Control Register
    UFCR = 0x90,
    /// UART Status Register 1
    USR1 = 0x94,
    /// UART Status Register 2
    USR2 = 0x98,
    /// UART Escape Character Register
    UESC = 0x9c,
    /// UART Escape Timer Register
    UTIM = 0xa0,
    /// UART BRM Incremental Register
    UBIR = 0xa4,
    /// UART BRM Modulator Register
    UBMR = 0xa8,
    /// UART Baud Rate Count Register
    UBRC = 0xac,
    /// UART One Millisecond Register
    ONEMS = 0xb0,
    /// UART Test Register
    UTS = 0xb4,
    /// UART RS-485 Mode Control Register
    UMCR = 0xb8,
}
