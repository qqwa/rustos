#![no_std]
#![feature(asm)]

#[macro_use]
extern crate bitfield;

pub mod raw;

bitfield!{
    pub struct MIDR(u32);
    impl Debug;
    revision, _: 3, 0;
    part_num, _: 15, 4;
    architecture, _: 19, 16;
    variant, _: 23, 20;
    implementer, _: 31, 24;
}

bitfield!{
    pub struct MPIDR(u64);
    impl Debug;
    aff0, _: 7, 0;
    aff1, _: 15, 8;
    aff2, _: 23, 16;
    mt, _: 24;
    u, _: 30;
    asd, _: 31;
    aff3, _: 39, 32;
}

bitfield!{
    pub struct SPSR(u32);
    impl Debug;
    n, _: 31;
    z, _: 30;
    c, _: 29;
    v, _: 28;
    uao, _: 23;
    pan, _: 22;
    ss, _: 21;
    il, _: 20;
    d, _: 9;
    a, _: 8;
    i, _: 7;
    f, _: 6;
    m, _: 4;
    mode, _: 3, 0;
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
