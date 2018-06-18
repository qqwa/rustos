fn default_handler() {
    println!("Exception default handler");
    unsafe {
        asm!("eret");
    }
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn sp0_sync() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn sp0_irq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn sp0_fiq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn sp0_serror() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn spx_sync() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn spx_irq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn spx_fiq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn spx_serror() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us64_sync() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us64_irq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us64_fiq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us64_serror() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us32_sync() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us32_irq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us32_fiq() -> ! {
    default_handler();
    loop {}
}

#[linkage = "weak"]
#[no_mangle]
#[naked]
pub extern "C" fn us32_serror() -> ! {
    default_handler();
    loop {}
}
