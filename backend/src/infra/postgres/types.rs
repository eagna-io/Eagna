#[derive(Debug, Clone, PartialEq, Eq, DbEnum)]
#[DieselType = "Market_status"]
pub enum MarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}
