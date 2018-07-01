#![feature(panic_implementation)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(used)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(u128_type)]

extern crate armv8_a;
#[macro_use]
extern crate bitfield;

#[macro_use]
mod arch;
mod uart;

#[panic_implementation]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Error: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    arch::init();
    unsafe {
        asm!("svc #2");
        asm!("brk #2");

        asm!("svc #9");
        asm!("brk #9");
    }
    loop {}
}

