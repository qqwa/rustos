lazy_static! {
    static ref STDOUT: spin::Mutex<crate::uart::Uart> = spin::Mutex::new(crate::uart::Uart::new(super::config::UART));
}

pub struct Writer<'a> {
    uart: spin::MutexGuard<'a, crate::uart::Uart>,
}

impl<'a> Writer<'a> {
    pub fn new() -> Writer<'a> {
        Writer {
            uart: STDOUT.lock(),
        }
    }
}

use core::fmt;
impl<'a> fmt::Write for Writer<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            unsafe {
                self.uart.print_char(c);
                if c == '\n' {
                    self.uart.print_char('\r');
                }
            }
        }
        Ok(())
    }
}
