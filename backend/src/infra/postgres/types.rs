#[derive(Debug, Clone, PartialEq, Eq, DbEnum)]
#[DieselType = "Market_status"]
pub enum MarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq, DbEnum)]
#[DieselType = "Order_type"]
pub enum OrderType {
    Normal,
    CoinSupply,
    Reward,
}

#[derive(Debug, Clone, PartialEq, Eq, DbEnum)]
#[DieselType = "Prize_trade_status"]
pub enum PrizeTradeStatus {
    Requested,
    Processed,
}
