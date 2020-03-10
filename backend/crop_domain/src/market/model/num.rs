use derive_more::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TipNum(pub(in crate::market) i32);

impl TipNum {
    pub fn as_i32(self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AddAssign, Add)]
pub struct ShareNum(pub(in crate::market) i32);

impl ShareNum {
    pub const ZERO: ShareNum = ShareNum(0);
    pub const ONE: ShareNum = ShareNum(1);

    pub fn as_i32(self) -> i32 {
        self.0
    }
}
