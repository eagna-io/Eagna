pub mod computer;
pub mod num;

use crate::account::model::AccountName;
use crate::market::model::computer::PriceComputer;
use crate::market::order::model::Order;
use std::collections::HashMap;
use uuid::Uuid;

use num::ShareNum;

pub struct Market {
    pub id: MarketId,
    pub orders: Vec<Order>,

    // 各アウトカムどれくらいのShareが流通しているか
    shares: HashMap<OutcomeId, ShareNum>,
    price_computer: PriceComputer,
}

impl Market {
    /// 新しくMarketを作成する
    pub fn new(outcomes: &[OutcomeId]) -> Market {
        let shares = outcomes.iter().map(|id| (*id, ShareNum::ZERO)).collect();
        Market {
            id: MarketId::new(),
            orders: Vec::new(),
            shares,
            price_computer: PriceComputer::default(),
        }
    }

    /// 対象のOutcomeを1つ購入する
    pub fn vote(&mut self, account: AccountName, outcome: OutcomeId) -> Order {
        let tip_cost = self.price_computer.compute_price(&self.shares, outcome);
        let order = Order::new(outcome, account, tip_cost);
        // Orderを記録する
        self.orders.push(order);
        self.increment_share(outcome);
        order
    }

    /// 対象のOutcomeのShareを1つ追加する
    fn increment_share(&mut self, outcome: OutcomeId) {
        *self.shares.get_mut(&outcome).unwrap() += ShareNum::ONE;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarketId(pub Uuid);

impl MarketId {
    pub fn new() -> MarketId {
        MarketId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub id: OutcomeId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutcomeId(pub Uuid);

impl OutcomeId {
    pub fn new() -> OutcomeId {
        OutcomeId(Uuid::new_v4())
    }
}
