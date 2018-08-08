#[cfg(target_arch = "aarch64")]
#[macro_use]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;

pub trait ArchImpl {
    fn init(&mut self);
    fn name() -> &'static str;
    fn exceptions_init(&mut self);
    fn exceptions_start(&mut self);
    fn exceptions_pause(&mut self);
}