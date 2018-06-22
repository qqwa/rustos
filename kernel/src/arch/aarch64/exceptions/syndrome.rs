// D10.2.39
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FaultKind {
    AddressSize,
    Translation,
    AccessFlag,
    Permission,
    Alignment,
    TLBConflict,
    Other(u8),
}

impl From<u32> for FaultKind {
    fn from(iss: u32) -> FaultKind {
        use self::FaultKind::*;
        let iss = InstructionAbortISS(iss);
		match iss.kind() {
			0b0000 => AddressSize,
			0b0001 => Translation,
            0b0010 => AccessFlag,
            0b0011 => Permission,
            0b0100 |
            0b0110 |
            0b0101 |
            0b0111 => Alignment,
            0b1100 => TLBConflict,
            _ => Other(iss.ifsc() as u8),
		}
    }
}

bitfield! {
    struct InstructionAbortISS(u32);
    impl Debug;
    set, _: 12, 11;
    fnv, _: 10;
    ea, _: 9;
    s1ptw, _: 7;
    ifsc, _: 5, 0;
    kind, _: 5, 2;
    lvl, _: 1, 0;
}

bitfield! {
    struct SystemCallISS(u32);
    impl Debug;
    imm16, _: 15, 0;
}

// See ARMv8-Reference-Manual D1.10.4
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Syndrome {
    Unknown,
    SVEoSIMD,
    IllegalExecutionState,
    SVC(u16),
    HVC(u16),
    SMC(u16),
    InstructionAbort {
        kind: FaultKind,
        level: u8,
    },
    PCAlignmentFault,
    DataAbort {
        kind: FaultKind,
        level: u8,
    },
    SPAlignmentFault,
    SError,
    Breakpoint,
    SoftwareStep,
    Watchpoint,
    BRK(u32),
    Other(u32),
}


bitfield! {
    struct SyndromeBits(u32);
    impl Debug;
    ec, _: 31, 26;
    il, _: 25;
    iss, _: 24, 0;
}

impl From<u32> for Syndrome {
    fn from(esr: u32) -> Syndrome {
        use self::Syndrome::*;

        let esr = SyndromeBits(esr);

        match esr.ec() {    // Page. 1877
            0b000000 => Unknown,

            0b000111 => SVEoSIMD,

            0b001110 => IllegalExecutionState,

            0b010001 |
            0b010101 => SVC(SystemCallISS(esr.iss()).imm16() as u16),

            0b010010 |
            0b010110 => HVC(0), //TODO: HVC instruction execution in AArch64 state

            0b010011 |
            0b010111 => SMC(0), //TODO: SMC instruction execution in AArch64 state

            0b100000 |
            0b100001 => InstructionAbort{ kind: FaultKind::from(esr.iss()), level: InstructionAbortISS(esr.iss()).lvl() as u8 },

            0b100010 => PCAlignmentFault, //TODO: PC alignment fault exception

            0b100100 |
            0b100101 => DataAbort{ kind: FaultKind::from(esr.iss()), level: InstructionAbortISS(esr.iss()).lvl() as u8 },

            0b100110 => SPAlignmentFault, //TODO: SP alignment fault exception

            0b101111 => SError, //TODO: SError interrupt

            0b110000 |
            0b110001 => Breakpoint, //TODO: Breakpoint exception same Exception level

            0b110010 |
            0b110100 => SoftwareStep, //TODO: Software Step exception same Exception level

            0b110100 |
            0b110101 => Watchpoint, //TODO: Watchpoint exception same Exception level

            0b111000 |
            0b111100 => BRK(esr.iss()), //TODO: BRK instruction execution in AArch64 state

            _ => Other(esr.0),
        }
    }
}

