/// Read Register
macro_rules! rreg {
    (
        $(#[$m:meta])*
        $function:ident, $type:ty, $instruction:expr, $register:expr
    ) => {
        $(#[$m])*
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
    (
        $(#[$m:meta])*
        $function:ident, $type:ty, $instruction:expr, $register:expr
    ) => {
        $(#[$m])*
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

// special-purpose registers

rreg!(
    /// determine the current Exception level
    get_current_el, u32, "mrs", "CurrentEL"
);

rreg!(
    /// {D, A, I, F} interrupt mask bits
    get_daif, u32, "mrs", "DAIF"
);

rreg!(
    /// get the address to return to for an exception return from EL1
    get_elr_el1, usize, "mrs", "ELR_EL1"
);
wreg!(
    /// set the address to return to for an exception return from EL1
    set_elr_el1, usize, "msr", "ELR_EL1"
);

rreg!(
    /// get the address to return to for an exception return from EL2
    get_elr_el2, usize, "mrs", "ELR_EL2"
);
wreg!(
    /// set the address to return to for an exception return from EL2
    set_elr_el2, usize, "msr", "ELR_EL2"
);

rreg!(
    /// set the address to return to for an exception return from EL3
    get_elr_el3, usize, "mrs", "ELR_EL3"
);
wreg!(
    /// get the address to return to for an exception return from EL3
    set_elr_el3, usize, "msr", "ELR_EL3"
);

rreg!(
    /// provides control of floating-point operation
    set_fpcr, u32, "mrs", "FPCR"
);
wreg!(
    /// provides control of floating-point operation
    get_fpcr, u32, "msr", "FPCR"
);

rreg!(
    /// provides floating-point status information
    set_fpsr, u32, "mrs", "FPSR"
);
wreg!(
    /// provides floating-point status information
    get_fpsr, u32, "msr", "FPSR"
);

rreg!(
    /// {N, Z, C, V} condition flags
    get_nzcv, u32, "mrs", "NZCV"
);
wreg!(
    /// {N, Z, C, V} condition flags
    set_nzcv, u32, "msr", "NZCV"
);

#[cfg(any(target_feature = "v8.1a", target_feature = "v8.2a", target_feature = "v8.3a"))]
rreg!(
    /// Privileged Access Never bit
    get_pan, u32, "mrs", "PAN"
);
#[cfg(any(target_feature = "v8.1a", target_feature = "v8.2a", target_feature = "v8.3a"))]
wreg!(
    /// Privileged Access Never bit
    set_pan, u32, "msr", "PAN"
);

rreg!(
    /// holds the stack pointer for EL0
    get_sp_el0, usize, "mrs", "SP_EL0"
);
wreg!(
    /// sets the stack pointer for EL0
    set_sp_el0, usize, "msr", "SP_EL0"
);

rreg!(
    /// holds the stack pointer for EL1
    get_sp_el1, usize, "mrs", "SP_EL1"
);
wreg!(
    /// sets the stack pointer for EL1
    set_sp_el1, usize, "msr", "SP_EL1"
);

rreg!(
    /// holds the stack pointer for EL2
    get_sp_el2, usize, "mrs", "SP_EL2"
);
wreg!(
    /// sets the stack pointer for EL2
    set_sp_el2, usize, "msr", "SP_EL2"
);

rreg!(
    /// holds the stack pointer for EL3
    get_sp_el3, usize, "mrs", "SP_EL3"
);
wreg!(
    /// sets the stack pointer for EL3
    set_sp_el3, usize, "msr", "SP_EL3"
);

rreg!(
    /// Stack Pointer Select between SP_EL0 and SP_ELx
    get_sp_sel, u32, "mrs", "SPSel"
);
wreg!(
    /// Stack Pointer Select between SP_EL0 and SP_ELx
    set_sp_sel, u32, "msr", "SPSel"
);

rreg!(
    /// holds process state on taking an exception to AArch32 Abort mode
    get_spsr_abt, u32, "mrs", "SPSR_abt"
);
wreg!(
    /// holds process state on taking an exception to AArch32 Abort mode
    set_spsr_abt, u32, "msr", "SPSR_abt"
);

rreg!(
    /// holds process state on taking an exception to AArch64 EL1
    get_spsr_el1, u32, "mrs", "SPSR_EL1"
);
wreg!(
    /// holds process state on taking an exception to AArch64 EL1
    set_spsr_el1, u32, "msr", "SPSR_EL1"
);

rreg!(
    /// holds process state on taking an exception to AArch64 EL2
    get_spsr_el2, u32, "mrs", "SPSR_EL2"
);
wreg!(
    /// holds process state on taking an exception to AArch64 EL2
    set_spsr_el2, u32, "msr", "SPSR_EL2"
);

rreg!(
    /// holds process state on taking an exception to AArch64 EL3
    get_spsr_el3, u32, "mrs", "SPSR_EL3"
);
wreg!(
    /// holds process state on taking an exception to AArch64 EL3
    set_spsr_el3, u32, "msr", "SPSR_EL3"
);

rreg!(
    /// holds process state on taking an exception to AArch32 FIQ mode
    get_spsr_fiq, u32, "mrs", "SPSR_fiq"
);
wreg!(
    /// holds process state on taking an exception to AArch32 FIQ mode
    set_spsr_fiq, u32, "msr", "SPSR_fiq"
);

rreg!(
    /// holds process state on taking an exception to AArch32 IRQ mode
    get_spsr_irq, u32, "mrs", "SPSR_irq"
);
wreg!(
    /// holds process state on taking an exception to AArch32 IRQ mode
    set_spsr_irq, u32, "msr", "SPSR_irq"
);

rreg!(
    /// holds process state on taking an exception to AArch32 Undefined mode
    get_spsr_und, u32, "mrs", "SPSR_und"
);
wreg!(
    /// holds process state on taking an exception to AArch32 Undefined mode
    set_spsr_und, u32, "msr", "SPSR_und"
);

rreg!(
    /// holds the address to return to for a return from Debug state
    get_dlr_el0, usize, "mrs", "DLR_EL0"
);
wreg!(
    /// holds the address to return to for a return from Debug state
    set_dlr_el0, usize, "msr", "DLR_EL0"
);

rreg!(
    /// holds process state on entry to Debug state
    get_dspsr_el0, u32, "mrs", "DSPSR_EL0"
);
wreg!(
    /// holds process state on entry to Debug state
    set_dspsr_el0, u32, "msr", "DSPSR_EL0"
);

// General system control registers

rreg!(
    get_midr_el1, u32, "mrs", "MIDR_EL1"
);

rreg!(
    get_mpidr_el1, u64, "mrs", "MPIDR_EL1"
);

rreg!(
    get_vmpidr_el2, u64, "mrs", "VMPIDR_EL2"
);

rreg!(
    get_cpacr_el1, u32, "mrs", "CPACR_EL1"
);
wreg!(
    set_cpacr_el1, u32, "msr", "CPACR_EL1"
);

rreg!(
    get_tpidr_el1, u64, "mrs", "TPIDR_EL1"
);
wreg!(
    set_tpidr_el1, u64, "msr", "TPIDR_EL1"
);

rreg!(
    get_sctlr_el1, u32, "mrs", "SCTLR_EL1"
);
wreg!(
    set_sctlr_el1, u32, "msr", "SCTLR_EL1"
);

rreg!(
    get_ttbr0_el1, u64, "mrs", "TTBR0_EL1"
);
wreg!(
    wet_ttbr0_el1, u64, "msr", "TTBR0_EL1"
);

rreg!(
    get_far_el1, u64, "mrs", "FAR_EL1"
);
