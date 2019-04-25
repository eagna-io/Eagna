use redis::{Connection as RedisConnection, RedisError};

#[derive(Debug, Clone)]
pub struct ConnectionFactory(redis::Client);

impl ConnectionFactory {
    pub fn new_with_env() -> ConnectionFactory {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL is not presented");
        ConnectionFactory::new(redis_url)
    }

    pub fn new(url: String) -> ConnectionFactory {
        ConnectionFactory(redis::Client::open(url.as_str()).unwrap())
    }

    pub fn establish(&self) -> Result<RedisConnection, RedisError> {
        Ok(self.0.get_connection()?)
    }
}
