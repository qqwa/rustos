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

// linkage weak is needed because the arch module is private and the function is
// not called inside of this module
#[linkage = "weak"]
#[no_mangle]
pub extern fn exception_handler(info: Info, esr: u32, frame: &mut frame::Frame) {

    unsafe {
        RECURSIVE_EXCEPTION += 1;
        if 10 < RECURSIVE_EXCEPTION {
            println!("To many recursive exceptions: entering endless loop");
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
            println!("{:?}:", syndrome);
            println!("  {:?}", info);
        },
        Syndrome::SVC(ref x) => {
            systemcall(x, frame);
        },
        x => {
            // Exceptions which don't have a own match branch, will be handled
            // by this default handler. Logging the exception and returning
            // from it.
            println!("Default exception handler encountered:");
            println!("{:02x?}", x);
        },
    }

    unsafe {
        RECURSIVE_EXCEPTION -= 1;
    }
}

fn systemcall(id: &u16, frame: &mut frame::Frame) {
    match id {
        x @ 0...10 => {
            frame.register.x8 = *x as u64;
        },
        42 => {
            println!("Printing x8 of frame: {}", frame.register.x8);
        },
        _ => {
            println!("requested unexpected syscall({})", id);
        },
    }
}

