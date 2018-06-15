#![feature(panic_implementation)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(used)]
#![feature(naked_functions)]
#![feature(asm)]

extern crate armv8_a;

#[macro_use]
mod arch;
mod uart;

global_asm!(include_str!("head.s"));
global_asm!(include_str!("sp.s"));

#[panic_implementation]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_begin_unwind(_fmt: ::core::fmt::Arguments, _file: &str, _line: u32) -> ! {
    loop {}
}

const N: usize = 6;
// #[link_section = ".vector_table.interrupts"]
// #[no_mangle]
// #[used]
// static INTERRUPTS: [Option<extern "C" fn()>; N] = [
//     Some(RESET),
//     None,
//     None,
//     None,
//     None,
//     None,
// ];

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    println!("Hello World! 0x{:x}", 42);
    unsafe {
        println!("CurrentEL:    {:b}", armv8_a::raw::get_current_el());
        println!("NZCV:        {:b}", armv8_a::raw::get_nzcv());
        armv8_a::raw::set_nzcv(0b1000000000000000000000000000000);
        println!("NZCV:        {:b}", armv8_a::raw::get_nzcv());
    }
    loop {}
}
