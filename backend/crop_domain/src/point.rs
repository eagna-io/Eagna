use std::{iter::Sum, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Add, AddAssign, Sub)]
pub struct Point(u32);

impl Point {
    pub const fn zero() -> Point {
        Point(0)
    }

    pub const fn one() -> Point {
        Point(1)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl Sum<Point> for Point {
    fn sum<I>(iter: I) -> Point
    where
        I: Iterator<Item = Point>,
    {
        iter.fold(Point::zero(), Point::add)
    }
}

impl<'a> Sum<&'a Point> for Point {
    fn sum<I>(iter: I) -> Point
    where
        I: Iterator<Item = &'a Point>,
    {
        iter.fold(Point::zero(), |a, b| a + *b)
    }
}
