use crate::domain::{market::MarketId, prize::PrizeId};
use chrono::{DateTime, Utc};
use getset::Getters;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct PointHistory(Vec<PointHistoryItem>);

impl PointHistory {
    pub fn new() -> PointHistory {
        PointHistory(Vec::new())
    }

    pub fn as_slice(&self) -> &[PointHistoryItem] {
        self.0.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PointHistoryItem> {
        self.0.iter()
    }

    pub fn sum(&self) -> u32 {
        self.iter().map(|item| item.amount_point()).sum::<i32>() as u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointHistoryItem {
    MarketReward(MarketRewardHistoryItem),
    PrizeTrade(PrizeTradeHistoryItem),
}

impl PointHistoryItem {
    pub fn amount_point(&self) -> i32 {
        match self {
            PointHistoryItem::MarketReward(ref reward) => reward.point as i32,
            PointHistoryItem::PrizeTrade(ref trade) => -(trade.point as i32),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, From)]
#[get = "pub"]
pub struct MarketRewardHistoryItem {
    point: u32,
    time: DateTime<Utc>,
    market_id: MarketId,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, From)]
#[get = "pub"]
pub struct PrizeTradeHistoryItem {
    point: u32,
    time: DateTime<Utc>,
    prize_id: PrizeId,
    status: PrizeTradeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrizeTradeStatus {
    Requested,
    Processed,
}
