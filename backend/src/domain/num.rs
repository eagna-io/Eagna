#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Add, Sub, Neg, AddAssign, From)]
pub struct Point(i32);

impl Point {
    pub const fn zero() -> Point {
        Point(0)
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }
}
