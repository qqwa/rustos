#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Frame {
    simd: SIMDRegs,
    regs: Regs,
    sp: u64,
    id: u64,
    res: u64,
    x0: u64,
    lr: u64,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
struct Regs {
    x30: u64,
    x29: u64,
    x28: u64,
    x27: u64,
    x26: u64,
    x25: u64,
    x24: u64,
    x23: u64,
    x22: u64,
    x21: u64,
    x20: u64,
    x19: u64,
    x18: u64,
    x17: u64,
    x16: u64,
    x15: u64,
    x14: u64,
    x13: u64,
    x12: u64,
    x11: u64,
    x10: u64,
    x9:  u64,
    x8:  u64,
    x7:  u64,
    x6:  u64,
    x5:  u64,
    x4:  u64,
    x3:  u64,
    x2:  u64,
    x1:  u64,
}


#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
struct SIMDRegs {
    v31: u128,
    v30: u128,
    v29: u128,
    v28: u128,
    v27: u128,
    v26: u128,
    v25: u128,
    v24: u128,
    v23: u128,
    v22: u128,
    v21: u128,
    v20: u128,
    v19: u128,
    v18: u128,
    v17: u128,
    v16: u128,
    v15: u128,
    v14: u128,
    v13: u128,
    v12: u128,
    v11: u128,
    v10: u128,
    v9:  u128,
    v8:  u128,
    v7:  u128,
    v6:  u128,
    v5:  u128,
    v4:  u128,
    v3:  u128,
    v2:  u128,
    v1:  u128,
    v0:  u128,
}

impl Frame {
    pub fn setSIMDRegs(&mut self, val: u128) {
        self.simd.v0 = val;
        self.simd.v1 = val + 1;
        self.simd.v2 = val + 2;
        self.simd.v3 = val + 3;
    }
}

