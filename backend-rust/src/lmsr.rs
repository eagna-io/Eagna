#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct B(pub u32);

pub struct Cost(i32);

impl std::ops::Sub for Cost {
    type Output = i32;

    fn sub(self, rhs: Cost) -> i32 {
        self.0 - rhs.0
    }
}

pub fn cost<I>(b: B, distributions: I) -> Cost
where
    I: Iterator<Item = i32>,
{
    let cost: f64 = distributions
        .map(|amount| (f64::from(amount) / f64::from(b.0)).exp())
        .sum::<f64>()
        .ln()
        * f64::from(b.0);
    let normalized_cost = cost * 1000f64;

    // f64 から u32 への cast では 0 方向に自動的に丸められる
    Cost(normalized_cost as i32)
}
