use redis::{Client as RedisClient, Commands, Connection as RedisConn};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::InfraFactory;

pub trait RedisInfra: Send + 'static {
    fn save_access_token(
        &self,
        access_token_id: &str,
        user_id: &Uuid,
        expire_sec: usize,
    ) -> Result<(), failure::Error>;

    fn query_user_id_by_access_token(
        &self,
        access_token_id: &str,
    ) -> Result<Option<Uuid>, failure::Error>;

    fn delete_access_token(&self, access_token_id: &str) -> Result<(), failure::Error>;
}

#[derive(Clone)]
pub struct Redis {
    conn: Arc<Mutex<RedisConn>>,
}

impl RedisInfra for Redis {
    fn save_access_token(
        &self,
        access_token_id: &str,
        user_id: &Uuid,
        expire_sec: usize,
    ) -> Result<(), failure::Error> {
        let mut user_id_buf = Uuid::encode_buffer();
        let user_id_str = user_id.to_simple_ref().encode_lower(&mut user_id_buf) as &_;
        Ok(self
            .conn
            .lock()
            .unwrap()
            .set_ex(access_token_id, user_id_str, expire_sec)?)
    }

    fn query_user_id_by_access_token(
        &self,
        access_token_id: &str,
    ) -> Result<Option<Uuid>, failure::Error> {
        Ok(self
            .conn
            .lock()
            .unwrap()
            .get::<_, Option<String>>(access_token_id)?
            .map(|user_id| Uuid::parse_str(user_id.as_str()).unwrap()))
    }

    fn delete_access_token(&self, access_token_id: &str) -> Result<(), failure::Error> {
        self.conn.lock().unwrap().del(access_token_id)?;
        Ok(())
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
            conn: Arc::new(Mutex::new(client.get_connection()?)),
        })
    }
}
