#[derive(Debug, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[DieselType = "Market_status"]
pub enum MarketStatus {
    Preparing,
    Open,
    Closed,
    Settled,
}

#[derive(Debug, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[DieselType = "Order_type"]
pub enum OrderType {
    Normal,
    InitialSupply,
    Settle,
}
