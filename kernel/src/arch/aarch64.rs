pub mod config;
pub mod print;
pub mod exceptions;

global_asm!(include_str!("aarch64/header.s"));
global_asm!(include_str!("aarch64/init.s"));

pub struct Arch;

impl Arch {
    pub fn new() -> Arch {
        Arch
    }
}

impl crate::arch::ArchImpl for Arch {
    fn init(&mut self) {
        init();
    }

    fn name() -> &'static str {
        "aarch64"
    }

    fn exceptions_init(&mut self) {
        // nothing to do here, already handled by init.s
    }

    fn exceptions_start(&mut self) {
        unimplemented!()
    }

    fn exceptions_pause(&mut self) {
        unimplemented!()
    }
}

pub fn init() {
    // dont trap SVE and SIMD instructions/reg accesses
    let mut cpacr = 0;
    cpacr += 0b11 << 20;
    cpacr += 0b11 << 16;
    unsafe {
        ::armv8_a::raw::set_cpacr_el1(cpacr);
    }
}
