pub struct PointTokenRate(u32);

pub const POINT_TOKEN_RATE: PointTokenRate = PointTokenRate(1000);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Neg, AddAssign, From)]
pub struct Point(i32);

impl Point {
    pub const fn zero() -> Point {
        Point(0)
    }

    pub fn as_i32(self) -> i32 {
        self.0
    }
}
