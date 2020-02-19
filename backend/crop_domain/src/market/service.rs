use crate::market::model::{
    num::{ShareNum, TipNum},
    OutcomeId,
};
use std::collections::HashMap;

pub enum PriceComputer {
    LMSR(LMSR),
}

impl PriceComputer {
    /// 対象のOutcomeを1つ購入するときの価格(Tip)を計算する
    pub fn compute_price(
        &self,
        share_distri: &HashMap<OutcomeId, ShareNum>,
        outcome: OutcomeId,
    ) -> TipNum {
        match self {
            PriceComputer::LMSR(lmsr) => lmsr.compute_price(share_distri, outcome),
        }
    }
}

impl Default for PriceComputer {
    fn default() -> PriceComputer {
        PriceComputer::LMSR(LMSR::default())
    }
}

pub struct LMSR {
    b: f64,
}

impl LMSR {
    fn new(b: f64) -> LMSR {
        LMSR { b }
    }

    // # NOTE
    // もっと最適化できる
    // テストを書きながら最適化する
    fn compute_price(
        &self,
        cur_distri: &HashMap<OutcomeId, ShareNum>,
        outcome: OutcomeId,
    ) -> TipNum {
        let cur_cost = self.compute_cost(cur_distri.values().copied());

        let new_distri = cur_distri.iter().map(|(o, n)| {
            if *o == outcome {
                *n + ShareNum::ONE
            } else {
                *n
            }
        });
        let new_cost = self.compute_cost(new_distri);

        TipNum((new_cost - cur_cost) as i32)
    }

    fn compute_cost<I>(&self, distribution: I) -> u32
    where
        I: Iterator<Item = ShareNum>,
    {
        let real_cost = distribution
            .map(|n| (f64::from(n.as_i32()) / self.b).exp()) // exp(n/b)
            .sum::<f64>()
            .ln()
            * self.b;
        let normalized_cost = real_cost * 1000f64;

        // f64からu32へのcastでは0方向に丸められる（切り捨て）
        normalized_cost as u32
    }
}

impl Default for LMSR {
    fn default() -> LMSR {
        LMSR::new(30f64)
    }
}
