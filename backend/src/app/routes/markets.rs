mod get;
pub mod orders;
mod post;
mod put;
pub use get::{get, get_list};
pub use post::post;
pub use put::put;

use crate::domain::market::MarketStatus;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ApiMarketStatus {
    Upcoming,
    Open,
    Closed,
    Resolved,
}

impl<'a> From<&'a MarketStatus> for ApiMarketStatus {
    fn from(s: &'a MarketStatus) -> ApiMarketStatus {
        match *s {
            MarketStatus::Upcoming => ApiMarketStatus::Upcoming,
            MarketStatus::Open => ApiMarketStatus::Open,
            MarketStatus::Closed => ApiMarketStatus::Closed,
            MarketStatus::Resolved => ApiMarketStatus::Resolved,
        }
    }
}

impl Into<MarketStatus> for ApiMarketStatus {
    fn into(self) -> MarketStatus {
        match self {
            ApiMarketStatus::Upcoming => MarketStatus::Upcoming,
            ApiMarketStatus::Open => MarketStatus::Open,
            ApiMarketStatus::Closed => MarketStatus::Closed,
            ApiMarketStatus::Resolved => MarketStatus::Resolved,
        }
    }
}
