pub struct Writer {}

impl Writer {
    pub fn new() -> Writer {
        Writer {}
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            unsafe {
                *super::config::UART = c as u32;
            }
        }
        Ok(())
    }
}
