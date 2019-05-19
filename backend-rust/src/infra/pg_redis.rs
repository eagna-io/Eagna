pub mod schema;
pub use types::*;

mod market_store;
mod user_store;

use crate::domain::{
    infra::postgres::{market_store, user_store},
    models::{
        market::{ClosedMarket, Market, MarketId, OpenMarket},
        user::{User, UserId},
    },
    services::{
        market_store::{UpdateMarketLastOrderResult, UpdateMarketStatusResult},
        MarketStore, UserStore,
    },
};
use diesel::{
    pg::PgConnection,
    result::{ConnectionError, Error as PgError},
    Connection,
};

pub struct PgRedisStore {
    pg_conn: PgConn,
    redis_conn: RedisConn,
}

impl MarketStore for PgRedisStore {
    type Error = PgError;

    /// 単純な実装。
    /// リクエストのたびに、全ての情報をDBから取ってくる
    /// 将来的にはOpenMarketのみ構造体内部にキャッシュするような実装にしたい
    fn query_market(&self, market_id: &MarketId) -> Result<Option<Market>, Self::Error> {
        market_store::query_market(&self.pg_conn, market_id)
    }

    fn query_market_ids_related_to_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, Self::Error> {
        market_store::query_market_ids_related_to_user(&self.pg_conn, user_id)
    }

    fn query_market_ids_ready_to_open(&self) -> Result<Vec<MarketId>, Self::Error> {
        market_store::query_market_ids_ready_to_open(&self.pg_conn)
    }

    fn query_market_ids_ready_to_close(&self) -> Result<Vec<MarketId>, Self::Error> {
        market_store::query_market_ids_ready_to_close(&self.pg_conn)
    }

    fn update_market_last_order(
        &self,
        market: &OpenMarket,
    ) -> UpdateMarketLastOrderResult<Self::Error> {
        market_store::update_market_last_order(&self.pg_conn, market)
    }

    fn update_market_status_to_open(
        &self,
        market: &OpenMarket,
    ) -> UpdateMarketStatusResult<Self::Error> {
        market_store::update_market_status_to_open(&self.pg_conn, market)
    }

    fn update_market_status_to_closed(
        &self,
        market: &ClosedMarket,
    ) -> UpdateMarketStatusResult<Self::Error> {
        market_store::update_market_status_to_closed(&self.pg_conn, market)
    }
}

impl UserStore for PgStore {
    type Error = PgError;

    fn query_user(&self, user_id: &UserId) -> Result<Option<User>, Self::Error> {
        user_store::query_user(&self.pg_conn, user_id)
    }

    fn query_user_by_email_and_hashed_pass(
        &self,
        email: &str,
        hashed_pass: &str,
    ) -> Result<Option<User>, Self::Error> {
        user_store::query_user_by_email_and_hashed_pass(&self.pg_conn, email, hashed_pass)
    }

    fn query_all_user_ids(&self) -> Result<Vec<UserId>, Self::Error> {
        user_store::query_all_user_ids(&self.pg_conn)
    }
}
