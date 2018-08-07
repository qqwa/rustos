pub mod frame;
mod syndrome;

use self::syndrome::Syndrome;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
   Synchronous = 0,
   IRQ = 1,
   FIQ = 2,
   SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSP0 = 0,
    CurrentSPx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

static mut RECURSIVE_EXCEPTION: u32 = 0;


#[linkage = "weak"]
#[no_mangle]
pub extern fn exception_handler(info: Info, esr: u32, frame: &mut frame::Frame) {

    unsafe {
        RECURSIVE_EXCEPTION += 1;
        if 10 < RECURSIVE_EXCEPTION {
            println!("To many unknown exceptions entering endless loop..");
            loop {}
        }
    }

    let syndrome = Syndrome::from(esr);

    if cfg!(feature="verbose-exception-handler") {
        println!("{:?}", info);
        println!("{:?}", syndrome);
        println!("{:#x?}", frame);
        println!("");
    }

    match &syndrome {
        Syndrome::Unknown => {
            println!("{:?}", info);
            println!("{:?}", syndrome);
        }
        Syndrome::BRK(ref x) => {
            println!("TODO: Starting shell");
            println!("Entering endless loop");
            loop {
                unsafe { asm!("wfe"); }
            }
        },
        Syndrome::SVC(ref x) => {
            systemcall(x, frame);
        }
        x => {
            // Exceptions which don't have a own match branch, will be handled
            // by this default handler. Logging the exception and returning
            // from it.
            println!("Default exception handler found:");
            println!("{:02x?}", x);
        }
    }

    unsafe {
        RECURSIVE_EXCEPTION -= 1;
    }
}

fn systemcall(id: &u16, frame: &mut frame::Frame) {
    match id {
        0 => {
            println!("Called Yield systemcall");
        },
        2 => {
            println!("Set tpidr_el1 to 55");
            // unsafe{ ::armv8_a::raw::set_tpidr_el1(55); }
            frame.id = 55;
        },
        _ => {
            println!("Blabla requested unexpected syscall({})", id);
        },
    }
}

