use crate::domain::models::market::MarketStatus as DomainMarketStatus;

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

impl From<DomainMarketStatus> for MarketStatus {
    fn from(s: DomainMarketStatus) -> MarketStatus {
        match s {
            DomainMarketStatus::Preparing => MarketStatus::Preparing,
            DomainMarketStatus::Open => MarketStatus::Open,
            DomainMarketStatus::Closed => MarketStatus::Closed,
            DomainMarketStatus::Settled => MarketStatus::Settled,
        }
    }
}
