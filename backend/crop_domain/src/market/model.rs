pub mod computer;
pub mod num;
pub mod price_history;

use crate::account::model::AccountName;
use crate::market::model::computer::PriceComputer;
use crate::market::order::model::Order;
use crop_primitive::string::String as MyString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use num::ShareNum;

/// `Market` は1つの `Question` をもつ。(現在はtitleとして保存)
/// 1つの `Question` には `realize` と `unrealize` 2つの `Outcome` がある。
pub struct Market {
    /*
     * ==========
     * Immutable
     * ==========
     */
    pub id: MarketId,
    pub title: MyString,

    /*
     * ===========
     * Mutable
     * ===========
     */
    // これ保存しておく必要ある？
    orders: Vec<Order>,
    // 直近30minの1秒刻みの価格の推移履歴
    // price_history: PriceHistory,
    // 各アウトカムどれくらいのShareが流通しているか
    shares: HashMap<Outcome, ShareNum>,
    price_computer: PriceComputer,
}

impl Market {
    /// 新しくMarketを作成する
    pub fn new(title: MyString) -> Market {
        let shares = [Outcome::Realize, Outcome::Unrealize]
            .iter()
            .map(|o| (*o, ShareNum::ZERO))
            .collect();
        Market {
            id: MarketId::new(),
            title,
            orders: Vec::new(),
            shares,
            price_computer: PriceComputer::default(),
        }
    }

    /// 対象のOutcomeを1つ購入する
    pub fn vote(&mut self, account: AccountName, outcome: Outcome) -> &Order {
        let tip_cost = self.price_computer.compute_price(&self.shares, outcome);
        let order = Order::new(outcome, account, tip_cost);
        // Orderを記録する
        self.orders.push(order);
        self.increment_share(outcome);
        self.orders.last().unwrap()
    }

    /// 対象のOutcomeのShareを1つ追加する
    fn increment_share(&mut self, outcome: Outcome) {
        *self.shares.get_mut(&outcome).unwrap() += ShareNum::ONE;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct MarketId(pub Uuid);

impl MarketId {
    pub fn new() -> MarketId {
        MarketId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum Outcome {
    #[serde(rename = "realize")]
    Realize,
    #[serde(rename = "unrealize")]
    Unrealize,
}
