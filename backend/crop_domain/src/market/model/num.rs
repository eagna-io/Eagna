use derive_more::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TipNum(isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, AddAssign, Add)]
pub struct ShareNum(isize);

impl ShareNum {
    pub const ONE: ShareNum = ShareNum(1);
}
