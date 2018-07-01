pub mod config;
pub mod print;
#[macro_use]
pub mod macros;
pub mod exceptions;

global_asm!(include_str!("header.s"));
global_asm!(include_str!("init.s"));

pub fn init() {
    // dont trap SVE and SIMD instructions/reg accesses
    let mut cpacr = 0;
    cpacr += 0b11 << 20;
    unsafe {
        ::armv8_a::raw::set_cpacr_el1(cpacr);
    }
}
