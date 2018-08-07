pub mod config;
pub mod print;
#[macro_use]
pub mod macros;
pub mod exceptions;

global_asm!(include_str!("aarch64/header.s"));
global_asm!(include_str!("aarch64/init.s"));

pub fn init() {
    // dont trap SVE and SIMD instructions/reg accesses
    let mut cpacr = 0;
    cpacr += 0b11 << 20;
    cpacr += 0b11 << 16;
    unsafe {
        ::armv8_a::raw::set_cpacr_el1(cpacr);
    }
}
