#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Frame {
    pub simd: SIMDRegister,
    pub id: u64,
    pub sp: u64,
    pub register: Register,
    res: u64,
    pub x0: u64,
    pub lr: u64,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Register {
    crate x30: u64,
    crate x29: u64,
    crate x28: u64,
    crate x27: u64,
    crate x26: u64,
    crate x25: u64,
    crate x24: u64,
    crate x23: u64,
    crate x22: u64,
    crate x21: u64,
    crate x20: u64,
    crate x19: u64,
    crate x18: u64,
    crate x17: u64,
    crate x16: u64,
    crate x15: u64,
    crate x14: u64,
    crate x13: u64,
    crate x12: u64,
    crate x11: u64,
    crate x10: u64,
    crate x9:  u64,
    crate x8:  u64,
    crate x7:  u64,
    crate x6:  u64,
    crate x5:  u64,
    crate x4:  u64,
    crate x3:  u64,
    crate x2:  u64,
    crate x1:  u64,
}


#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct SIMDRegister {
    crate v31: u128,
    crate v30: u128,
    crate v29: u128,
    crate v28: u128,
    crate v27: u128,
    crate v26: u128,
    crate v25: u128,
    crate v24: u128,
    crate v23: u128,
    crate v22: u128,
    crate v21: u128,
    crate v20: u128,
    crate v19: u128,
    crate v18: u128,
    crate v17: u128,
    crate v16: u128,
    crate v15: u128,
    crate v14: u128,
    crate v13: u128,
    crate v12: u128,
    crate v11: u128,
    crate v10: u128,
    crate v9:  u128,
    crate v8:  u128,
    crate v7:  u128,
    crate v6:  u128,
    crate v5:  u128,
    crate v4:  u128,
    crate v3:  u128,
    crate v2:  u128,
    crate v1:  u128,
    crate v0:  u128,
}

impl Frame {

    pub fn new(id: u64, lr: u64, sp: u64) -> Frame {
        Frame {
            id,
            lr,
            sp,
            ..Default::default()
        }
    }
}

