#![no_std]
#![no_main]
#![feature(panic_implementation)]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(asm)]

#[macro_use]
extern crate bitfield;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod macros;
pub mod arch;
mod process;
mod scheduler;
mod uart;

use crate::arch::ArchImpl;

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Error: {}", info);
    loop {}
}

extern "C" {
    static __stack_top: usize;
    static __kernel_start: usize;
}

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    let mut arch = arch::Arch::new();
    arch.init();
    println!(
        "version:  {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("arch:     {}", arch::Arch::name());
    unsafe {
        asm!("svc #0");
        asm!("svc #42");
        asm!("svc #5");
        asm!("svc #42");
        asm!("svc #10");
        asm!("svc #42");
        asm!("svc #15");
    }
    loop {}
}

// functions to test user space applications and scheduling
#[allow(dead_code)]
fn proc1() {
    println!("Hello from proc1");
    for _ in 0..10_000_000 {
        unsafe {
            asm!("nop");
        }
    }
}

#[allow(dead_code)]
fn proc2() {
    println!("Hello from proc2");
    for _ in 0..10_000_000 {
        unsafe {
            asm!("nop");
        }
    }
}

#[allow(dead_code)]
fn proc3() {
    println!("Hello from proc3");
    for _ in 0..10_000_000 {
        unsafe {
            asm!("nop");
        }
    }
}
