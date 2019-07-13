#[derive(Debug, PartialEq, Eq, DbEnum)]
#[DieselType = "Market_status"]
pub enum MarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}

#[derive(Debug, PartialEq, Eq, DbEnum)]
#[DieselType = "Order_type"]
pub enum OrderType {
    Normal,
    CoinSupply,
    Reward,
}
