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
    pub(crate) x30: u64,
    pub(crate) x29: u64,
    pub(crate) x28: u64,
    pub(crate) x27: u64,
    pub(crate) x26: u64,
    pub(crate) x25: u64,
    pub(crate) x24: u64,
    pub(crate) x23: u64,
    pub(crate) x22: u64,
    pub(crate) x21: u64,
    pub(crate) x20: u64,
    pub(crate) x19: u64,
    pub(crate) x18: u64,
    pub(crate) x17: u64,
    pub(crate) x16: u64,
    pub(crate) x15: u64,
    pub(crate) x14: u64,
    pub(crate) x13: u64,
    pub(crate) x12: u64,
    pub(crate) x11: u64,
    pub(crate) x10: u64,
    pub(crate) x9: u64,
    pub(crate) x8: u64,
    pub(crate) x7: u64,
    pub(crate) x6: u64,
    pub(crate) x5: u64,
    pub(crate) x4: u64,
    pub(crate) x3: u64,
    pub(crate) x2: u64,
    pub(crate) x1: u64,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct SIMDRegister {
    pub(crate) v31: u128,
    pub(crate) v30: u128,
    pub(crate) v29: u128,
    pub(crate) v28: u128,
    pub(crate) v27: u128,
    pub(crate) v26: u128,
    pub(crate) v25: u128,
    pub(crate) v24: u128,
    pub(crate) v23: u128,
    pub(crate) v22: u128,
    pub(crate) v21: u128,
    pub(crate) v20: u128,
    pub(crate) v19: u128,
    pub(crate) v18: u128,
    pub(crate) v17: u128,
    pub(crate) v16: u128,
    pub(crate) v15: u128,
    pub(crate) v14: u128,
    pub(crate) v13: u128,
    pub(crate) v12: u128,
    pub(crate) v11: u128,
    pub(crate) v10: u128,
    pub(crate) v9: u128,
    pub(crate) v8: u128,
    pub(crate) v7: u128,
    pub(crate) v6: u128,
    pub(crate) v5: u128,
    pub(crate) v4: u128,
    pub(crate) v3: u128,
    pub(crate) v2: u128,
    pub(crate) v1: u128,
    pub(crate) v0: u128,
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
