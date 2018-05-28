#![feature(panic_implementation)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(used)]
#![feature(naked_functions)]
#![feature(asm)]

global_asm!(include_str!("head.s"));
global_asm!(include_str!("sp.s"));

#[panic_implementation]
fn panic(_info: &core::panic::PanicInfo) -> ! {
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

// print char to uart
// const UART: *mut u32 = 0x30890040 as *mut u32;
const UART: *mut u32 = 0x9000000 as *mut u32;
unsafe fn print_char(c: char) {
    *UART = c as u32;
}

fn print(data: &str) {
    for c in data.chars() {
        unsafe { print_char(c); }
    }
}

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    print("Hello World!\n");
    loop {}
}
