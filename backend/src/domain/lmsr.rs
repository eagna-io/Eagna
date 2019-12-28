use crate::domain::market::{AmountCoin, AmountToken};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, From)]
pub struct B(u32);

impl B {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

pub fn cost<I>(b: B, distributions: I) -> AmountCoin
where
    I: Iterator<Item = AmountToken>,
{
    let cost: f64 = distributions
        .map(|amount| (f64::from(amount.as_i32()) / f64::from(b.0)).exp())
        .sum::<f64>()
        .ln()
        * f64::from(b.0);
    let normalized_cost = cost * 1000f64;

    // f64 から u32 への cast では 0 方向に自動的に丸められる
    AmountCoin::from(normalized_cost as i32)
}
