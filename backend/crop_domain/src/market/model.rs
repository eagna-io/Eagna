pub mod num;

use crate::account::model::AccountId;
use crate::market::order::model::Order;
use std::collections::HashMap;
use uuid::Uuid;

use num::{ShareNum, TipNum};

pub struct Market {
    pub id: MarketId,
    pub orders: Vec<Order>,

    share_distri: HashMap<OutcomeId, ShareNum>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarketId(Uuid);

impl Market {
    /// 対象のOutcomeを1つ購入する
    pub fn new_order(&mut self, outcome: OutcomeId, account: AccountId) -> Order {
        let tip_cost = self.compute_cost(outcome);
        let order = Order::new(outcome, account, tip_cost);
        // Orderを記録する
        self.orders.push(order);
        self.increment_share(outcome);
        order
    }

    /// 対象のOutcomeを1つ購入するときのTipのコストを計算する
    pub fn compute_cost(&self, outcome: OutcomeId) -> TipNum {
        todo!();
    }

    /// 対象のOutcomeのShareを1つ追加する
    fn increment_share(&mut self, outcome: OutcomeId) {
        *self.share_distri.get_mut(&outcome).unwrap() += ShareNum::ONE;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub id: OutcomeId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutcomeId(Uuid);
