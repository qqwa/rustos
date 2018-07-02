#![feature(panic_implementation)]
#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(used)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(u128_type)]
#![feature(const_fn)]
#![feature(optin_builtin_traits)]
#![feature(alloc, allocator_api, global_allocator)]
#![feature(ptr_internals)]

#[macro_use]
extern crate alloc;

extern crate armv8_a;
#[macro_use]
extern crate bitfield;
extern crate linked_list_allocator;

#[macro_use]
mod arch;
mod uart;
mod process;
mod scheduler;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
		// let heap_start = __stack_top + 4096;
		let heap_start = __kernel_start;
		let heap_size = 4096;
		println!("HEAP: {} - {}", heap_start, heap_start + heap_size);
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[lang = "oom"]
#[no_mangle]
pub extern fn rust_oom() -> ! {
    panic!("kernel memory allocation failed");
}

#[panic_implementation]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Error: {}", info);
    loop {}
}

extern {
    static __stack_top: usize;
    static __kernel_start: usize;
}

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
	println!("help");
    arch::init();
	init_heap();
    unsafe {

        // let scheduler = &process::SCHEDULER;
        // let tmp: *const Option<&process::Scheduler> = scheduler;
        // let tmp_mut: *mut Option<&process::Scheduler> = tmp as *mut Option<&process::Scheduler>;
        // println!("{:?}", *tmp_mut);
        // let mut scheduler = process::Scheduler::new(stack, 4096);
        // *tmp_mut = Some(&scheduler);
        // println!("{:?}", *tmp_mut);

		println!("Create Scheduler");
        let stack = unsafe { &__stack_top as *const _ as usize };
		let mut scheduler = scheduler::Scheduler::new(stack, 4096);

		// println!("Add processes");
        // scheduler.add(proc1 as fn());
        // scheduler.add(proc2 as fn());
        // scheduler.add(proc3 as fn());

        // scheduler.start();

        asm!("svc #0");
        proc1();
        asm!("svc #2");
        proc2();
        asm!("svc #9");

        // println!("{:#?}", process::PROCESSES);
        // asm!("brk #9");

        loop {}
    }
}

fn proc1() {
    println!("Hello from proc1");
    for _ in 0..10_000_000 {
        unsafe { asm!("nop"); }
    }
    unsafe { asm!("svc #0"); }
}

fn proc2() {
    println!("Hello from proc2");
    for _ in 0..10_000_000 {
        unsafe { asm!("nop"); }
    }
    unsafe { asm!("svc #0"); }
}

fn proc3() {
    println!("Hello from proc3");
    for _ in 0..10_000_000 {
        unsafe { asm!("nop"); }
    }
    unsafe { asm!("svc #0"); }
}

