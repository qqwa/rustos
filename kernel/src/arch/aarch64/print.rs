pub struct Writer {
    uart: ::uart::Uart,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            uart: ::uart::Uart::new(super::config::UART),
        }
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            unsafe {
                // *super::config::UART = c as u32;
                self.uart.print_char(c);
            }
        }
        Ok(())
    }
}
