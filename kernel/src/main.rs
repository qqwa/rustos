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
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Error: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    // let midr;
    let mpidr;
    unsafe {
        println!("Hello World! 0x{:x}", 42);
        println!("CurrentEL:    {:b}", armv8_a::raw::get_current_el());
        println!("NZCV:         {:b}", armv8_a::raw::get_nzcv());
        armv8_a::raw::set_nzcv(0b1000000000000000000000000000000);
        println!("NZCV:         {:b}", armv8_a::raw::get_nzcv());
        println!("MIDR:         {:b}", armv8_a::raw::get_midr_el1());
        println!("MPIDR:        {:b}", armv8_a::raw::get_mpidr_el1());
        // midr = armv8_a::MPIDR(armv8_a::raw::get_vmpidr_el2());
        mpidr = armv8_a::MPIDR(armv8_a::raw::get_mpidr_el1());
    }
    // println!("{:?}", midr);
    println!("{:?}", mpidr);
    loop {}
}

