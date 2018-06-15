/// Read Register
macro_rules! rreg {
    ($function:ident, $type:ty, $instruction:expr, $register:expr) => {
        #[inline(always)]
        pub unsafe fn $function() -> $type
        {
            let ret: $type;
            asm!(concat!($instruction, " $0, ", $register):"=r"(ret):::"volatile");
            ret
        }
    }
}

/// Write Register
macro_rules! wreg {
    ($function:ident, $type:ty, $instruction:expr, $register:expr) => {
        #[inline(always)]
        pub unsafe fn $function(val: $type) {
           asm!(concat!($instruction, " ", $register, ", $0")::"r"(val)::"volatile");
        }
    }
}

macro_rules! instr {
    ($function:ident, $instruction:expr) => {
        #[inline(always)]
        pub unsafe fn $function() {
            asm!(concat!($instruction)::::"volatile");
        }
    }
}

/// Condition Flags
rreg!(get_nzcv, u32, "mrs", "NZCV");
wreg!(set_nzcv, u32, "msr", "NZCV");

/// Interrupt Mask Bits
rreg!(get_daif, u32, "mrs", "DAIF");

/// Current Exception Level
rreg!(get_current_el, u32, "mrs", "CurrentEL");

/// Stack Pointer Select
rreg!(get_sp_sel, u32, "mrs", "SPSel");
wreg!(set_sp_sel, u32, "msr", "SPSel");

// /// Privileged Access Never
// rreg!(get_pan, u32, "mrs", "PAN");

// /// User Access Override
// rreg!(get_uao, u32, "mrs", "UAO");

