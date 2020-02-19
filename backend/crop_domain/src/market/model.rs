pub mod num;

use crate::account::model::AccountId;
use crate::market::order::model::Order;
use std::collections::HashMap;
use uuid::Uuid;

use num::ShareNum;

pub struct Market {
    pub id: MarketId,
    pub orders: Vec<Order>,

    share_distri: HashMap<OutcomeId, ShareNum>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MarketId(Uuid);

impl Market {
    pub fn new_order(&mut self, outcome: OutcomeId, account: AccountId) -> Order {
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub id: OutcomeId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutcomeId(Uuid);
