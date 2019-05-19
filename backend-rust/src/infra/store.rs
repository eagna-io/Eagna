use crate::{
    domain::{
        models::{
            access_token::{AccessToken, AccessTokenId, TOKEN_EXPIRE_SEC},
            market::{ClosedMarket, Market, MarketId, OpenMarket},
            user::{User, UserId},
        },
        services::{
            market_store::{UpdateMarketLastOrderErrorKind, UpdateMarketStatusErrorKind},
            AccessTokenStore, MarketStore, Store, StoreFactory, UserStore,
        },
    },
    infra::postgres::{market_store, user_store},
};
use diesel::{connection::TransactionManager, pg::PgConnection as PgConn, Connection};
use redis::{Commands, Connection as RedisConn};
use std::sync::Arc;

pub struct DbStoreFactory {
    pg_conn_url: Arc<String>,
    redis_client: redis::Client,
}

pub struct DbStore {
    pg_conn_url: Arc<String>,
    redis_client: redis::Client,
    pg_conn: Option<PgConn>,
    redis_conn: Option<RedisConn>,
}

impl DbStoreFactory {
    pub fn new_with_env() -> DbStoreFactory {
        let pg_conn_url = std::env::var("PG_URL").expect("PG_URL is not defined");
        let redis_conn_url = std::env::var("REDIS_URL").expect("REDIS_URL is not defined");
        let redis_client = redis::Client::open(redis_conn_url.as_str()).unwrap();
        DbStoreFactory {
            pg_conn_url: Arc::new(pg_conn_url),
            redis_client,
        }
    }
}

impl StoreFactory<DbStore> for DbStoreFactory {
    fn establish(&self) -> DbStore {
        DbStore {
            pg_conn_url: self.pg_conn_url.clone(),
            redis_client: self.redis_client.clone(),
            pg_conn: None,
            redis_conn: None,
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
}

impl Store for DbStore {
    type Error = failure::Error;

    fn commit(mut self) -> Result<(), Self::Error> {
        match self.pg_conn.take() {
            Some(conn) => Ok(conn.transaction_manager().commit_transaction(&conn)?),
            None => Ok(()),
        }
    }
}

impl std::ops::Drop for DbStore {
    fn drop(&mut self) {
        match self.pg_conn.take() {
            Some(conn) => match conn.transaction_manager().rollback_transaction(&conn) {
                Ok(()) => {}
                Err(e) => {
                    println!("Error while pg transaction rollback : {:?}", e);
                }
            },
            None => {}
        }
    }
}

impl MarketStore for DbStore {
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

    fn update_market_last_order(
        &mut self,
        market: &OpenMarket,
    ) -> Result<(), UpdateMarketLastOrderErrorKind<Self::Error>> {
        match self.pg_conn() {
            Ok(conn) => market_store::update_market_last_order(conn, market),
            Err(e) => Err(UpdateMarketLastOrderErrorKind::Error(e)),
        }
    }

    fn update_market_status_to_open(
        &mut self,
        market: &OpenMarket,
    ) -> Result<(), UpdateMarketStatusErrorKind<Self::Error>> {
        match self.pg_conn() {
            Ok(conn) => market_store::update_market_status_to_open(conn, market),
            Err(e) => Err(UpdateMarketStatusErrorKind::Error(e)),
        }
    }

    fn update_market_status_to_closed(
        &mut self,
        market: &ClosedMarket,
    ) -> Result<(), UpdateMarketStatusErrorKind<Self::Error>> {
        match self.pg_conn() {
            Ok(conn) => market_store::update_market_status_to_closed(conn, market),
            Err(e) => Err(UpdateMarketStatusErrorKind::Error(e)),
        }
    }
}

impl UserStore for DbStore {
    fn query_user(&mut self, user_id: &UserId) -> Result<Option<User>, <Self as Store>::Error> {
        Ok(user_store::query_user(self.pg_conn()?, user_id)?)
    }

    fn query_user_by_email_and_hashed_pass(
        &mut self,
        email: &str,
        hashed_pass: &str,
    ) -> Result<Option<User>, <Self as Store>::Error> {
        Ok(user_store::query_user_by_email_and_hashed_pass(
            self.pg_conn()?,
            email,
            hashed_pass,
        )?)
    }

    fn query_all_user_ids(&mut self) -> Result<Vec<UserId>, <Self as Store>::Error> {
        Ok(user_store::query_all_user_ids(self.pg_conn()?)?)
    }
}

impl AccessTokenStore for DbStore {
    fn save_access_token(&mut self, token: &AccessToken) -> Result<(), <Self as Store>::Error> {
        Ok(self
            .redis_conn()?
            .set_ex(token.id.as_str(), token.user_id.0, TOKEN_EXPIRE_SEC)?)
    }

    fn query_access_token(
        &mut self,
        access_token_id: &AccessTokenId,
    ) -> Result<Option<AccessToken>, <Self as Store>::Error> {
        match self.redis_conn()?.get(access_token_id.as_str())? {
            Some(user_id) => Ok(Some(AccessToken {
                id: *access_token_id,
                user_id: UserId(user_id),
            })),
            None => Ok(None),
        }
    }
}
