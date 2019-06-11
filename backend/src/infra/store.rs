use super::{
    firebase as firebase_store,
    postgres::{market_store, user_store},
    redis as redis_store,
};
use crate::domain::{
    models::{
        access_token::{AccessToken, AccessTokenId},
        market::{Market, MarketId, MarketStatus, Order, OrderId},
        user::{User, UserId},
    },
    services::{
        market_store::NewMarket, user_store::NewUser, AccessTokenStore, MarketStore, Store,
        StoreFactory, UserStore,
    },
};
use diesel::{connection::TransactionManager, pg::PgConnection as PgConn, Connection};
use redis::{Client as RedisClient, Connection as RedisConn};
use std::sync::Arc;

pub struct DbStoreFactory {
    pg_conn_url: Arc<String>,
    redis_client: RedisClient,
    firebase_api_key: Arc<String>,
}

pub struct DbStore {
    pg_conn_url: Arc<String>,
    redis_client: RedisClient,
    firebase_api_key: Arc<String>,
    pg_conn: Option<PgConn>,
    redis_conn: Option<RedisConn>,
}

impl DbStoreFactory {
    pub fn new(pg_url: String, redis_url: String, firebase_api_key: String) -> DbStoreFactory {
        DbStoreFactory {
            pg_conn_url: Arc::new(pg_url),
            redis_client: RedisClient::open(redis_url.as_str()).unwrap(),
            firebase_api_key: Arc::new(firebase_api_key),
        }
    }
}

impl StoreFactory<DbStore> for DbStoreFactory {
    fn transaction<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut DbStore) -> Result<T, E>,
        E: From<<DbStore as Store>::Error>,
    {
        let mut store = DbStore {
            pg_conn_url: self.pg_conn_url.clone(),
            redis_client: self.redis_client.clone(),
            firebase_api_key: self.firebase_api_key.clone(),
            pg_conn: None,
            redis_conn: None,
        };
        match f(&mut store) {
            Ok(t) => {
                store.commit()?;
                Ok(t)
            }
            Err(e) => {
                store.rollback()?;
                Err(e)
            }
        }
    }
}

impl DbStore {
    fn pg_conn(&mut self) -> Result<&PgConn, failure::Error> {
        if self.pg_conn.is_none() {
            let pg_conn = PgConn::establish(self.pg_conn_url.as_str())?;
            pg_conn.transaction_manager().begin_transaction(&pg_conn)?;
            self.pg_conn = Some(pg_conn);
        }
        Ok(self.pg_conn.as_ref().unwrap())
    }

    fn redis_conn(&mut self) -> Result<&RedisConn, failure::Error> {
        if self.redis_conn.is_none() {
            self.redis_conn = Some(self.redis_client.get_connection()?);
        }
        Ok(self.redis_conn.as_ref().unwrap())
    }

    fn commit(mut self) -> Result<(), <Self as Store>::Error> {
        match self.pg_conn.take() {
            Some(conn) => Ok(conn.transaction_manager().commit_transaction(&conn)?),
            None => Ok(()),
        }
    }

    fn rollback(mut self) -> Result<(), <Self as Store>::Error> {
        match self.pg_conn.take() {
            Some(conn) => Ok(conn.transaction_manager().rollback_transaction(&conn)?),
            None => Ok(()),
        }
    }
}

impl Store for DbStore {
    type Error = failure::Error;
}

impl MarketStore for DbStore {
    fn lock_market_inner(&mut self, market_id: &MarketId) -> Result<(), Self::Error> {
        market_store::lock_market(self.pg_conn()?, market_id)
    }

    fn insert_market(&mut self, market: NewMarket) -> Result<MarketId, Self::Error> {
        Ok(market_store::insert_market(self.pg_conn()?, market)?)
    }

    /// 単純な実装。
    /// リクエストのたびに、全ての情報をDBから取ってくる
    /// 将来的にはOpenMarketのみ構造体内部にキャッシュするような実装にしたい
    fn query_market(&mut self, market_id: &MarketId) -> Result<Option<Market>, Self::Error> {
        Ok(market_store::query_market(self.pg_conn()?, market_id)?)
    }

    fn query_market_ids_related_to_user(
        &mut self,
        user_id: &UserId,
    ) -> Result<Vec<MarketId>, Self::Error> {
        Ok(market_store::query_market_ids_related_to_user(
            self.pg_conn()?,
            user_id,
        )?)
    }

    fn query_market_ids_ready_to_open(&mut self) -> Result<Vec<MarketId>, Self::Error> {
        Ok(market_store::query_market_ids_ready_to_open(
            self.pg_conn()?,
        )?)
    }

    fn query_market_ids_ready_to_close(&mut self) -> Result<Vec<MarketId>, Self::Error> {
        Ok(market_store::query_market_ids_ready_to_close(
            self.pg_conn()?,
        )?)
    }

    fn update_market_status(
        &mut self,
        market_id: &MarketId,
        status: &MarketStatus,
    ) -> Result<(), Self::Error> {
        market_store::update_market_status(self.pg_conn()?, market_id, status)
    }

    fn insert_market_orders<'a, I>(
        &mut self,
        market_id: &MarketId,
        orders: I,
    ) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (OrderId, &'a Order)>,
    {
        market_store::insert_market_orders(self.pg_conn()?, market_id, orders)
    }
}

impl UserStore for DbStore {
    fn query_user(&mut self, user_id: &UserId) -> Result<Option<User>, <Self as Store>::Error> {
        Ok(user_store::query_user(self.pg_conn()?, user_id)?)
    }

    fn query_all_user_ids(&mut self) -> Result<Vec<UserId>, <Self as Store>::Error> {
        Ok(user_store::query_all_user_ids(self.pg_conn()?)?)
    }

    fn save_user(&mut self, new_user: NewUser) -> Result<(), Self::Error> {
        Ok(user_store::save_user(self.pg_conn()?, new_user)?)
    }
}

impl AccessTokenStore for DbStore {
    fn validate_access_token(
        &mut self,
        access_token_id: &AccessTokenId,
    ) -> Result<Option<AccessToken>, Self::Error> {
        match redis_store::query_access_token(self.redis_conn()?, access_token_id)? {
            Some(token) => return Ok(Some(token)),
            None => {}
        }

        // Token を Cache する時間。10分
        const CACHE_EXPIRE_SEC: usize = 60 * 10;

        match firebase_store::get_user(self.firebase_api_key.as_str(), access_token_id)? {
            Some(token) => {
                redis_store::save_access_token(self.redis_conn()?, &token, CACHE_EXPIRE_SEC)?;
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }
}
