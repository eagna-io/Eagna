pub mod schema;
pub use types::*;

mod market_store;

use crate::domain::{
    models::{
        market::{ClosedMarket, Market, MarketId, OpenMarket},
        user::UserId,
    },
    services::market_store::{MarketStore, UpdateMarketLastOrderResult, UpdateMarketStatusResult},
};
use diesel::{
    pg::PgConnection,
    result::{ConnectionError, Error as PgError},
    Connection,
};

#[derive(Debug, Clone)]
pub struct PgStoreFactory {
    url: String,
}

pub struct PgStore {
    conn: PgConnection,
}

pub mod types {
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
}

impl PgStoreFactory {
    pub fn new_with_env() -> PgStoreFactory {
        let db_url = std::env::var("PG_URL").expect("PG_URL is not presented");
        PgStoreFactory::new(db_url)
    }

    pub fn new(url: String) -> PgStoreFactory {
        PgStoreFactory { url: url }
    }

    pub fn establish(&self) -> Result<PgStore, ConnectionError> {
        Ok(PgStore {
            conn: PgConnection::establish(self.url.as_str())?,
        })
    }
}

impl MarketStore for PgStore {
    type Error = PgError;

    fn query_market(&self, market_id: &MarketId) -> Result<Option<Market>, Self::Error> {
        market_store::query_market(&self.conn, market_id)
    }

    fn query_market_ids_related_to_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, Self::Error> {
        market_store::query_market_ids_related_to_user(&self.conn, user_id)
    }

    fn query_market_ids_ready_to_open(&self) -> Result<Vec<MarketId>, Self::Error> {
        market_store::query_market_ids_ready_to_open(&self.conn)
    }

    fn query_market_ids_ready_to_close(&self) -> Result<Vec<MarketId>, Self::Error> {
        market_store::query_market_ids_ready_to_close(&self.conn)
    }

    fn update_market_last_order(
        &self,
        market: &OpenMarket,
    ) -> UpdateMarketLastOrderResult<Self::Error> {
        market_store::update_market_last_order(&self.conn, market)
    }

    fn update_market_status_to_open(
        &self,
        market: &OpenMarket,
    ) -> UpdateMarketStatusResult<Self::Error> {
        market_store::update_market_status_to_open(&self.conn, market)
    }

    fn update_market_status_to_closed(
        &self,
        market: &ClosedMarket,
    ) -> UpdateMarketStatusResult<Self::Error> {
        market_store::update_market_status_to_closed(&self.conn, market)
    }
}
