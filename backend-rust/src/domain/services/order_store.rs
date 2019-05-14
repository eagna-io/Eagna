use crate::domain::models::{market::MarketId, order::MarketOrders};

pub trait OrderStore {
    type Error: std::fmt::Debug;

    fn query_market_orders(&self, market_id: &MarketId) -> Result<MarketOrders, Self::Error>;
}
