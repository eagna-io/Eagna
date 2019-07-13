use redis::{Commands, Connection as RedisConn, Client as RedisClient};
use std::sync::Arc;

use super::InfraFactory;

pub trait RedisInfra {
    fn save_access_token(
        &self,
        access_token_id: &str,
        user_id: &str,
        expire_sec: usize,
    ) -> Result<(), failure::Error>;

    fn query_user_id_by_access_token(
        &self,
        access_token_id: &str,
    ) -> Result<Option<String>, failure::Error>;
}

pub struct Redis {
    conn: RedisConn,
}

impl RedisInfra for Redis {
    fn save_access_token(
        &self,
        access_token_id: &str,
        user_id: &str,
        expire_sec: usize,
    ) -> Result<(), failure::Error> {
        Ok(self.conn.set_ex(access_token_id, user_id, expire_sec)?)
    }

    fn query_user_id_by_access_token(
        &self,
        access_token_id: &str,
    ) -> Result<Option<String>, failure::Error> {
        Ok(self.conn.get::<_, Option<String>>(access_token_id)?)
    }
}

#[derive(Debug, Clone)]
pub struct RedisFactory {
    url: Arc<String>,
}

impl RedisFactory {
    pub fn new(url: String) -> RedisFactory {
        RedisFactory { url: Arc::new(url) }
    }
}

impl InfraFactory<Redis> for RedisFactory {
    fn create(&self) -> Result<Redis, failure::Error> {
        let client = RedisClient::open(self.url.as_str())?;
        Ok(Redis {
            conn: client.get_connection()?,
        })
    }
}
