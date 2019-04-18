use failure::Error;
use redis::Connection as RedisConnection;

#[derive(Debug, Clone)]
pub struct ConnectionFactory(redis::Client);

impl ConnectionFactory {
    pub fn new_with_env() -> ConnectionFactory {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not presented");
        ConnectionFactory::new(redis_url)
    }

    pub fn new(url: String) -> ConnectionFactory {
        ConnectionFactory(redis::Client::open(url.as_str()).unwrap())
    }

    pub fn establish_connection(&self) -> Result<RedisConnection, Error> {
        Ok(self.0.get_connection()?)
    }
}
