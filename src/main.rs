#![feature(lang_items)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(used)]
#![feature(naked_functions)]
#![feature(asm)]

// global_asm!(include_str!("startup.s"));

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32, _column: u32) -> !
{
    loop {}
}

const N: usize = 6;

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
static INTERRUPTS: [Option<extern "C" fn()>; N] = [
    Some(RESET),
    None,
    None,
    None,
    None,
    None,
];

extern "C" {
    // #[linkage = "weak"]
    static stack_top: usize;
}

#[linkage = "weak"]
#[naked]
#[no_mangle]
extern "C" fn RESET() {
    let pointer: usize;
    unsafe {
	    // asm!("LDR $0, [pc, #4]":"=r"(pointer):::"volatile");
	    // asm!("LDR $0, [$1]":"=r"(pointer):"r"(stack_top)::"volatile");
        asm!("MOV sp, $0"::"r"(*&stack_top)::"volatile");
	    asm!("BL rust_entry");
	    asm!("B .");
    }
}

use core::ptr::{read_volatile, write_volatile};

const RCC: usize            = 0x40021000;
const RCC_APB2ENR: usize    = RCC + 0x18;
const RCC_APB1ENR: usize    = RCC + 0x1c;

const GPIOA: usize          = 0x09030000;
const GPIOA_CRL: usize      = GPIOA + 0x00;
const GPIOA_CHR: usize      = GPIOA + 0x04;

const USART2: usize         = 0x09000000;
const USART2_SR: usize      = USART2 + 0x00;
const USART2_DR: usize      = USART2 + 0x04;
const USART2_CR1: usize     = USART2 + 0x0c;

const USART_FLAG_TXE: usize = 0x00000080;

#[inline(never)]
unsafe fn print_char(c: char) {

    // while *(USART2_SR as *const usize) & USART_FLAG_TXE != 0 {
    //     write_volatile(USART2_DR as *mut _, c as u32);
    // }

    let UART = 0x09000000 as *const u32 as *mut u32;

    *UART = c as u32;

}

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    unsafe {
        // write_volatile(RCC_APB2ENR as *mut _, *(RCC_APB2ENR as *mut usize) | 0x5);
        // write_volatile(RCC_APB1ENR as *mut _, *(RCC_APB1ENR as *mut usize) | 0x00020000);

        // write_volatile(GPIOA_CRL as *mut _, 0x00004b00);
        // write_volatile(GPIOA_CHR as *mut _, 0x44444444);

        // write_volatile(USART2_CR1 as *mut _, 0x0000000c);
        // write_volatile(USART2_CR1 as *mut _, *(USART2_CR1 as *const usize) | 0x00002000);

        print_char('H');
        print_char('a');
        print_char('l');
        print_char('l');
        print_char('o');
        print_char('\n');
    }
    loop {}
}
